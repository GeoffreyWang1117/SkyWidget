use log::{error, info};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::rules::AlertSeverity;
use crate::network::api::AlertNotification;
use crate::network::node::NodeInfo;

/// 告警通知器
pub struct AlertNotifier {
    /// 本地节点信息
    local_node: Arc<RwLock<NodeInfo>>,

    /// 已发现的远程节点列表
    remote_nodes: Arc<RwLock<Vec<NodeInfo>>>,

    /// HTTP 客户端
    http_client: reqwest::Client,
}

impl AlertNotifier {
    /// 创建新的告警通知器
    pub fn new(
        local_node: Arc<RwLock<NodeInfo>>,
        remote_nodes: Arc<RwLock<Vec<NodeInfo>>>,
    ) -> Self {
        Self {
            local_node,
            remote_nodes,
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }

    /// 发送告警通知
    pub async fn send_alert(
        &self,
        alert_id: &str,
        alert_name: &str,
        message: &str,
        severity: &AlertSeverity,
        target_node_ids: &[String],
    ) {
        // 发送本地桌面通知
        self.send_local_notification(alert_name, message, severity);

        // 获取本地节点信息
        let local_node = self.local_node.read().await;

        // 创建告警通知消息
        let notification = AlertNotification {
            source_node_id: local_node.id.clone(),
            source_node_name: local_node.name.clone(),
            alert_type: alert_id.to_string(),
            message: message.to_string(),
            severity: format!("{:?}", severity),
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        // 获取目标节点列表
        let remote_nodes = self.remote_nodes.read().await;

        // 确定通知目标
        let targets: Vec<&NodeInfo> = if target_node_ids.is_empty() {
            // 通知所有节点
            remote_nodes.iter().collect()
        } else {
            // 只通知指定节点
            remote_nodes
                .iter()
                .filter(|node| target_node_ids.contains(&node.id))
                .collect()
        };

        // 发送网络通知到远程节点
        for node in targets {
            self.send_network_notification(node, &notification).await;
        }
    }

    /// 发送本地桌面通知
    fn send_local_notification(
        &self,
        title: &str,
        message: &str,
        severity: &AlertSeverity,
    ) {
        #[cfg(not(target_os = "linux"))]
        {
            use notify_rust::{Notification, Timeout};

            let icon = match severity {
                AlertSeverity::Info => "dialog-information",
                AlertSeverity::Warning => "dialog-warning",
                AlertSeverity::Error => "dialog-error",
                AlertSeverity::Critical => "dialog-error",
            };

            if let Err(e) = Notification::new()
                .summary(title)
                .body(message)
                .icon(icon)
                .timeout(Timeout::Milliseconds(10000))
                .show()
            {
                error!("Failed to show desktop notification: {}", e);
            } else {
                info!("Sent local notification: {}", title);
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Linux 上可能需要不同的通知方式，或者跳过
            info!("Local notification (Linux): {} - {}", title, message);
        }
    }

    /// 发送网络通知到远程节点
    async fn send_network_notification(
        &self,
        target_node: &NodeInfo,
        notification: &AlertNotification,
    ) {
        let url = format!("{}/alerts/notify", target_node.api_url());

        match self.http_client.post(&url).json(notification).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!(
                        "Successfully sent alert notification to {} ({})",
                        target_node.name, target_node.id
                    );
                } else {
                    error!(
                        "Failed to send alert notification to {}: HTTP {}",
                        target_node.name,
                        response.status()
                    );
                }
            }
            Err(e) => {
                error!(
                    "Failed to send alert notification to {}: {}",
                    target_node.name, e
                );
            }
        }
    }

    /// 更新远程节点列表
    pub async fn update_remote_nodes(&self, nodes: Vec<NodeInfo>) {
        let mut remote_nodes = self.remote_nodes.write().await;
        *remote_nodes = nodes;
    }
}
