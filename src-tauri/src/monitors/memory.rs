use serde::Serialize;
use sysinfo::System;

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

        MemoryInfo {
            total,
            used,
            available,
            usage_percent,
            swap_total,
            swap_used,
            swap_usage_percent,
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
