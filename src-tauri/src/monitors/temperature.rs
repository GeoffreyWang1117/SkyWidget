use serde::Serialize;
use sysinfo::{ComponentRefreshKind, Components, RefreshKind};

/// 传感器类型
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum SensorType {
    Cpu,
    Gpu,
    Chipset,  // 南桥/PCH/FCH
    Disk,
    Other,
}

/// 温度传感器信息
#[derive(Debug, Clone, Serialize)]
pub struct TemperatureSensor {
    /// 传感器标签/名称
    pub label: String,

    /// 传感器类型
    pub sensor_type: SensorType,

    /// 当前温度 (°C)
    pub temperature: f32,

    /// 最高温度 (°C) - 如果可用
    pub max_temperature: Option<f32>,

    /// 临界温度 (°C) - 如果可用
    pub critical: Option<f32>,
}

/// 所有温度传感器信息
#[derive(Debug, Clone, Serialize)]
pub struct TemperatureInfo {
    /// 所有传感器列表
    pub sensors: Vec<TemperatureSensor>,

    /// CPU 平均温度 (°C) - 如果可用
    pub cpu_avg_temp: Option<f32>,

    /// 南桥/PCH 温度 (°C) - 如果可用
    pub chipset_temp: Option<f32>,

    /// 最高温度 (°C)
    pub max_temp: Option<f32>,
}

pub struct TemperatureMonitor {
    components: Components,
}

impl TemperatureMonitor {
    /// 创建新的温度监控器
    pub fn new() -> Self {
        let components = Components::new_with_refreshed_list();

        Self { components }
    }

    /// 识别传感器类型
    fn identify_sensor_type(label: &str) -> SensorType {
        let label_lower = label.to_lowercase();

        // 南桥/PCH/FCH 识别（优先级最高，因为最关键）
        if label_lower.contains("pch")           // Platform Controller Hub (Intel)
            || label_lower.contains("southbridge")
            || label_lower.contains("fch")       // Fusion Controller Hub (AMD)
            || label_lower.contains("ich")       // I/O Controller Hub (Intel 老式)
            || label_lower.contains("chipset")
            || label_lower.contains("sb")        // Southbridge 缩写
        {
            return SensorType::Chipset;
        }

        // CPU 识别
        if label_lower.contains("cpu")
            || label_lower.contains("core")
            || label_lower.contains("processor")
            || label_lower.contains("package")
        {
            return SensorType::Cpu;
        }

        // GPU 识别
        if label_lower.contains("gpu")
            || label_lower.contains("video")
            || label_lower.contains("graphics")
            || label_lower.contains("nvidia")
            || label_lower.contains("amd")
            || label_lower.contains("radeon")
        {
            return SensorType::Gpu;
        }

        // 磁盘识别
        if label_lower.contains("disk")
            || label_lower.contains("drive")
            || label_lower.contains("hdd")
            || label_lower.contains("ssd")
            || label_lower.contains("nvme")
        {
            return SensorType::Disk;
        }

        SensorType::Other
    }

    /// 获取温度信息
    pub fn get_info(&mut self) -> TemperatureInfo {
        // 刷新组件数据
        self.components.refresh();

        let mut sensors = Vec::new();
        let mut cpu_temps = Vec::new();
        let mut chipset_temp: Option<f32> = None;
        let mut max_temp: Option<f32> = None;

        for component in &self.components {
            let label = component.label().to_string();
            let temperature = component.temperature();
            let max_temperature = Some(component.max());
            let critical = component.critical();

            // 识别传感器类型
            let sensor_type = Self::identify_sensor_type(&label);

            // 收集传感器信息
            sensors.push(TemperatureSensor {
                label: label.clone(),
                sensor_type: sensor_type.clone(),
                temperature,
                max_temperature,
                critical,
            });

            // 收集 CPU 温度
            if sensor_type == SensorType::Cpu {
                cpu_temps.push(temperature);
            }

            // 收集南桥温度（取最高值，如果有多个）
            if sensor_type == SensorType::Chipset {
                if let Some(current_chipset) = chipset_temp {
                    if temperature > current_chipset {
                        chipset_temp = Some(temperature);
                    }
                } else {
                    chipset_temp = Some(temperature);
                }
            }

            // 更新最高温度
            if let Some(current_max) = max_temp {
                if temperature > current_max {
                    max_temp = Some(temperature);
                }
            } else {
                max_temp = Some(temperature);
            }
        }

        // 计算 CPU 平均温度
        let cpu_avg_temp = if !cpu_temps.is_empty() {
            Some(cpu_temps.iter().sum::<f32>() / cpu_temps.len() as f32)
        } else {
            None
        };

        TemperatureInfo {
            sensors,
            cpu_avg_temp,
            chipset_temp,
            max_temp,
        }
    }

    /// 检查是否支持温度监控
    pub fn is_supported(&self) -> bool {
        !self.components.is_empty()
    }
}

impl Default for TemperatureMonitor {
    fn default() -> Self {
        Self::new()
    }
}
