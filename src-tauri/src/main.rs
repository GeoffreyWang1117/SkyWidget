// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitors;
mod network;
mod alerts;
mod storage;

use alerts::engine::AlertEngine;
use alerts::notifier::AlertNotifier;
use alerts::rules::{default_rules, AlertCondition, AlertRule, AlertSeverity};
use log::info;
use monitors::{CpuMonitor, DiskMonitor, MemoryMonitor, TemperatureMonitor};
use network::api::{start_api_server, ApiState};
use network::discovery::DiscoveryService;
use network::node::{Node, NodeInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use storage::alerts_store::{AlertRecord, AlertsStore};
use storage::metrics::MetricsStore;
use sysinfo::System;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, State,
};
use tokio::sync::RwLock;

// 全局状态管理
pub struct AppState {
    cpu_monitor: Arc<RwLock<CpuMonitor>>,
    memory_monitor: Arc<RwLock<MemoryMonitor>>,
    disk_monitor: Arc<RwLock<DiskMonitor>>,
    temperature_monitor: Arc<RwLock<TemperatureMonitor>>,
    node_info: Arc<RwLock<NodeInfo>>,
    discovered_nodes: Arc<RwLock<Vec<NodeInfo>>>,
    alert_engine: Arc<RwLock<Option<Arc<AlertEngine>>>>,
    alerts_store: Arc<RwLock<AlertsStore>>,
    metrics_store: Arc<RwLock<MetricsStore>>,
}

// 简单的问候命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}! 欢迎使用 SkyWidget 硬件监控工具。", name)
}

// 获取系统信息
#[tauri::command]
fn get_system_info() -> serde_json::Value {
    let mut sys = System::new_all();
    sys.refresh_all();

    serde_json::json!({
        "system_name": System::name(),
        "kernel_version": System::kernel_version(),
        "os_version": System::os_version(),
        "host_name": System::host_name(),
        "cpu_count": sys.cpus().len(),
        "total_memory": sys.total_memory(),
        "used_memory": sys.used_memory(),
    })
}

// 获取 CPU 信息
#[tauri::command]
async fn get_cpu_info(state: State<'_, AppState>) -> Result<monitors::cpu::CpuInfo, String> {
    let mut monitor = state.cpu_monitor.write().await;
    Ok(monitor.get_info())
}

// 获取内存信息
#[tauri::command]
async fn get_memory_info(state: State<'_, AppState>) -> Result<monitors::memory::MemoryInfo, String> {
    let mut monitor = state.memory_monitor.write().await;
    Ok(monitor.get_info())
}

// 获取磁盘信息
#[tauri::command]
async fn get_disk_info(state: State<'_, AppState>) -> Result<monitors::disk::DisksInfo, String> {
    let mut monitor = state.disk_monitor.write().await;
    Ok(monitor.get_info())
}

// 获取所有硬件信息（一次性获取全部数据）
#[tauri::command]
async fn get_all_hardware_info(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let cpu_info = get_cpu_info(state.clone()).await?;
    let memory_info = get_memory_info(state.clone()).await?;
    let disk_info = get_disk_info(state.clone()).await?;

    Ok(serde_json::json!({
        "cpu": cpu_info,
        "memory": memory_info,
        "disk": disk_info,
        "timestamp": chrono::Utc::now().timestamp_millis(),
    }))
}

// 获取本地节点信息
#[tauri::command]
async fn get_local_node_info(state: State<'_, AppState>) -> Result<NodeInfo, String> {
    let node = state.node_info.read().await;
    Ok(node.clone())
}

// 获取已发现的节点列表
#[tauri::command]
async fn get_discovered_nodes(state: State<'_, AppState>) -> Result<Vec<NodeInfo>, String> {
    let nodes = state.discovered_nodes.read().await;
    Ok(nodes.clone())
}

// 获取告警规则
#[tauri::command]
async fn get_alert_rules(state: State<'_, AppState>) -> Result<Vec<AlertRule>, String> {
    let engine_opt = state.alert_engine.read().await;
    if let Some(engine) = engine_opt.as_ref() {
        Ok(engine.get_rules().await)
    } else {
        Ok(vec![])
    }
}

// 切换告警规则启用状态
#[tauri::command]
async fn toggle_alert_rule(
    state: State<'_, AppState>,
    rule_id: String,
    enabled: bool,
) -> Result<(), String> {
    let engine_opt = state.alert_engine.read().await;
    if let Some(engine) = engine_opt.as_ref() {
        engine.toggle_rule(&rule_id, enabled).await;
        Ok(())
    } else {
        Err("Alert engine not initialized".to_string())
    }
}

// 获取告警历史记录
#[tauri::command]
async fn get_alert_history(state: State<'_, AppState>) -> Result<Vec<AlertRecord>, String> {
    let store = state.alerts_store.read().await;
    Ok(store.get_all_records())
}

// 确认告警
#[tauri::command]
async fn acknowledge_alert(state: State<'_, AppState>, record_id: String) -> Result<(), String> {
    let mut store = state.alerts_store.write().await;
    if store.acknowledge(&record_id) {
        Ok(())
    } else {
        Err("Alert record not found".to_string())
    }
}

// 清空告警历史
#[tauri::command]
async fn clear_alert_history(state: State<'_, AppState>) -> Result<(), String> {
    let mut store = state.alerts_store.write().await;
    store.clear();
    Ok(())
}

// 添加新的告警规则
#[tauri::command]
async fn add_alert_rule(
    state: State<'_, AppState>,
    name: String,
    description: String,
    condition_type: String,
    threshold: f32,
    severity: String,
) -> Result<(), String> {
    let condition = match condition_type.as_str() {
        "cpu_usage" => AlertCondition::CpuUsageAbove(threshold),
        "memory_usage" => AlertCondition::MemoryUsageAbove(threshold),
        "disk_usage" => AlertCondition::DiskUsageAbove(threshold),
        "cpu_temperature" => AlertCondition::CpuTemperatureAbove(threshold),
        _ => return Err("Invalid condition type".to_string()),
    };

    let severity = match severity.as_str() {
        "Info" => AlertSeverity::Info,
        "Warning" => AlertSeverity::Warning,
        "Error" => AlertSeverity::Error,
        "Critical" => AlertSeverity::Critical,
        _ => return Err("Invalid severity".to_string()),
    };

    let rule = AlertRule::new(
        uuid::Uuid::new_v4().to_string(),
        name,
        description,
        condition,
        severity,
    );

    let engine_opt = state.alert_engine.read().await;
    if let Some(engine) = engine_opt.as_ref() {
        engine.add_rule(rule).await;
        Ok(())
    } else {
        Err("Alert engine not initialized".to_string())
    }
}

// 删除告警规则
#[tauri::command]
async fn remove_alert_rule(state: State<'_, AppState>, rule_id: String) -> Result<(), String> {
    let engine_opt = state.alert_engine.read().await;
    if let Some(engine) = engine_opt.as_ref() {
        engine.remove_rule(&rule_id).await;
        Ok(())
    } else {
        Err("Alert engine not initialized".to_string())
    }
}

// 导出告警历史为 JSON
#[tauri::command]
async fn export_alert_history(state: State<'_, AppState>) -> Result<String, String> {
    let store = state.alerts_store.read().await;
    store.export_json()
}

// 导出硬件指标数据
#[tauri::command]
async fn export_metrics(state: State<'_, AppState>) -> Result<String, String> {
    let store = state.metrics_store.read().await;
    store.export_json()
}

// 获取温度信息
#[tauri::command]
async fn get_temperature_info(
    state: State<'_, AppState>,
) -> Result<monitors::temperature::TemperatureInfo, String> {
    let mut monitor = state.temperature_monitor.write().await;
    Ok(monitor.get_info())
}

// 检查是否支持温度监控
#[tauri::command]
async fn is_temperature_supported(state: State<'_, AppState>) -> Result<bool, String> {
    let monitor = state.temperature_monitor.read().await;
    Ok(monitor.is_supported())
}

// 获取指标历史数据（用于图表）
#[tauri::command]
async fn get_metrics_history(
    state: State<'_, AppState>,
    metric_name: String,
    max_points: Option<usize>,
) -> Result<Vec<storage::metrics::MetricDataPoint>, String> {
    let store = state.metrics_store.read().await;

    if let Some(data) = store.get_metric(&metric_name) {
        let points = if let Some(max) = max_points {
            let start = if data.len() > max {
                data.len() - max
            } else {
                0
            };
            data[start..].to_vec()
        } else {
            data.clone()
        };
        Ok(points)
    } else {
        Ok(Vec::new())
    }
}

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::init();

    // 获取本地 IP 地址
    let local_ip = local_ip_address::local_ip()
        .unwrap_or_else(|_| IpAddr::from([127, 0, 0, 1]));

    info!("Local IP address: {}", local_ip);

    // API 端口
    let api_port = 3030;

    // 获取主机名
    let hostname = System::host_name().unwrap_or_else(|| "SkyWidget-Node".to_string());

    // 创建本地节点
    let local_node = Node::new(hostname.clone(), local_ip, api_port);
    let node_info = Arc::new(RwLock::new(local_node.info().clone()));

    // 初始化监控器
    let cpu_monitor = Arc::new(RwLock::new(CpuMonitor::new()));
    let memory_monitor = Arc::new(RwLock::new(MemoryMonitor::new()));
    let disk_monitor = Arc::new(RwLock::new(DiskMonitor::new()));
    let temperature_monitor = Arc::new(RwLock::new(TemperatureMonitor::new()));

    // 已发现的节点列表
    let discovered_nodes = Arc::new(RwLock::new(Vec::new()));

    // 创建 mDNS 服务发现
    let service_type = "_skywidget._tcp.local.";
    let discovery = DiscoveryService::new(service_type, local_node.info().id.clone())
        .expect("Failed to create discovery service");

    // 注册本地服务
    let mut properties = HashMap::new();
    properties.insert("id".to_string(), local_node.info().id.clone());
    properties.insert("name".to_string(), hostname.clone());
    properties.insert("os_info".to_string(), local_node.info().os_info.clone());
    properties.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());

    discovery
        .register_service(&hostname, api_port, properties)
        .expect("Failed to register mDNS service");

    // 开始浏览服务
    discovery
        .browse_services()
        .expect("Failed to start browsing services");

    // 定期更新已发现的节点
    let discovery_clone = Arc::new(discovery);
    let discovered_nodes_clone = discovered_nodes.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let nodes = discovery_clone.get_discovered_nodes();
            let mut nodes_guard = discovered_nodes_clone.write().await;
            *nodes_guard = nodes;

            // 清理离线节点（超过 30 秒没有心跳）
            discovery_clone.cleanup_offline_nodes(30);
        }
    });

    // 创建告警通知器
    let notifier = AlertNotifier::new(node_info.clone(), discovered_nodes.clone());

    // 创建数据存储
    let alerts_store = Arc::new(RwLock::new(AlertsStore::new(1000))); // 最多存储 1000 条记录
    let metrics_store = Arc::new(RwLock::new(MetricsStore::new(86400))); // 24小时数据

    // 创建告警引擎
    let alert_rules = default_rules();
    let mut engine = AlertEngine::new(
        alert_rules,
        notifier,
        cpu_monitor.clone(),
        memory_monitor.clone(),
        disk_monitor.clone(),
    );

    // 设置告警历史存储
    engine.set_alerts_store(alerts_store.clone());

    // 启动告警引擎（每 10 秒检查一次）
    let engine = Arc::new(engine);
    engine.start(10).await;

    let alert_engine = Arc::new(RwLock::new(Some(engine.clone())));

    // 定期收集指标数据并存储
    let metrics_store_clone = metrics_store.clone();
    let cpu_monitor_clone = cpu_monitor.clone();
    let memory_monitor_clone = memory_monitor.clone();
    let disk_monitor_clone = disk_monitor.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
        loop {
            interval.tick().await;

            let mut store = metrics_store_clone.write().await;

            // 收集 CPU 数据
            {
                let mut cpu = cpu_monitor_clone.write().await;
                let cpu_info = cpu.get_info();
                store.add_metric("cpu_usage", cpu_info.usage);
            }

            // 收集内存数据
            {
                let mut memory = memory_monitor_clone.write().await;
                let memory_info = memory.get_info();
                let usage_percent = if memory_info.total > 0 {
                    (memory_info.used as f32 / memory_info.total as f32) * 100.0
                } else {
                    0.0
                };
                store.add_metric("memory_usage_percent", usage_percent);
            }

            // 收集磁盘数据
            {
                let mut disk = disk_monitor_clone.write().await;
                let disk_info = disk.get_info();

                let mut total_space = 0u64;
                let mut total_used = 0u64;

                for disk in &disk_info.disks {
                    total_space += disk.total_space;
                    total_used += disk.total_space - disk.available_space;
                }

                let usage_percent = if total_space > 0 {
                    (total_used as f32 / total_space as f32) * 100.0
                } else {
                    0.0
                };

                store.add_metric("disk_usage_percent", usage_percent);
            }

            // 每小时清理一次旧数据
            store.cleanup_old_data(86400); // 保留 24 小时
        }
    });

    // 创建应用状态
    let app_state = AppState {
        cpu_monitor: cpu_monitor.clone(),
        memory_monitor: memory_monitor.clone(),
        disk_monitor: disk_monitor.clone(),
        temperature_monitor: temperature_monitor.clone(),
        node_info: node_info.clone(),
        discovered_nodes: discovered_nodes.clone(),
        alert_engine,
        alerts_store,
        metrics_store,
    };

    // 启动 HTTP API 服务器
    let api_state = Arc::new(ApiState::new(
        cpu_monitor,
        memory_monitor,
        disk_monitor,
        node_info,
    ));
    api_state.discovered_nodes.write().await;

    let api_state_clone = api_state.clone();
    let discovered_nodes_for_api = discovered_nodes.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let nodes = discovered_nodes_for_api.read().await;
            let mut api_nodes = api_state_clone.discovered_nodes.write().await;
            *api_nodes = nodes.clone();
        }
    });

    tokio::spawn(async move {
        start_api_server(api_state, api_port).await;
    });

    info!("Starting Tauri application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            get_system_info,
            get_cpu_info,
            get_memory_info,
            get_disk_info,
            get_all_hardware_info,
            get_temperature_info,
            is_temperature_supported,
            get_local_node_info,
            get_discovered_nodes,
            get_alert_rules,
            toggle_alert_rule,
            get_alert_history,
            acknowledge_alert,
            clear_alert_history,
            add_alert_rule,
            remove_alert_rule,
            export_alert_history,
            export_metrics,
            get_metrics_history,
        ])
        .setup(|app| {
            // 创建系统托盘菜单
            let show_i = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "隐藏窗口", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

            // 创建系统托盘图标
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("SkyWidget - 硬件监控")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // 双击托盘图标显示窗口
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
