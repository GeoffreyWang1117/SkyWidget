/// 告警系统模块
///
/// 负责：
/// - 告警规则定义
/// - 告警检测引擎
/// - 告警通知分发
/// - 告警历史记录

pub mod rules;
pub mod engine;
pub mod notifier;

pub use rules::{AlertRule, AlertCondition, AlertSeverity};
pub use engine::AlertEngine;
pub use notifier::AlertNotifier;
