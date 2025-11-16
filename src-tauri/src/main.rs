// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitors;

use monitors::{CpuMonitor, DiskMonitor, MemoryMonitor};
use std::sync::Mutex;
use sysinfo::System;
use tauri::State;

// 全局状态管理
pub struct AppState {
    cpu_monitor: Mutex<CpuMonitor>,
    memory_monitor: Mutex<MemoryMonitor>,
    disk_monitor: Mutex<DiskMonitor>,
}

// 简单的问候命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}! 欢迎使用 SkyWidget 硬件监控工具。", name)
}

// 获取系统信息
#[tauri::command]
fn get_system_info() -> serde_json::Value {
    let mut sys = System::new_all();
    sys.refresh_all();

    serde_json::json!({
        "system_name": System::name(),
        "kernel_version": System::kernel_version(),
        "os_version": System::os_version(),
        "host_name": System::host_name(),
        "cpu_count": sys.cpus().len(),
        "total_memory": sys.total_memory(),
        "used_memory": sys.used_memory(),
    })
}

// 获取 CPU 信息
#[tauri::command]
fn get_cpu_info(state: State<AppState>) -> Result<monitors::cpu::CpuInfo, String> {
    state
        .cpu_monitor
        .lock()
        .map_err(|e| format!("Failed to lock CPU monitor: {}", e))
        .map(|mut monitor| monitor.get_info())
}

// 获取内存信息
#[tauri::command]
fn get_memory_info(state: State<AppState>) -> Result<monitors::memory::MemoryInfo, String> {
    state
        .memory_monitor
        .lock()
        .map_err(|e| format!("Failed to lock memory monitor: {}", e))
        .map(|mut monitor| monitor.get_info())
}

// 获取磁盘信息
#[tauri::command]
fn get_disk_info(state: State<AppState>) -> Result<monitors::disk::DisksInfo, String> {
    state
        .disk_monitor
        .lock()
        .map_err(|e| format!("Failed to lock disk monitor: {}", e))
        .map(|mut monitor| monitor.get_info())
}

// 获取所有硬件信息（一次性获取全部数据）
#[tauri::command]
fn get_all_hardware_info(state: State<AppState>) -> Result<serde_json::Value, String> {
    let cpu_info = get_cpu_info(state.clone())?;
    let memory_info = get_memory_info(state.clone())?;
    let disk_info = get_disk_info(state)?;

    Ok(serde_json::json!({
        "cpu": cpu_info,
        "memory": memory_info,
        "disk": disk_info,
        "timestamp": chrono::Utc::now().timestamp_millis(),
    }))
}

fn main() {
    // 初始化监控器
    let app_state = AppState {
        cpu_monitor: Mutex::new(CpuMonitor::new()),
        memory_monitor: Mutex::new(MemoryMonitor::new()),
        disk_monitor: Mutex::new(DiskMonitor::new()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            get_system_info,
            get_cpu_info,
            get_memory_info,
            get_disk_info,
            get_all_hardware_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
