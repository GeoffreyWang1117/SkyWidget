use serde::Serialize;
use sysinfo::Components;
use std::fs;

/// 电压类型
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum VoltageType {
    Cpu,         // CPU 核心电压
    Memory,      // 内存电压
    Vcore,       // 核心电压
    V12,         // +12V
    V5,          // +5V
    V3_3,        // +3.3V
    Vbat,        // 主板电池电压
    Other,       // 其他
}

/// 电压状态
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum VoltageStatus {
    Normal,      // 正常
    Low,         // 偏低
    High,        // 偏高
    Critical,    // 危险
    Unknown,     // 未知
}

/// 单个电压传感器信息
#[derive(Debug, Clone, Serialize)]
pub struct VoltageInfo {
    /// 传感器标签
    pub label: String,

    /// 电压类型
    pub voltage_type: VoltageType,

    /// 当前电压 (V)
    pub voltage: f32,

    /// 最小电压 (V) - 如果支持
    pub min_voltage: Option<f32>,

    /// 最大电压 (V) - 如果支持
    pub max_voltage: Option<f32>,

    /// 标称电压 (V) - 参考值
    pub nominal_voltage: Option<f32>,

    /// 电压状态
    pub status: VoltageStatus,
}

/// 所有电压信息汇总
#[derive(Debug, Clone, Serialize)]
pub struct PowerInfo {
    /// 所有电压传感器
    pub voltages: Vec<VoltageInfo>,

    /// 传感器数量
    pub sensor_count: usize,

    /// 异常电压数量
    pub abnormal_count: usize,

    /// 是否检测到电压问题
    pub has_issues: bool,
}

pub struct PowerMonitor {
    components: Components,
}

impl PowerMonitor {
    /// 创建新的电源监控器
    pub fn new() -> Self {
        let components = Components::new_with_refreshed_list();

        Self { components }
    }

    /// 识别电压类型
    fn identify_voltage_type(label: &str) -> VoltageType {
        let label_lower = label.to_lowercase();

        // CPU 电压
        if label_lower.contains("cpu") || label_lower.contains("vcore") || label_lower.contains("processor") {
            return VoltageType::Cpu;
        }

        // 内存电压
        if label_lower.contains("mem") || label_lower.contains("dimm") || label_lower.contains("dram") {
            return VoltageType::Memory;
        }

        // +12V
        if label_lower.contains("12v") || label_lower.contains("+12") {
            return VoltageType::V12;
        }

        // +5V
        if label_lower.contains("5v") || label_lower.contains("+5") {
            return VoltageType::V5;
        }

        // +3.3V
        if label_lower.contains("3.3v") || label_lower.contains("3v3") || label_lower.contains("+3.3") {
            return VoltageType::V3_3;
        }

        // 电池电压
        if label_lower.contains("bat") || label_lower.contains("cmos") {
            return VoltageType::Vbat;
        }

        // Vcore
        if label_lower.contains("vcore") {
            return VoltageType::Vcore;
        }

        VoltageType::Other
    }

    /// 获取标称电压
    fn get_nominal_voltage(voltage_type: &VoltageType) -> Option<f32> {
        match voltage_type {
            VoltageType::V12 => Some(12.0),
            VoltageType::V5 => Some(5.0),
            VoltageType::V3_3 => Some(3.3),
            VoltageType::Vbat => Some(3.0),  // CR2032 电池
            _ => None, // CPU/Memory 电压因型号而异
        }
    }

    /// 判断电压状态
    fn determine_voltage_status(voltage: f32, voltage_type: &VoltageType, nominal: Option<f32>) -> VoltageStatus {
        if let Some(nom) = nominal {
            let deviation_percent = ((voltage - nom).abs() / nom) * 100.0;

            if deviation_percent > 10.0 {
                VoltageStatus::Critical
            } else if deviation_percent > 5.0 {
                if voltage > nom {
                    VoltageStatus::High
                } else {
                    VoltageStatus::Low
                }
            } else {
                VoltageStatus::Normal
            }
        } else {
            // 没有标称值，使用经验值判断
            match voltage_type {
                VoltageType::Cpu | VoltageType::Vcore => {
                    if voltage > 1.5 || voltage < 0.5 {
                        VoltageStatus::Critical
                    } else if voltage > 1.4 || voltage < 0.7 {
                        VoltageStatus::High
                    } else {
                        VoltageStatus::Normal
                    }
                }
                VoltageType::Memory => {
                    if voltage > 1.5 || voltage < 1.0 {
                        VoltageStatus::Critical
                    } else if voltage > 1.4 || voltage < 1.1 {
                        VoltageStatus::High
                    } else {
                        VoltageStatus::Normal
                    }
                }
                VoltageType::Vbat => {
                    if voltage < 2.5 {
                        VoltageStatus::Low  // 电池需要更换
                    } else if voltage < 2.8 {
                        VoltageStatus::Critical
                    } else {
                        VoltageStatus::Normal
                    }
                }
                _ => VoltageStatus::Unknown,
            }
        }
    }

    /// 读取电压信息（Linux）
    #[cfg(target_os = "linux")]
    fn read_voltage_sensors() -> Vec<(String, f32)> {
        let mut voltages = Vec::new();

        // 尝试从 /sys/class/hwmon/hwmon*/in*_input 读取
        if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
            for entry in entries.flatten() {
                let hwmon_path = entry.path();

                // 读取传感器名称
                let name_path = hwmon_path.join("name");
                let sensor_name = fs::read_to_string(&name_path).unwrap_or_default().trim().to_string();

                // 查找所有 in*_input 文件
                if let Ok(files) = fs::read_dir(&hwmon_path) {
                    for file in files.flatten() {
                        let file_name = file.file_name().to_string_lossy().to_string();

                        if file_name.starts_with("in") && file_name.ends_with("_input") {
                            if let Ok(voltage_str) = fs::read_to_string(file.path()) {
                                if let Ok(voltage_millivolts) = voltage_str.trim().parse::<f32>() {
                                    let voltage = voltage_millivolts / 1000.0; // mV -> V

                                    // 读取对应的 label 文件
                                    let index = file_name.trim_start_matches("in").trim_end_matches("_input");
                                    let label_path = hwmon_path.join(format!("in{}_label", index));
                                    let label = if let Ok(label_str) = fs::read_to_string(&label_path) {
                                        label_str.trim().to_string()
                                    } else {
                                        format!("{} in{}", sensor_name, index)
                                    };

                                    voltages.push((label, voltage));
                                }
                            }
                        }
                    }
                }
            }
        }

        voltages
    }

    /// 读取电压信息（非 Linux）
    #[cfg(not(target_os = "linux"))]
    fn read_voltage_sensors() -> Vec<(String, f32)> {
        // Windows/macOS 需要使用其他方法
        // Windows: WMI 查询或 LibreHardwareMonitor
        // macOS: SMC (System Management Controller)
        Vec::new()
    }

    /// 获取电压信息
    pub fn get_info(&mut self) -> PowerInfo {
        // 刷新组件
        self.components.refresh();

        let voltage_readings = Self::read_voltage_sensors();
        let mut voltages = Vec::new();
        let mut abnormal_count = 0;

        for (label, voltage) in voltage_readings {
            let voltage_type = Self::identify_voltage_type(&label);
            let nominal_voltage = Self::get_nominal_voltage(&voltage_type);
            let status = Self::determine_voltage_status(voltage, &voltage_type, nominal_voltage);

            if !matches!(status, VoltageStatus::Normal | VoltageStatus::Unknown) {
                abnormal_count += 1;
            }

            voltages.push(VoltageInfo {
                label,
                voltage_type,
                voltage,
                min_voltage: None,  // 需要持续监控来确定
                max_voltage: None,  // 需要持续监控来确定
                nominal_voltage,
                status,
            });
        }

        PowerInfo {
            sensor_count: voltages.len(),
            has_issues: abnormal_count > 0,
            abnormal_count,
            voltages,
        }
    }

    /// 检查是否支持电压监控
    pub fn is_supported(&self) -> bool {
        // 简单检查：如果能读取到电压传感器，说明支持
        !Self::read_voltage_sensors().is_empty()
    }
}

impl Default for PowerMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_voltage_type() {
        assert_eq!(
            PowerMonitor::identify_voltage_type("CPU Vcore"),
            VoltageType::Cpu
        );
        assert_eq!(
            PowerMonitor::identify_voltage_type("+12V"),
            VoltageType::V12
        );
        assert_eq!(
            PowerMonitor::identify_voltage_type("DIMM AB"),
            VoltageType::Memory
        );
    }

    #[test]
    fn test_nominal_voltage() {
        assert_eq!(PowerMonitor::get_nominal_voltage(&VoltageType::V12), Some(12.0));
        assert_eq!(PowerMonitor::get_nominal_voltage(&VoltageType::V5), Some(5.0));
        assert_eq!(PowerMonitor::get_nominal_voltage(&VoltageType::V3_3), Some(3.3));
    }
}
