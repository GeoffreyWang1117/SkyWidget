use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 时序指标数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    /// 时间戳（毫秒）
    pub timestamp: i64,

    /// 指标值
    pub value: f32,
}

/// 时序指标存储
pub struct MetricsStore {
    /// 内存存储（指标名 -> 数据点列表）
    data: HashMap<String, Vec<MetricDataPoint>>,

    /// 最大保留数据点数量
    max_data_points: usize,
}

impl MetricsStore {
    /// 创建新的指标存储
    pub fn new(max_data_points: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_data_points,
        }
    }

    /// 添加指标数据点
    pub fn add_metric(&mut self, metric_name: &str, value: f32) {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let data_point = MetricDataPoint { timestamp, value };

        let points = self.data.entry(metric_name.to_string()).or_insert_with(Vec::new);
        points.push(data_point);

        // 保持数据点数量在限制内
        if points.len() > self.max_data_points {
            points.drain(0..(points.len() - self.max_data_points));
        }
    }

    /// 获取指标数据
    pub fn get_metric(&self, metric_name: &str) -> Option<&Vec<MetricDataPoint>> {
        self.data.get(metric_name)
    }

    /// 获取指标最新值
    pub fn get_latest(&self, metric_name: &str) -> Option<f32> {
        self.data
            .get(metric_name)
            .and_then(|points| points.last())
            .map(|point| point.value)
    }

    /// 获取指标平均值（最近 N 个数据点）
    pub fn get_average(&self, metric_name: &str, last_n_points: usize) -> Option<f32> {
        self.data.get(metric_name).and_then(|points| {
            if points.is_empty() {
                return None;
            }

            let start = if points.len() > last_n_points {
                points.len() - last_n_points
            } else {
                0
            };

            let sum: f32 = points[start..].iter().map(|p| p.value).sum();
            let count = (points.len() - start) as f32;

            Some(sum / count)
        })
    }

    /// 清理旧数据（超过指定时间）
    pub fn cleanup_old_data(&mut self, max_age_seconds: i64) {
        let cutoff_time = chrono::Utc::now().timestamp_millis() - (max_age_seconds * 1000);

        for points in self.data.values_mut() {
            points.retain(|p| p.timestamp > cutoff_time);
        }

        info!("Cleaned up old metric data");
    }

    /// 获取所有指标名称
    pub fn get_metric_names(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    /// 导出所有数据为 JSON
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.data)
            .map_err(|e| format!("Failed to export metrics: {}", e))
    }
}
