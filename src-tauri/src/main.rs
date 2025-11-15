// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::System;

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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
