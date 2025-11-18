/// 数据存储模块
///
/// 负责：
/// - 时序指标数据存储
/// - 告警历史存储
/// - 节点配置持久化

pub mod metrics;
pub mod alerts_store;

pub use metrics::MetricsStore;
pub use alerts_store::AlertsStore;
