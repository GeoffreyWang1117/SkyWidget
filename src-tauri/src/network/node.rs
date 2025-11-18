use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;

/// 节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// 节点唯一 ID
    pub id: String,

    /// 节点名称（主机名）
    pub name: String,

    /// 节点 IP 地址
    pub ip_address: IpAddr,

    /// HTTP API 端口
    pub api_port: u16,

    /// 最后心跳时间（Unix 时间戳）
    pub last_heartbeat: i64,

    /// 节点状态
    pub status: NodeStatus,

    /// 操作系统信息
    pub os_info: String,

    /// 节点版本
    pub version: String,
}

/// 节点状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    /// 在线
    Online,

    /// 离线
    Offline,

    /// 告警中
    Alerting,
}

/// 节点实例
pub struct Node {
    info: NodeInfo,
}

impl Node {
    /// 创建新节点
    pub fn new(name: String, ip_address: IpAddr, api_port: u16) -> Self {
        let os_info = format!(
            "{} {}",
            sysinfo::System::name().unwrap_or_else(|| "Unknown".to_string()),
            sysinfo::System::os_version().unwrap_or_else(|| "Unknown".to_string())
        );

        Self {
            info: NodeInfo {
                id: Uuid::new_v4().to_string(),
                name,
                ip_address,
                api_port,
                last_heartbeat: chrono::Utc::now().timestamp(),
                status: NodeStatus::Online,
                os_info,
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }

    /// 获取节点信息
    pub fn info(&self) -> &NodeInfo {
        &self.info
    }

    /// 更新心跳时间
    pub fn update_heartbeat(&mut self) {
        self.info.last_heartbeat = chrono::Utc::now().timestamp();
    }

    /// 更新节点状态
    pub fn update_status(&mut self, status: NodeStatus) {
        self.info.status = status;
    }

    /// 获取节点 API 地址
    pub fn api_url(&self) -> String {
        format!("http://{}:{}", self.info.ip_address, self.info.api_port)
    }

    /// 检查节点是否在线（心跳超时检测）
    pub fn is_online(&self, timeout_seconds: i64) -> bool {
        let now = chrono::Utc::now().timestamp();
        now - self.info.last_heartbeat < timeout_seconds
    }
}

impl NodeInfo {
    /// 获取节点 API 地址
    pub fn api_url(&self) -> String {
        format!("http://{}:{}", self.ip_address, self.api_port)
    }
}
