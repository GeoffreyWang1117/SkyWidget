// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitors;
mod network;
mod alerts;
mod storage;

use alerts::engine::AlertEngine;
use alerts::notifier::AlertNotifier;
use alerts::rules::{default_rules, AlertRule};
use log::info;
use monitors::{CpuMonitor, DiskMonitor, MemoryMonitor};
use network::api::{start_api_server, ApiState};
use network::discovery::DiscoveryService;
use network::node::{Node, NodeInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use sysinfo::System;
use tauri::{Manager, State};
use tokio::sync::RwLock;

// 全局状态管理
pub struct AppState {
    cpu_monitor: Arc<RwLock<CpuMonitor>>,
    memory_monitor: Arc<RwLock<MemoryMonitor>>,
    disk_monitor: Arc<RwLock<DiskMonitor>>,
    node_info: Arc<RwLock<NodeInfo>>,
    discovered_nodes: Arc<RwLock<Vec<NodeInfo>>>,
    alert_engine: Arc<RwLock<Option<Arc<AlertEngine>>>>,
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

    // 创建告警引擎
    let alert_rules = default_rules();
    let engine = Arc::new(AlertEngine::new(
        alert_rules,
        notifier,
        cpu_monitor.clone(),
        memory_monitor.clone(),
        disk_monitor.clone(),
    ));

    // 启动告警引擎（每 10 秒检查一次）
    engine.start(10).await;

    let alert_engine = Arc::new(RwLock::new(Some(engine)));

    // 创建应用状态
    let app_state = AppState {
        cpu_monitor: cpu_monitor.clone(),
        memory_monitor: memory_monitor.clone(),
        disk_monitor: disk_monitor.clone(),
        node_info: node_info.clone(),
        discovered_nodes: discovered_nodes.clone(),
        alert_engine,
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
            get_local_node_info,
            get_discovered_nodes,
            get_alert_rules,
            toggle_alert_rule,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
