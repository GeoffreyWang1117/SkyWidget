use serde::Serialize;
use sysinfo::System;
use std::fs;

/// 内存错误信息
#[derive(Debug, Clone, Serialize)]
pub struct MemoryErrors {
    /// 可纠正错误计数
    pub correctable_errors: Option<u64>,

    /// 不可纠正错误计数
    pub uncorrectable_errors: Option<u64>,

    /// 是否检测到错误
    pub has_errors: bool,
}

/// 内存温度信息
#[derive(Debug, Clone, Serialize)]
pub struct MemoryTemperature {
    /// 内存温度 (°C)
    pub temperature: Option<f32>,

    /// 温度状态 (Normal/Warning/Critical)
    pub status: String,
}

/// 内存信息结构体
#[derive(Debug, Clone, Serialize)]
pub struct MemoryInfo {
    /// 总内存 (字节)
    pub total: u64,
    /// 已用内存 (字节)
    pub used: u64,
    /// 可用内存 (字节)
    pub available: u64,
    /// 内存使用率 (0-100)
    pub usage_percent: f64,
    /// 交换分区总大小 (字节)
    pub swap_total: u64,
    /// 交换分区已用 (字节)
    pub swap_used: u64,
    /// 交换分区使用率 (0-100)
    pub swap_usage_percent: f64,
    /// 内存温度信息
    pub temperature: Option<MemoryTemperature>,
    /// 内存错误信息
    pub errors: Option<MemoryErrors>,
}

pub struct MemoryMonitor {
    system: System,
}

impl MemoryMonitor {
    /// 创建新的内存监控器
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_memory();

        Self { system }
    }

    /// 读取内存温度（Linux）
    #[cfg(target_os = "linux")]
    fn read_memory_temperature() -> Option<f32> {
        // 尝试从多个可能的位置读取内存温度
        // 某些主板支持 DIMM 温度传感器

        // 1. 尝试从 /sys/class/hwmon/hwmon* 读取
        if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
            for entry in entries.flatten() {
                let hwmon_path = entry.path();

                // 读取传感器名称
                let name_path = hwmon_path.join("name");
                if let Ok(name) = fs::read_to_string(&name_path) {
                    let name_lower = name.trim().to_lowercase();

                    // 查找内存相关的传感器
                    if name_lower.contains("dimm") || name_lower.contains("mem") {
                        // 查找温度文件
                        if let Ok(files) = fs::read_dir(&hwmon_path) {
                            for file in files.flatten() {
                                let file_name = file.file_name().to_string_lossy().to_string();

                                if file_name.starts_with("temp") && file_name.ends_with("_input") {
                                    if let Ok(temp_str) = fs::read_to_string(file.path()) {
                                        if let Ok(temp_millis) = temp_str.trim().parse::<f32>() {
                                            return Some(temp_millis / 1000.0);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// 读取内存温度（非 Linux）
    #[cfg(not(target_os = "linux"))]
    fn read_memory_temperature() -> Option<f32> {
        // Windows/macOS 需要使用其他方法
        // Windows: WMI 查询或 LibreHardwareMonitor
        // macOS: SMC (System Management Controller)
        None
    }

    /// 读取内存 ECC 错误（Linux）
    #[cfg(target_os = "linux")]
    fn read_memory_errors() -> Option<MemoryErrors> {
        // 尝试从 /sys/devices/system/edac/mc/ 读取 ECC 错误
        // 注意：仅支持 ECC 内存的系统

        let mut correctable_errors = None;
        let mut uncorrectable_errors = None;

        // 检查是否存在 EDAC 目录
        let edac_path = "/sys/devices/system/edac/mc";
        if let Ok(entries) = fs::read_dir(edac_path) {
            for entry in entries.flatten() {
                let mc_path = entry.path();

                // 读取可纠正错误
                let ce_path = mc_path.join("ce_count");
                if let Ok(ce_str) = fs::read_to_string(&ce_path) {
                    if let Ok(ce_count) = ce_str.trim().parse::<u64>() {
                        correctable_errors = Some(correctable_errors.unwrap_or(0) + ce_count);
                    }
                }

                // 读取不可纠正错误
                let ue_path = mc_path.join("ue_count");
                if let Ok(ue_str) = fs::read_to_string(&ue_path) {
                    if let Ok(ue_count) = ue_str.trim().parse::<u64>() {
                        uncorrectable_errors = Some(uncorrectable_errors.unwrap_or(0) + ue_count);
                    }
                }
            }
        }

        if correctable_errors.is_some() || uncorrectable_errors.is_some() {
            let has_errors = correctable_errors.unwrap_or(0) > 0 || uncorrectable_errors.unwrap_or(0) > 0;

            Some(MemoryErrors {
                correctable_errors,
                uncorrectable_errors,
                has_errors,
            })
        } else {
            None
        }
    }

    /// 读取内存错误（非 Linux）
    #[cfg(not(target_os = "linux"))]
    fn read_memory_errors() -> Option<MemoryErrors> {
        // Windows/macOS 需要使用其他方法
        None
    }

    /// 判断温度状态
    fn determine_temp_status(temp: f32) -> String {
        if temp >= 85.0 {
            "Critical".to_string()
        } else if temp >= 75.0 {
            "Warning".to_string()
        } else {
            "Normal".to_string()
        }
    }

    /// 获取内存信息
    pub fn get_info(&mut self) -> MemoryInfo {
        // 刷新内存数据
        self.system.refresh_memory();

        let total = self.system.total_memory();
        let available = self.system.available_memory();
        let used = total - available;
        let usage_percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let swap_total = self.system.total_swap();
        let swap_used = self.system.used_swap();
        let swap_usage_percent = if swap_total > 0 {
            (swap_used as f64 / swap_total as f64) * 100.0
        } else {
            0.0
        };

        // 读取温度
        let temperature = Self::read_memory_temperature().map(|temp| {
            MemoryTemperature {
                status: Self::determine_temp_status(temp),
                temperature: Some(temp),
            }
        });

        // 读取错误
        let errors = Self::read_memory_errors();

        MemoryInfo {
            total,
            used,
            available,
            usage_percent,
            swap_total,
            swap_used,
            swap_usage_percent,
            temperature,
            errors,
        }
    }

    /// 格式化内存大小为人类可读格式
    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}
