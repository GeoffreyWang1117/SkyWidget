use serde::Serialize;
use sysinfo::{Disks, System};
use std::fs;
use std::path::Path;

/// 磁盘健康状态
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DiskHealthStatus {
    Good,        // 健康
    Warning,     // 警告（有问题但仍可用）
    Critical,    // 严重（即将失效）
    Failed,      // 失效
    Unknown,     // 未知
}

/// 磁盘类型
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DiskType {
    HDD,         // 机械硬盘
    SSD,         // SATA SSD
    NVMe,        // NVMe SSD
    Unknown,     // 未知类型
}

/// SMART 属性信息
#[derive(Debug, Clone, Serialize)]
pub struct SmartInfo {
    /// 是否支持 SMART
    pub supported: bool,

    /// 健康状态
    pub health_status: DiskHealthStatus,

    /// 温度 (°C) - 如果支持
    pub temperature: Option<f32>,

    /// 通电时间 (小时)
    pub power_on_hours: Option<u64>,

    /// 读写错误计数
    pub error_count: Option<u64>,

    /// 重分配扇区数
    pub reallocated_sectors: Option<u64>,

    /// 剩余寿命百分比 (SSD)
    pub remaining_life: Option<f32>,
}

/// 单个磁盘信息
#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    /// 磁盘名称
    pub name: String,
    /// 挂载点
    pub mount_point: String,
    /// 文件系统类型
    pub file_system: String,
    /// 总容量 (字节)
    pub total_space: u64,
    /// 可用空间 (字节)
    pub available_space: u64,
    /// 已用空间 (字节)
    pub used_space: u64,
    /// 使用率 (0-100)
    pub usage_percent: f64,
    /// 是否为可移动磁盘
    pub is_removable: bool,
    /// 磁盘类型
    pub disk_type: DiskType,
    /// SMART 信息
    pub smart_info: Option<SmartInfo>,
}

/// 所有磁盘信息汇总
#[derive(Debug, Clone, Serialize)]
pub struct DisksInfo {
    /// 所有磁盘列表
    pub disks: Vec<DiskInfo>,
    /// 总磁盘数量
    pub disk_count: usize,
    /// 所有磁盘总容量
    pub total_space: u64,
    /// 所有磁盘已用空间
    pub total_used: u64,
    /// 所有磁盘可用空间
    pub total_available: u64,
    /// 警告状态的磁盘数量
    pub warning_disks: usize,
    /// 严重状态的磁盘数量
    pub critical_disks: usize,
    /// 最高磁盘温度
    pub max_disk_temperature: Option<f32>,
}

pub struct DiskMonitor {
    disks: Disks,
}

impl DiskMonitor {
    /// 创建新的磁盘监控器
    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();

        Self { disks }
    }

    /// 识别磁盘类型
    fn identify_disk_type(name: &str, file_system: &str) -> DiskType {
        let name_lower = name.to_lowercase();

        // NVMe 识别
        if name_lower.contains("nvme") {
            return DiskType::NVMe;
        }

        // SSD 识别（通过常见命名）
        if name_lower.contains("ssd") {
            return DiskType::SSD;
        }

        // 机械硬盘识别
        if name_lower.contains("hdd") || name_lower.contains("st") || name_lower.contains("wd") {
            return DiskType::HDD;
        }

        // 默认未知
        DiskType::Unknown
    }

    /// 读取 NVMe 温度（Linux）
    fn read_nvme_temperature(device_name: &str) -> Option<f32> {
        // 尝试从 /sys/class/nvme/ 读取温度
        // 实际路径可能是 /sys/class/nvme/nvme0/hwmon/hwmon*/temp1_input

        #[cfg(target_os = "linux")]
        {
            // 提取设备编号，例如 "nvme0n1" -> "nvme0"
            let device_base = if let Some(pos) = device_name.find('n') {
                if let Some(num_end) = device_name[pos+1..].find(|c: char| !c.is_ascii_digit()) {
                    &device_name[..pos + num_end + 1]
                } else {
                    device_name
                }
            } else {
                device_name
            };

            let hwmon_path = format!("/sys/class/nvme/{}", device_base);
            if let Ok(entries) = fs::read_dir(&hwmon_path) {
                for entry in entries.flatten() {
                    if entry.file_name().to_string_lossy().starts_with("hwmon") {
                        let temp_path = entry.path().join("temp1_input");
                        if let Ok(temp_str) = fs::read_to_string(&temp_path) {
                            if let Ok(temp_millis) = temp_str.trim().parse::<f32>() {
                                return Some(temp_millis / 1000.0); // 转换为摄氏度
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// 获取 SMART 信息（基础实现）
    fn get_smart_info(name: &str, disk_type: &DiskType) -> Option<SmartInfo> {
        // 注意：完整的 SMART 实现需要：
        // 1. Linux: smartctl 命令或直接读取 /dev/ 设备（需要 root）
        // 2. Windows: WMI 查询或第三方库
        // 3. macOS: diskutil 或 smartctl

        // 这里提供一个基础框架
        let temperature = match disk_type {
            DiskType::NVMe => Self::read_nvme_temperature(name),
            _ => None, // HDD/SSD 温度读取需要其他方法
        };

        // 如果能读取到温度，说明至少部分支持
        let supported = temperature.is_some();

        if supported || matches!(disk_type, DiskType::NVMe | DiskType::SSD) {
            Some(SmartInfo {
                supported,
                health_status: DiskHealthStatus::Unknown, // 需要实际 SMART 查询
                temperature,
                power_on_hours: None,    // 需要 SMART 查询
                error_count: None,       // 需要 SMART 查询
                reallocated_sectors: None, // 需要 SMART 查询
                remaining_life: None,    // 需要 SMART 查询 (SSD only)
            })
        } else {
            None
        }
    }

    /// 获取所有磁盘信息
    pub fn get_info(&mut self) -> DisksInfo {
        // 刷新磁盘列表
        self.disks.refresh_list();

        let mut disk_infos = Vec::new();
        let mut total_space = 0u64;
        let mut total_available = 0u64;
        let mut warning_disks = 0;
        let mut critical_disks = 0;
        let mut max_temp: Option<f32> = None;

        for disk in self.disks.list() {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            let usage_percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            total_space += total;
            total_available += available;

            let name = disk.name().to_string_lossy().to_string();
            let file_system = String::from_utf8_lossy(disk.file_system()).to_string();
            let disk_type = Self::identify_disk_type(&name, &file_system);
            let smart_info = Self::get_smart_info(&name, &disk_type);

            // 统计健康状态
            if let Some(ref smart) = smart_info {
                match smart.health_status {
                    DiskHealthStatus::Warning => warning_disks += 1,
                    DiskHealthStatus::Critical | DiskHealthStatus::Failed => critical_disks += 1,
                    _ => {}
                }

                // 更新最高温度
                if let Some(temp) = smart.temperature {
                    max_temp = Some(max_temp.map_or(temp, |max| max.max(temp)));
                }
            }

            let disk_info = DiskInfo {
                name,
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                file_system,
                total_space: total,
                available_space: available,
                used_space: used,
                usage_percent,
                is_removable: disk.is_removable(),
                disk_type,
                smart_info,
            };

            disk_infos.push(disk_info);
        }

        let total_used = total_space.saturating_sub(total_available);

        DisksInfo {
            disk_count: disk_infos.len(),
            disks: disk_infos,
            total_space,
            total_used,
            total_available,
            warning_disks,
            critical_disks,
            max_disk_temperature: max_temp,
        }
    }

    /// 格式化磁盘大小为人类可读格式
    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if bytes >= TB {
            format!("{:.2} TB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
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

impl Default for DiskMonitor {
    fn default() -> Self {
        Self::new()
    }
}
