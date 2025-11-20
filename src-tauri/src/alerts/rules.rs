use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 告警严重级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    /// 信息级别
    Info,
    /// 警告级别
    Warning,
    /// 错误级别
    Error,
    /// 严重级别
    Critical,
}

/// 告警条件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    /// CPU 使用率超过阈值 (%)
    CpuUsageAbove(f32),

    /// 内存使用率超过阈值 (%)
    MemoryUsageAbove(f32),

    /// 磁盘使用率超过阈值 (%)
    DiskUsageAbove(f32),

    /// CPU 温度超过阈值 (°C) - 预留
    CpuTemperatureAbove(f32),

    /// 南桥/PCH 温度超过阈值 (°C)
    ChipsetTemperatureAbove(f32),

    /// 风扇已停转
    FanStopped,

    /// 风扇转速过低
    FanSlowSpeed,

    /// NVMe/SSD 温度过高
    DiskTemperatureAbove(f32),

    /// 磁盘健康警告
    DiskHealthWarning,

    /// 电压异常
    VoltageAbnormal,

    /// 内存温度过高
    MemoryTemperatureAbove(f32),

    /// 内存 ECC 错误
    MemoryErrors,

    /// 自定义条件
    Custom {
        metric_name: String,
        threshold: f32,
        operator: String, // ">", "<", "==", "!="
    },
}

/// 告警规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// 规则 ID
    pub id: String,

    /// 规则名称
    pub name: String,

    /// 规则描述
    pub description: String,

    /// 告警条件
    pub condition: AlertCondition,

    /// 严重级别
    pub severity: AlertSeverity,

    /// 是否启用
    pub enabled: bool,

    /// 冷却时间（秒）- 防止重复告警
    pub cooldown_seconds: i64,

    /// 最后触发时间
    #[serde(skip)]
    pub last_triggered: Option<i64>,

    /// 通知目标节点 ID 列表（空表示通知所有节点）
    pub notify_nodes: Vec<String>,
}

impl AlertRule {
    /// 创建新规则
    pub fn new(
        id: String,
        name: String,
        description: String,
        condition: AlertCondition,
        severity: AlertSeverity,
    ) -> Self {
        Self {
            id,
            name,
            description,
            condition,
            severity,
            enabled: true,
            cooldown_seconds: 300, // 默认 5 分钟冷却
            last_triggered: None,
            notify_nodes: Vec::new(),
        }
    }

    /// 检查规则是否应该触发
    pub fn should_trigger(&self, metrics: &HashMap<String, f32>) -> bool {
        if !self.enabled {
            return false;
        }

        // 检查冷却时间
        if let Some(last_trigger) = self.last_triggered {
            let now = chrono::Utc::now().timestamp();
            if now - last_trigger < self.cooldown_seconds {
                return false;
            }
        }

        // 检查条件
        match &self.condition {
            AlertCondition::CpuUsageAbove(threshold) => {
                if let Some(&usage) = metrics.get("cpu_usage") {
                    usage > *threshold
                } else {
                    false
                }
            }
            AlertCondition::MemoryUsageAbove(threshold) => {
                if let Some(&usage) = metrics.get("memory_usage_percent") {
                    usage > *threshold
                } else {
                    false
                }
            }
            AlertCondition::DiskUsageAbove(threshold) => {
                if let Some(&usage) = metrics.get("disk_usage_percent") {
                    usage > *threshold
                } else {
                    false
                }
            }
            AlertCondition::CpuTemperatureAbove(threshold) => {
                if let Some(&temp) = metrics.get("cpu_temperature") {
                    temp > *threshold
                } else {
                    false
                }
            }
            AlertCondition::ChipsetTemperatureAbove(threshold) => {
                if let Some(&temp) = metrics.get("chipset_temperature") {
                    temp > *threshold
                } else {
                    false
                }
            }
            AlertCondition::FanStopped => {
                if let Some(&stopped) = metrics.get("fans_stopped_count") {
                    stopped > 0.0
                } else {
                    false
                }
            }
            AlertCondition::FanSlowSpeed => {
                if let Some(&slow) = metrics.get("fans_slow_speed_count") {
                    slow > 0.0
                } else {
                    false
                }
            }
            AlertCondition::DiskTemperatureAbove(threshold) => {
                if let Some(&temp) = metrics.get("disk_max_temperature") {
                    temp > *threshold
                } else {
                    false
                }
            }
            AlertCondition::DiskHealthWarning => {
                if let Some(&warning_disks) = metrics.get("disk_warning_count") {
                    warning_disks > 0.0
                } else if let Some(&critical_disks) = metrics.get("disk_critical_count") {
                    critical_disks > 0.0
                } else {
                    false
                }
            }
            AlertCondition::VoltageAbnormal => {
                if let Some(&abnormal) = metrics.get("voltage_abnormal_count") {
                    abnormal > 0.0
                } else {
                    false
                }
            }
            AlertCondition::MemoryTemperatureAbove(threshold) => {
                if let Some(&temp) = metrics.get("memory_temperature") {
                    temp > *threshold
                } else {
                    false
                }
            }
            AlertCondition::MemoryErrors => {
                if let Some(&uncorrectable) = metrics.get("memory_uncorrectable_errors") {
                    uncorrectable > 0.0
                } else {
                    false
                }
            }
            AlertCondition::Custom {
                metric_name,
                threshold,
                operator,
            } => {
                if let Some(&value) = metrics.get(metric_name) {
                    match operator.as_str() {
                        ">" => value > *threshold,
                        "<" => value < *threshold,
                        "==" => (value - threshold).abs() < 0.001,
                        "!=" => (value - threshold).abs() >= 0.001,
                        _ => false,
                    }
                } else {
                    false
                }
            }
        }
    }

    /// 生成告警消息
    pub fn generate_message(&self, metrics: &HashMap<String, f32>) -> String {
        match &self.condition {
            AlertCondition::CpuUsageAbove(threshold) => {
                let usage = metrics.get("cpu_usage").unwrap_or(&0.0);
                format!(
                    "{}: CPU 使用率 {:.1}% 超过阈值 {:.1}%",
                    self.name, usage, threshold
                )
            }
            AlertCondition::MemoryUsageAbove(threshold) => {
                let usage = metrics.get("memory_usage_percent").unwrap_or(&0.0);
                format!(
                    "{}: 内存使用率 {:.1}% 超过阈值 {:.1}%",
                    self.name, usage, threshold
                )
            }
            AlertCondition::DiskUsageAbove(threshold) => {
                let usage = metrics.get("disk_usage_percent").unwrap_or(&0.0);
                format!(
                    "{}: 磁盘使用率 {:.1}% 超过阈值 {:.1}%",
                    self.name, usage, threshold
                )
            }
            AlertCondition::CpuTemperatureAbove(threshold) => {
                let temp = metrics.get("cpu_temperature").unwrap_or(&0.0);
                format!(
                    "{}: CPU 温度 {:.1}°C 超过阈值 {:.1}°C",
                    self.name, temp, threshold
                )
            }
            AlertCondition::ChipsetTemperatureAbove(threshold) => {
                let temp = metrics.get("chipset_temperature").unwrap_or(&0.0);
                format!(
                    "⚠️ {}: 南桥/PCH 温度 {:.1}°C 超过阈值 {:.1}°C！可能导致磁盘掉线或 CMOS 错误！",
                    self.name, temp, threshold
                )
            }
            AlertCondition::FanStopped => {
                let stopped_count = metrics.get("fans_stopped_count").unwrap_or(&0.0) as i32;
                format!(
                    "🚨 {}: 检测到 {} 个风扇已停转！可能导致硬件过热和损坏！",
                    self.name, stopped_count
                )
            }
            AlertCondition::FanSlowSpeed => {
                let slow_count = metrics.get("fans_slow_speed_count").unwrap_or(&0.0) as i32;
                format!(
                    "⚠️ {}: 检测到 {} 个风扇转速过低！请检查风扇状态。",
                    self.name, slow_count
                )
            }
            AlertCondition::DiskTemperatureAbove(threshold) => {
                let temp = metrics.get("disk_max_temperature").unwrap_or(&0.0);
                format!(
                    "🔥 {}: NVMe/SSD 温度 {:.1}°C 超过阈值 {:.1}°C！可能导致性能下降或数据丢失！",
                    self.name, temp, threshold
                )
            }
            AlertCondition::DiskHealthWarning => {
                let warning = metrics.get("disk_warning_count").unwrap_or(&0.0) as i32;
                let critical = metrics.get("disk_critical_count").unwrap_or(&0.0) as i32;
                if critical > 0 {
                    format!(
                        "🚨 {}: 检测到 {} 个磁盘处于严重状态！请立即备份数据！",
                        self.name, critical
                    )
                } else {
                    format!(
                        "⚠️ {}: 检测到 {} 个磁盘健康状态警告！建议检查磁盘状态。",
                        self.name, warning
                    )
                }
            }
            AlertCondition::VoltageAbnormal => {
                let abnormal = metrics.get("voltage_abnormal_count").unwrap_or(&0.0) as i32;
                format!(
                    "⚡ {}: 检测到 {} 个电压异常！可能导致系统不稳定或损坏硬件！",
                    self.name, abnormal
                )
            }
            AlertCondition::MemoryTemperatureAbove(threshold) => {
                let temp = metrics.get("memory_temperature").unwrap_or(&0.0);
                format!(
                    "🔥 {}: 内存温度 {:.1}°C 超过阈值 {:.1}°C！可能导致系统不稳定！",
                    self.name, temp, threshold
                )
            }
            AlertCondition::MemoryErrors => {
                let uncorrectable = metrics.get("memory_uncorrectable_errors").unwrap_or(&0.0) as i32;
                format!(
                    "🚨 {}: 检测到 {} 个内存不可纠正错误！可能导致系统崩溃或数据损坏！",
                    self.name, uncorrectable
                )
            }
            AlertCondition::Custom { metric_name, .. } => {
                format!("{}: 自定义指标 {} 触发告警", self.name, metric_name)
            }
        }
    }

    /// 更新最后触发时间
    pub fn mark_triggered(&mut self) {
        self.last_triggered = Some(chrono::Utc::now().timestamp());
    }
}

/// 默认告警规则
pub fn default_rules() -> Vec<AlertRule> {
    vec![
        AlertRule::new(
            "cpu_high".to_string(),
            "CPU 高负载告警".to_string(),
            "CPU 使用率超过 80%".to_string(),
            AlertCondition::CpuUsageAbove(80.0),
            AlertSeverity::Warning,
        ),
        AlertRule::new(
            "cpu_critical".to_string(),
            "CPU 严重告警".to_string(),
            "CPU 使用率超过 95%".to_string(),
            AlertCondition::CpuUsageAbove(95.0),
            AlertSeverity::Critical,
        ),
        AlertRule::new(
            "memory_high".to_string(),
            "内存高负载告警".to_string(),
            "内存使用率超过 85%".to_string(),
            AlertCondition::MemoryUsageAbove(85.0),
            AlertSeverity::Warning,
        ),
        AlertRule::new(
            "disk_high".to_string(),
            "磁盘高负载告警".to_string(),
            "磁盘使用率超过 90%".to_string(),
            AlertCondition::DiskUsageAbove(90.0),
            AlertSeverity::Warning,
        ),
        AlertRule::new(
            "chipset_warning".to_string(),
            "南桥温度警告".to_string(),
            "南桥/PCH 温度超过 60°C，可能影响系统稳定性".to_string(),
            AlertCondition::ChipsetTemperatureAbove(60.0),
            AlertSeverity::Warning,
        ),
        AlertRule::new(
            "chipset_critical".to_string(),
            "南桥温度严重告警".to_string(),
            "南桥/PCH 温度超过 70°C，可能导致磁盘掉线或 CMOS 错误".to_string(),
            AlertCondition::ChipsetTemperatureAbove(70.0),
            AlertSeverity::Critical,
        ),
        AlertRule::new(
            "fan_stopped".to_string(),
            "风扇停转告警".to_string(),
            "检测到风扇停转，可能导致硬件过热和损坏".to_string(),
            AlertCondition::FanStopped,
            AlertSeverity::Critical,
        ),
        AlertRule::new(
            "fan_slow_speed".to_string(),
            "风扇转速过低告警".to_string(),
            "检测到风扇转速过低，请检查风扇状态".to_string(),
            AlertCondition::FanSlowSpeed,
            AlertSeverity::Warning,
        ),
        AlertRule::new(
            "nvme_temp_high".to_string(),
            "NVMe/SSD 温度过高".to_string(),
            "NVMe/SSD 温度超过 70°C，可能导致性能下降".to_string(),
            AlertCondition::DiskTemperatureAbove(70.0),
            AlertSeverity::Warning,
        ),
        AlertRule::new(
            "nvme_temp_critical".to_string(),
            "NVMe/SSD 温度严重".to_string(),
            "NVMe/SSD 温度超过 80°C，可能导致数据丢失".to_string(),
            AlertCondition::DiskTemperatureAbove(80.0),
            AlertSeverity::Critical,
        ),
        AlertRule::new(
            "disk_health_warning".to_string(),
            "磁盘健康警告".to_string(),
            "检测到磁盘健康状态异常，请检查 SMART 状态".to_string(),
            AlertCondition::DiskHealthWarning,
            AlertSeverity::Error,
        ),
        AlertRule::new(
            "voltage_abnormal".to_string(),
            "电压异常告警".to_string(),
            "检测到电压偏离正常范围，可能影响系统稳定性".to_string(),
            AlertCondition::VoltageAbnormal,
            AlertSeverity::Warning,
        ),
        AlertRule::new(
            "memory_temp_high".to_string(),
            "内存温度过高".to_string(),
            "内存温度超过 75°C，可能影响系统稳定性".to_string(),
            AlertCondition::MemoryTemperatureAbove(75.0),
            AlertSeverity::Warning,
        ),
        AlertRule::new(
            "memory_errors_critical".to_string(),
            "内存错误严重告警".to_string(),
            "检测到内存不可纠正错误，系统可能不稳定".to_string(),
            AlertCondition::MemoryErrors,
            AlertSeverity::Critical,
        ),
    ]
}
