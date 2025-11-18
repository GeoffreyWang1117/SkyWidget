/// 网络通信模块
///
/// 负责：
/// - 节点发现 (mDNS)
/// - HTTP REST API 服务器
/// - 节点间通信
/// - WebSocket 实时数据推送

pub mod node;
pub mod discovery;
pub mod api;

pub use node::{Node, NodeInfo};
pub use discovery::DiscoveryService;
