use serde::Serialize;
use sysinfo::{ComponentRefreshKind, Components, RefreshKind};

/// 温度传感器信息
#[derive(Debug, Clone, Serialize)]
pub struct TemperatureSensor {
    /// 传感器标签/名称
    pub label: String,

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

    /// 获取温度信息
    pub fn get_info(&mut self) -> TemperatureInfo {
        // 刷新组件数据
        self.components.refresh();

        let mut sensors = Vec::new();
        let mut cpu_temps = Vec::new();
        let mut max_temp: Option<f32> = None;

        for component in &self.components {
            let label = component.label().to_string();
            let temperature = component.temperature();
            let max_temperature = Some(component.max());
            let critical = component.critical();

            // 收集传感器信息
            sensors.push(TemperatureSensor {
                label: label.clone(),
                temperature,
                max_temperature,
                critical,
            });

            // 收集 CPU 相关温度
            let label_lower = label.to_lowercase();
            if label_lower.contains("cpu")
                || label_lower.contains("core")
                || label_lower.contains("processor")
            {
                cpu_temps.push(temperature);
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
