use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::alerts::rules::AlertSeverity;

/// 告警历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRecord {
    /// 记录 ID
    pub id: String,

    /// 规则 ID
    pub rule_id: String,

    /// 规则名称
    pub rule_name: String,

    /// 告警消息
    pub message: String,

    /// 严重级别
    pub severity: AlertSeverity,

    /// 触发时间
    pub timestamp: i64,

    /// 是否已确认
    pub acknowledged: bool,
}

/// 告警历史存储
pub struct AlertsStore {
    /// 告警记录队列
    records: VecDeque<AlertRecord>,

    /// 最大记录数量
    max_records: usize,
}

impl AlertsStore {
    /// 创建新的告警存储
    pub fn new(max_records: usize) -> Self {
        Self {
            records: VecDeque::new(),
            max_records,
        }
    }

    /// 添加告警记录
    pub fn add_record(
        &mut self,
        rule_id: &str,
        rule_name: &str,
        message: &str,
        severity: &AlertSeverity,
    ) {
        let record = AlertRecord {
            id: uuid::Uuid::new_v4().to_string(),
            rule_id: rule_id.to_string(),
            rule_name: rule_name.to_string(),
            message: message.to_string(),
            severity: severity.clone(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            acknowledged: false,
        };

        self.records.push_back(record);

        // 保持记录数量在限制内
        if self.records.len() > self.max_records {
            self.records.pop_front();
        }
    }

    /// 获取所有记录
    pub fn get_all_records(&self) -> Vec<AlertRecord> {
        self.records.iter().cloned().collect()
    }

    /// 获取未确认记录
    pub fn get_unacknowledged_records(&self) -> Vec<AlertRecord> {
        self.records
            .iter()
            .filter(|r| !r.acknowledged)
            .cloned()
            .collect()
    }

    /// 确认告警
    pub fn acknowledge(&mut self, record_id: &str) -> bool {
        if let Some(record) = self.records.iter_mut().find(|r| r.id == record_id) {
            record.acknowledged = true;
            true
        } else {
            false
        }
    }

    /// 清空所有记录
    pub fn clear(&mut self) {
        self.records.clear();
    }

    /// 获取记录数量
    pub fn count(&self) -> usize {
        self.records.len()
    }

    /// 导出为 JSON
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.records)
            .map_err(|e| format!("Failed to export alerts: {}", e))
    }
}
