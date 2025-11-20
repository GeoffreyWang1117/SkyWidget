use serde::Serialize;
use sysinfo::{Components, RefreshKind};

/// 风扇状态
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum FanStatus {
    Running,        // 正常运行
    Stopped,        // 停转 (0 RPM)
    SlowSpeed,      // 转速过低 (< 500 RPM)
    Unknown,        // 未知状态
}

/// 风扇类型
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum FanType {
    CpuFan,         // CPU 风扇
    CaseFan,        // 机箱风扇
    GpuFan,         // GPU 风扇
    ChipsetFan,     // 南桥/芯片组风扇
    PowerSupply,    // 电源风扇
    Other,          // 其他风扇
}

/// 风扇信息
#[derive(Debug, Clone, Serialize)]
pub struct FanInfo {
    /// 风扇标签/名称
    pub label: String,

    /// 风扇类型
    pub fan_type: FanType,

    /// 转速 (RPM)
    pub rpm: Option<f32>,

    /// PWM 百分比 (0-100%)
    pub pwm_percent: Option<f32>,

    /// 风扇状态
    pub status: FanStatus,

    /// 最小转速 (RPM) - 如果可用
    pub min_rpm: Option<f32>,

    /// 最大转速 (RPM) - 如果可用
    pub max_rpm: Option<f32>,
}

/// 所有风扇信息
#[derive(Debug, Clone, Serialize)]
pub struct AllFansInfo {
    /// 所有风扇列表
    pub fans: Vec<FanInfo>,

    /// 停转的风扇数量
    pub stopped_count: usize,

    /// 转速过低的风扇数量
    pub slow_speed_count: usize,

    /// 总风扇数量
    pub total_count: usize,
}

pub struct FanMonitor {
    components: Components,
    /// 慢速阈值 (RPM)
    slow_speed_threshold: f32,
}

impl FanMonitor {
    /// 创建新的风扇监控器
    pub fn new() -> Self {
        let components = Components::new_with_refreshed_list();

        Self {
            components,
            slow_speed_threshold: 500.0, // 默认 500 RPM 以下视为转速过低
        }
    }

    /// 设置慢速阈值
    pub fn set_slow_speed_threshold(&mut self, threshold: f32) {
        self.slow_speed_threshold = threshold;
    }

    /// 识别风扇类型
    fn identify_fan_type(label: &str) -> FanType {
        let label_lower = label.to_lowercase();

        // CPU 风扇识别
        if label_lower.contains("cpu")
            || label_lower.contains("processor")
            || label_lower.contains("cpu_fan")
            || label_lower.contains("cpufan")
        {
            return FanType::CpuFan;
        }

        // GPU 风扇识别
        if label_lower.contains("gpu")
            || label_lower.contains("video")
            || label_lower.contains("graphics")
            || label_lower.contains("gpu_fan")
            || label_lower.contains("vga")
        {
            return FanType::GpuFan;
        }

        // 南桥/芯片组风扇识别
        if label_lower.contains("chipset")
            || label_lower.contains("pch")
            || label_lower.contains("southbridge")
            || label_lower.contains("sb_fan")
        {
            return FanType::ChipsetFan;
        }

        // 电源风扇识别
        if label_lower.contains("psu")
            || label_lower.contains("power")
            || label_lower.contains("ps_fan")
        {
            return FanType::PowerSupply;
        }

        // 机箱风扇识别
        if label_lower.contains("case")
            || label_lower.contains("chassis")
            || label_lower.contains("sys")
            || label_lower.contains("intake")
            || label_lower.contains("exhaust")
            || label_lower.contains("front")
            || label_lower.contains("rear")
            || label_lower.contains("top")
            || label_lower.contains("bottom")
        {
            return FanType::CaseFan;
        }

        FanType::Other
    }

    /// 判断风扇状态
    fn determine_fan_status(&self, rpm: Option<f32>) -> FanStatus {
        match rpm {
            Some(speed) => {
                if speed <= 0.0 {
                    FanStatus::Stopped
                } else if speed < self.slow_speed_threshold {
                    FanStatus::SlowSpeed
                } else {
                    FanStatus::Running
                }
            }
            None => FanStatus::Unknown,
        }
    }

    /// 从组件中提取 RPM 信息
    /// 注意：sysinfo 的 Components 主要用于温度传感器，
    /// 风扇信息可能需要从标签中解析或使用其他方法
    fn extract_fan_rpm(&self, label: &str) -> Option<f32> {
        // sysinfo 库在某些平台上可能不直接提供风扇转速
        // 这里我们先返回 None，实际实现可能需要：
        // 1. Linux: 读取 /sys/class/hwmon/hwmon*/fan*_input
        // 2. Windows: 使用 WMI 或 LibreHardwareMonitor
        // 3. macOS: 使用 SMC (System Management Controller)

        // 临时实现：如果标签包含 "fan" 关键字，我们认为它是风扇
        // 但 sysinfo Components 主要是温度传感器，可能需要单独实现
        None
    }

    /// 获取所有风扇信息
    pub fn get_info(&mut self) -> AllFansInfo {
        // 刷新组件数据
        self.components.refresh();

        let mut fans = Vec::new();
        let mut stopped_count = 0;
        let mut slow_speed_count = 0;

        // 注意：sysinfo 的 Components 主要提供温度传感器
        // 风扇信息可能需要通过其他方式获取
        // 这里我们先遍历所有组件，查找可能的风扇相关信息
        for component in &self.components {
            let label = component.label().to_string();
            let label_lower = label.to_lowercase();

            // 检查是否是风扇相关的组件
            if label_lower.contains("fan") {
                // 尝试提取 RPM 信息
                let rpm = self.extract_fan_rpm(&label);
                let fan_type = Self::identify_fan_type(&label);
                let status = self.determine_fan_status(rpm);

                // 统计状态
                match status {
                    FanStatus::Stopped => stopped_count += 1,
                    FanStatus::SlowSpeed => slow_speed_count += 1,
                    _ => {}
                }

                fans.push(FanInfo {
                    label: label.clone(),
                    fan_type,
                    rpm,
                    pwm_percent: None, // 暂不支持 PWM 百分比
                    status,
                    min_rpm: None,
                    max_rpm: None,
                });
            }
        }

        AllFansInfo {
            total_count: fans.len(),
            fans,
            stopped_count,
            slow_speed_count,
        }
    }

    /// 检查是否支持风扇监控
    pub fn is_supported(&self) -> bool {
        // 检查是否有风扇相关的组件
        for component in &self.components {
            let label = component.label().to_lowercase();
            if label.contains("fan") {
                return true;
            }
        }
        false
    }

    /// 获取特定类型的风扇
    pub fn get_fans_by_type(&mut self, fan_type: FanType) -> Vec<FanInfo> {
        let all_info = self.get_info();
        all_info
            .fans
            .into_iter()
            .filter(|fan| fan.fan_type == fan_type)
            .collect()
    }

    /// 检查是否有风扇故障
    pub fn has_fan_failures(&mut self) -> bool {
        let info = self.get_info();
        info.stopped_count > 0 || info.slow_speed_count > 0
    }
}

impl Default for FanMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_fan_type() {
        assert_eq!(
            FanMonitor::identify_fan_type("CPU Fan"),
            FanType::CpuFan
        );
        assert_eq!(
            FanMonitor::identify_fan_type("case_fan1"),
            FanType::CaseFan
        );
        assert_eq!(
            FanMonitor::identify_fan_type("GPU Fan 1"),
            FanType::GpuFan
        );
        assert_eq!(
            FanMonitor::identify_fan_type("Chipset Fan"),
            FanType::ChipsetFan
        );
        assert_eq!(
            FanMonitor::identify_fan_type("PSU Fan"),
            FanType::PowerSupply
        );
    }

    #[test]
    fn test_fan_status_determination() {
        let monitor = FanMonitor::new();

        assert_eq!(
            monitor.determine_fan_status(Some(0.0)),
            FanStatus::Stopped
        );
        assert_eq!(
            monitor.determine_fan_status(Some(300.0)),
            FanStatus::SlowSpeed
        );
        assert_eq!(
            monitor.determine_fan_status(Some(1500.0)),
            FanStatus::Running
        );
        assert_eq!(
            monitor.determine_fan_status(None),
            FanStatus::Unknown
        );
    }
}
