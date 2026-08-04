// src/scanner/mod.rs

pub mod tcp;
pub mod types;

use crate::scanner::types::{ScanRequest, ScanResult};

#[tauri::command]
pub async fn scan(request: ScanRequest) -> Result<ScanResult, String> {
    tauri::async_runtime::spawn_blocking(move || crate::scanner::tcp::scan_tcp(request))
        .await
        .map_err(|e| e.to_string())
}