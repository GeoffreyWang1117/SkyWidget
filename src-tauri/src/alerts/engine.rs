use log::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

use super::notifier::AlertNotifier;
use super::rules::{AlertRule, AlertSeverity};
use crate::monitors::{CpuMonitor, DiskMonitor, MemoryMonitor};

/// 告警引擎
pub struct AlertEngine {
    /// 告警规则列表
    rules: Arc<RwLock<Vec<AlertRule>>>,

    /// 告警通知器
    notifier: Arc<AlertNotifier>,

    /// CPU 监控器
    cpu_monitor: Arc<RwLock<CpuMonitor>>,

    /// 内存监控器
    memory_monitor: Arc<RwLock<MemoryMonitor>>,

    /// 磁盘监控器
    disk_monitor: Arc<RwLock<DiskMonitor>>,
}

impl AlertEngine {
    /// 创建新的告警引擎
    pub fn new(
        rules: Vec<AlertRule>,
        notifier: AlertNotifier,
        cpu_monitor: Arc<RwLock<CpuMonitor>>,
        memory_monitor: Arc<RwLock<MemoryMonitor>>,
        disk_monitor: Arc<RwLock<DiskMonitor>>,
    ) -> Self {
        Self {
            rules: Arc::new(RwLock::new(rules)),
            notifier: Arc::new(notifier),
            cpu_monitor,
            memory_monitor,
            disk_monitor,
        }
    }

    /// 启动告警引擎（定期检查）
    pub async fn start(&self, check_interval_seconds: u64) {
        info!("Starting alert engine with {} second interval", check_interval_seconds);

        let rules = self.rules.clone();
        let notifier = self.notifier.clone();
        let cpu_monitor = self.cpu_monitor.clone();
        let memory_monitor = self.memory_monitor.clone();
        let disk_monitor = self.disk_monitor.clone();

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(check_interval_seconds));

            loop {
                ticker.tick().await;

                // 收集当前指标
                let metrics = Self::collect_metrics(
                    &cpu_monitor,
                    &memory_monitor,
                    &disk_monitor,
                ).await;

                // 检查所有规则
                let mut rules_guard = rules.write().await;
                for rule in rules_guard.iter_mut() {
                    if rule.should_trigger(&metrics) {
                        let message = rule.generate_message(&metrics);
                        info!("Alert triggered: {}", message);

                        // 发送通知
                        notifier.send_alert(
                            &rule.id,
                            &rule.name,
                            &message,
                            &rule.severity,
                            &rule.notify_nodes,
                        ).await;

                        // 标记已触发
                        rule.mark_triggered();
                    }
                }
            }
        });
    }

    /// 收集当前硬件指标
    async fn collect_metrics(
        cpu_monitor: &Arc<RwLock<CpuMonitor>>,
        memory_monitor: &Arc<RwLock<MemoryMonitor>>,
        disk_monitor: &Arc<RwLock<DiskMonitor>>,
    ) -> HashMap<String, f32> {
        let mut metrics = HashMap::new();

        // CPU 指标
        {
            let mut cpu = cpu_monitor.write().await;
            let cpu_info = cpu.get_info();
            metrics.insert("cpu_usage".to_string(), cpu_info.usage);
        }

        // 内存指标
        {
            let mut memory = memory_monitor.write().await;
            let memory_info = memory.get_info();
            let usage_percent = if memory_info.total > 0 {
                (memory_info.used as f32 / memory_info.total as f32) * 100.0
            } else {
                0.0
            };
            metrics.insert("memory_usage_percent".to_string(), usage_percent);
            metrics.insert("memory_used_gb".to_string(), memory_info.used as f32 / (1024.0 * 1024.0 * 1024.0));
        }

        // 磁盘指标
        {
            let mut disk = disk_monitor.write().await;
            let disk_info = disk.get_info();

            // 计算总体磁盘使用率
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

            metrics.insert("disk_usage_percent".to_string(), usage_percent);
        }

        metrics
    }

    /// 添加新规则
    pub async fn add_rule(&self, rule: AlertRule) {
        let mut rules = self.rules.write().await;
        rules.push(rule);
        info!("Added new alert rule, total rules: {}", rules.len());
    }

    /// 移除规则
    pub async fn remove_rule(&self, rule_id: &str) {
        let mut rules = self.rules.write().await;
        rules.retain(|r| r.id != rule_id);
        info!("Removed alert rule: {}", rule_id);
    }

    /// 获取所有规则
    pub async fn get_rules(&self) -> Vec<AlertRule> {
        let rules = self.rules.read().await;
        rules.clone()
    }

    /// 启用/禁用规则
    pub async fn toggle_rule(&self, rule_id: &str, enabled: bool) {
        let mut rules = self.rules.write().await;
        if let Some(rule) = rules.iter_mut().find(|r| r.id == rule_id) {
            rule.enabled = enabled;
            info!("Rule {} is now {}", rule_id, if enabled { "enabled" } else { "disabled" });
        }
    }
}
