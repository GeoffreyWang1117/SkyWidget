use log::{error, info};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::node::NodeInfo;

/// mDNS 服务发现
pub struct DiscoveryService {
    /// mDNS 守护进程
    daemon: ServiceDaemon,

    /// 服务类型
    service_type: String,

    /// 已发现的节点列表
    discovered_nodes: Arc<Mutex<HashMap<String, NodeInfo>>>,

    /// 本地节点信息
    local_node_id: String,
}

impl DiscoveryService {
    /// 创建新的发现服务
    pub fn new(service_type: &str, local_node_id: String) -> Result<Self, String> {
        let daemon = ServiceDaemon::new()
            .map_err(|e| format!("Failed to create mDNS daemon: {}", e))?;

        Ok(Self {
            daemon,
            service_type: service_type.to_string(),
            discovered_nodes: Arc::new(Mutex::new(HashMap::new())),
            local_node_id,
        })
    }

    /// 注册本地服务
    pub fn register_service(
        &self,
        instance_name: &str,
        port: u16,
        properties: HashMap<String, String>,
    ) -> Result<(), String> {
        let service_info = ServiceInfo::new(
            &self.service_type,
            instance_name,
            instance_name,
            "",
            port,
            properties,
        )
        .map_err(|e| format!("Failed to create service info: {}", e))?;

        self.daemon
            .register(service_info)
            .map_err(|e| format!("Failed to register service: {}", e))?;

        info!("Registered mDNS service: {}", instance_name);
        Ok(())
    }

    /// 开始浏览服务
    pub fn browse_services(&self) -> Result<(), String> {
        let receiver = self
            .daemon
            .browse(&self.service_type)
            .map_err(|e| format!("Failed to browse services: {}", e))?;

        let discovered_nodes = self.discovered_nodes.clone();
        let local_node_id = self.local_node_id.clone();

        // 在后台线程中处理发现的服务
        std::thread::spawn(move || {
            loop {
                match receiver.recv_timeout(Duration::from_secs(1)) {
                    Ok(event) => {
                        Self::handle_service_event(event, &discovered_nodes, &local_node_id);
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // 超时，继续循环
                        continue;
                    }
                    Err(e) => {
                        error!("Error receiving mDNS event: {}", e);
                        break;
                    }
                }
            }
        });

        info!("Started browsing for services: {}", self.service_type);
        Ok(())
    }

    /// 处理服务发现事件
    fn handle_service_event(
        event: ServiceEvent,
        discovered_nodes: &Arc<Mutex<HashMap<String, NodeInfo>>>,
        local_node_id: &str,
    ) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                info!("Service resolved: {}", info.get_fullname());

                // 从服务信息中提取节点数据
                if let Some(node_info) = Self::extract_node_info(&info) {
                    // 跳过本地节点
                    if node_info.id == local_node_id {
                        return;
                    }

                    let mut nodes = discovered_nodes.lock().unwrap();
                    nodes.insert(node_info.id.clone(), node_info);
                }
            }
            ServiceEvent::ServiceRemoved(_, fullname) => {
                info!("Service removed: {}", fullname);

                // 从已发现节点中移除
                let mut nodes = discovered_nodes.lock().unwrap();
                nodes.retain(|_, node| {
                    let service_name = format!("{}._skywidget._tcp.local.", node.name);
                    service_name != fullname
                });
            }
            _ => {}
        }
    }

    /// 从服务信息中提取节点信息
    fn extract_node_info(service_info: &ServiceInfo) -> Option<NodeInfo> {
        let properties = service_info.get_properties();

        let id = properties.get("id")?.to_string();
        let name = properties.get("name")?.to_string();
        let os_info = properties.get("os_info")?.to_string();
        let version = properties.get("version")?.to_string();

        // 获取 IP 地址（优先使用 IPv4）
        let ip_address = service_info
            .get_addresses()
            .iter()
            .find(|ip| matches!(ip, IpAddr::V4(_)))
            .or_else(|| service_info.get_addresses().iter().next())?
            .clone();

        let api_port = service_info.get_port();

        Some(NodeInfo {
            id,
            name,
            ip_address,
            api_port,
            last_heartbeat: chrono::Utc::now().timestamp(),
            status: super::node::NodeStatus::Online,
            os_info,
            version,
        })
    }

    /// 获取已发现的节点列表
    pub fn get_discovered_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.discovered_nodes.lock().unwrap();
        nodes.values().cloned().collect()
    }

    /// 清理离线节点（超过指定时间没有心跳）
    pub fn cleanup_offline_nodes(&self, timeout_seconds: i64) {
        let mut nodes = self.discovered_nodes.lock().unwrap();
        let now = chrono::Utc::now().timestamp();

        nodes.retain(|_, node| {
            let is_alive = now - node.last_heartbeat < timeout_seconds;
            if !is_alive {
                info!("Removing offline node: {} ({})", node.name, node.id);
            }
            is_alive
        });
    }
}

impl Drop for DiscoveryService {
    fn drop(&mut self) {
        if let Err(e) = self.daemon.shutdown() {
            error!("Error shutting down mDNS daemon: {}", e);
        }
    }
}
