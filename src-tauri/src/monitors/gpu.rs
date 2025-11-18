use serde::{Deserialize, Serialize};
use log::{warn, info};

/// GPU 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Unknown,
}

/// 单个 GPU 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// GPU 索引
    pub index: u32,

    /// GPU 名称
    pub name: String,

    /// GPU 厂商
    pub vendor: GpuVendor,

    /// GPU 使用率 (0-100)
    pub utilization: Option<f32>,

    /// GPU 温度 (℃)
    pub temperature: Option<f32>,

    /// 显存总量 (MB)
    pub memory_total: Option<u64>,

    /// 显存已用 (MB)
    pub memory_used: Option<u64>,

    /// 显存使用率 (0-100)
    pub memory_utilization: Option<f32>,

    /// 功耗 (W)
    pub power_usage: Option<f32>,

    /// 风扇转速 (0-100)
    pub fan_speed: Option<f32>,

    /// 时钟频率 (MHz)
    pub clock_speed: Option<u32>,

    /// 驱动版本
    pub driver_version: Option<String>,
}

/// GPU 监控器
pub struct GpuMonitor {
    #[cfg(feature = "nvidia")]
    nvml: Option<nvml_wrapper::Nvml>,
}

impl GpuMonitor {
    /// 创建新的 GPU 监控器
    pub fn new() -> Self {
        #[cfg(feature = "nvidia")]
        {
            match nvml_wrapper::Nvml::init() {
                Ok(nvml) => {
                    info!("NVML initialized successfully");
                    Self { nvml: Some(nvml) }
                }
                Err(e) => {
                    warn!("Failed to initialize NVML: {}", e);
                    Self { nvml: None }
                }
            }
        }

        #[cfg(not(feature = "nvidia"))]
        {
            warn!("NVIDIA GPU support not compiled in");
            Self {}
        }
    }

    /// 获取所有 GPU 信息
    pub fn get_info(&self) -> Vec<GpuInfo> {
        let mut gpus = Vec::new();

        // 获取 NVIDIA GPU 信息
        #[cfg(feature = "nvidia")]
        if let Some(ref nvml) = self.nvml {
            gpus.extend(self.get_nvidia_gpus(nvml));
        }

        // 获取 AMD GPU 信息（如果有 AMD GPU）
        // 注意：AMD 需要特定的库支持，这里先标记为未来扩展
        // gpus.extend(self.get_amd_gpus());

        // 获取 Intel GPU 信息（如果有 Intel GPU）
        // 注意：Intel Arc 需要特定的库支持，这里先标记为未来扩展
        // gpus.extend(self.get_intel_gpus());

        // 如果没有检测到任何 GPU，返回一个占位信息
        if gpus.is_empty() {
            gpus.push(GpuInfo {
                index: 0,
                name: "No discrete GPU detected".to_string(),
                vendor: GpuVendor::Unknown,
                utilization: None,
                temperature: None,
                memory_total: None,
                memory_used: None,
                memory_utilization: None,
                power_usage: None,
                fan_speed: None,
                clock_speed: None,
                driver_version: None,
            });
        }

        gpus
    }

    /// 获取 NVIDIA GPU 信息
    #[cfg(feature = "nvidia")]
    fn get_nvidia_gpus(&self, nvml: &nvml_wrapper::Nvml) -> Vec<GpuInfo> {
        let mut gpus = Vec::new();

        match nvml.device_count() {
            Ok(count) => {
                for i in 0..count {
                    match nvml.device_by_index(i) {
                        Ok(device) => {
                            let gpu_info = self.get_nvidia_device_info(&device, i);
                            gpus.push(gpu_info);
                        }
                        Err(e) => {
                            warn!("Failed to get NVIDIA device {}: {}", i, e);
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Failed to get NVIDIA device count: {}", e);
            }
        }

        gpus
    }

    /// 获取单个 NVIDIA 设备信息
    #[cfg(feature = "nvidia")]
    fn get_nvidia_device_info(
        &self,
        device: &nvml_wrapper::Device,
        index: u32,
    ) -> GpuInfo {
        let name = device.name().unwrap_or_else(|_| "NVIDIA GPU".to_string());

        // GPU 使用率
        let utilization = device
            .utilization_rates()
            .ok()
            .map(|rates| rates.gpu as f32);

        // GPU 温度
        let temperature = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .ok()
            .map(|t| t as f32);

        // 显存信息
        let (memory_total, memory_used, memory_utilization) = match device.memory_info() {
            Ok(mem_info) => {
                let total_mb = (mem_info.total / 1024 / 1024) as u64;
                let used_mb = (mem_info.used / 1024 / 1024) as u64;
                let util = if mem_info.total > 0 {
                    (mem_info.used as f32 / mem_info.total as f32) * 100.0
                } else {
                    0.0
                };
                (Some(total_mb), Some(used_mb), Some(util))
            }
            Err(_) => (None, None, None),
        };

        // 功耗
        let power_usage = device
            .power_usage()
            .ok()
            .map(|p| p as f32 / 1000.0); // mW to W

        // 风扇转速
        let fan_speed = device
            .fan_speed(0)
            .ok()
            .map(|speed| speed as f32);

        // 时钟频率
        let clock_speed = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .ok();

        // 驱动版本
        let driver_version = device.driver_version().ok();

        GpuInfo {
            index,
            name,
            vendor: GpuVendor::Nvidia,
            utilization,
            temperature,
            memory_total,
            memory_used,
            memory_utilization,
            power_usage,
            fan_speed,
            clock_speed,
            driver_version,
        }
    }

    /// 检查是否支持 GPU 监控
    pub fn is_supported(&self) -> bool {
        #[cfg(feature = "nvidia")]
        {
            self.nvml.is_some()
        }

        #[cfg(not(feature = "nvidia"))]
        {
            false
        }
    }

    /// 获取支持的 GPU 类型
    pub fn get_supported_vendors(&self) -> Vec<String> {
        let mut vendors = Vec::new();

        #[cfg(feature = "nvidia")]
        if self.nvml.is_some() {
            vendors.push("NVIDIA".to_string());
        }

        // AMD 和 Intel 支持标记为未来扩展
        // vendors.push("AMD".to_string());
        // vendors.push("Intel".to_string());

        if vendors.is_empty() {
            vendors.push("None".to_string());
        }

        vendors
    }
}

impl Default for GpuMonitor {
    fn default() -> Self {
        Self::new()
    }
}
