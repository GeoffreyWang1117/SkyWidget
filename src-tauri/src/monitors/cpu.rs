use serde::Serialize;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

/// CPU 信息结构体
#[derive(Debug, Clone, Serialize)]
pub struct CpuInfo {
    /// CPU 品牌和型号
    pub brand: String,
    /// CPU 核心数量
    pub core_count: usize,
    /// 总体 CPU 使用率 (0-100)
    pub usage: f32,
    /// 各核心使用率
    pub core_usage: Vec<f32>,
    /// CPU 频率 (MHz)
    pub frequency: u64,
}

pub struct CpuMonitor {
    system: System,
}

impl CpuMonitor {
    /// 创建新的 CPU 监控器
    pub fn new() -> Self {
        let mut system = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
        );

        // 首次刷新 CPU 信息（sysinfo 需要两次刷新才能获取准确的使用率）
        system.refresh_cpu_all();

        Self { system }
    }

    /// 获取 CPU 信息
    pub fn get_info(&mut self) -> CpuInfo {
        // 刷新 CPU 数据
        self.system.refresh_cpu_all();

        let cpus = self.system.cpus();

        // 计算总体使用率（所有核心平均）
        let total_usage = if !cpus.is_empty() {
            cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
        } else {
            0.0
        };

        // 获取各核心使用率
        let core_usage: Vec<f32> = cpus.iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        // 获取 CPU 品牌（使用第一个核心的信息）
        let brand = cpus.first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        // 获取频率（使用第一个核心的频率）
        let frequency = cpus.first()
            .map(|cpu| cpu.frequency())
            .unwrap_or(0);

        CpuInfo {
            brand,
            core_count: cpus.len(),
            usage: total_usage,
            core_usage,
            frequency,
        }
    }
}

impl Default for CpuMonitor {
    fn default() -> Self {
        Self::new()
    }
}
