use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{Filter, Rejection, Reply};

use crate::monitors::{CpuMonitor, DiskMonitor, MemoryMonitor};
use super::node::NodeInfo;

/// API 服务器状态
pub struct ApiState {
    pub cpu_monitor: Arc<RwLock<CpuMonitor>>,
    pub memory_monitor: Arc<RwLock<MemoryMonitor>>,
    pub disk_monitor: Arc<RwLock<DiskMonitor>>,
    pub node_info: Arc<RwLock<NodeInfo>>,
    pub discovered_nodes: Arc<RwLock<Vec<NodeInfo>>>,
}

/// 健康检查响应
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: i64,
}

/// 硬件信息响应
#[derive(Serialize)]
struct HardwareInfoResponse {
    cpu: serde_json::Value,
    memory: serde_json::Value,
    disk: serde_json::Value,
    timestamp: i64,
}

/// 告警通知请求
#[derive(Debug, Deserialize, Serialize)]
pub struct AlertNotification {
    pub source_node_id: String,
    pub source_node_name: String,
    pub alert_type: String,
    pub message: String,
    pub severity: String,
    pub timestamp: i64,
}

impl ApiState {
    pub fn new(
        cpu_monitor: Arc<RwLock<CpuMonitor>>,
        memory_monitor: Arc<RwLock<MemoryMonitor>>,
        disk_monitor: Arc<RwLock<DiskMonitor>>,
        node_info: Arc<RwLock<NodeInfo>>,
    ) -> Self {
        Self {
            cpu_monitor,
            memory_monitor,
            disk_monitor,
            node_info,
            discovered_nodes: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

/// 启动 HTTP API 服务器
pub async fn start_api_server(state: Arc<ApiState>, port: u16) {
    info!("Starting API server on port {}", port);

    // 健康检查端点
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            let response = HealthResponse {
                status: "ok".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                timestamp: chrono::Utc::now().timestamp_millis(),
            };
            warp::reply::json(&response)
        });

    // 节点信息端点
    let state_for_node = state.clone();
    let node_info = warp::path("node")
        .and(warp::get())
        .and_then(move || {
            let state = state_for_node.clone();
            async move {
                let node = state.node_info.read().await;
                Ok::<_, Rejection>(warp::reply::json(&*node))
            }
        });

    // 硬件信息端点
    let state_for_hardware = state.clone();
    let hardware = warp::path("hardware")
        .and(warp::get())
        .and_then(move || {
            let state = state_for_hardware.clone();
            async move {
                let mut cpu = state.cpu_monitor.write().await;
                let mut memory = state.memory_monitor.write().await;
                let mut disk = state.disk_monitor.write().await;

                let cpu_info = cpu.get_info();
                let memory_info = memory.get_info();
                let disk_info = disk.get_info();

                let response = HardwareInfoResponse {
                    cpu: serde_json::to_value(&cpu_info).unwrap(),
                    memory: serde_json::to_value(&memory_info).unwrap(),
                    disk: serde_json::to_value(&disk_info).unwrap(),
                    timestamp: chrono::Utc::now().timestamp_millis(),
                };

                Ok::<_, Rejection>(warp::reply::json(&response))
            }
        });

    // 发现的节点列表端点
    let state_for_nodes = state.clone();
    let nodes = warp::path("nodes")
        .and(warp::get())
        .and_then(move || {
            let state = state_for_nodes.clone();
            async move {
                let nodes = state.discovered_nodes.read().await;
                Ok::<_, Rejection>(warp::reply::json(&*nodes))
            }
        });

    // 接收告警通知端点
    let alerts_notify = warp::path("alerts")
        .and(warp::path("notify"))
        .and(warp::post())
        .and(warp::body::json())
        .map(|alert: AlertNotification| {
            info!("Received alert from {}: {}", alert.source_node_name, alert.message);

            // 这里可以触发本地通知
            #[cfg(not(target_os = "linux"))]
            {
                use notify_rust::Notification;
                let _ = Notification::new()
                    .summary(&format!("Alert from {}", alert.source_node_name))
                    .body(&alert.message)
                    .show();
            }

            warp::reply::json(&serde_json::json!({
                "status": "ok",
                "message": "Alert received"
            }))
        });

    // CORS 配置
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    // 组合所有路由
    let routes = health
        .or(node_info)
        .or(hardware)
        .or(nodes)
        .or(alerts_notify)
        .with(cors)
        .with(warp::log("api"));

    // 启动服务器
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
