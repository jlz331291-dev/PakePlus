#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;

// 数据文件与 exe 同目录，便携版放 U 盘时数据跟着 U 盘走
fn data_file_path() -> PathBuf {
    let exe = std::env::current_exe().expect("failed to locate executable");
    let dir = exe
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    dir.join("securepass-data.json")
}

#[tauri::command]
fn read_data_file() -> Option<String> {
    let path = data_file_path();
    if path.exists() {
        fs::read_to_string(path).ok()
    } else {
        None
    }
}

#[tauri::command]
fn write_data_file(contents: String) -> Result<(), String> {
    fs::write(data_file_path(), contents).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_data_file, write_data_file])
        .run(tauri::generate_context!())
        .expect("error while running SecurePass");
}
