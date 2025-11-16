use serde::Serialize;
use sysinfo::{Disks, System};

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

    /// 获取所有磁盘信息
    pub fn get_info(&mut self) -> DisksInfo {
        // 刷新磁盘列表
        self.disks.refresh_list();

        let mut disk_infos = Vec::new();
        let mut total_space = 0u64;
        let mut total_available = 0u64;

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

            let disk_info = DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                file_system: String::from_utf8_lossy(disk.file_system()).to_string(),
                total_space: total,
                available_space: available,
                used_space: used,
                usage_percent,
                is_removable: disk.is_removable(),
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
