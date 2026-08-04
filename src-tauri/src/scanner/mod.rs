// src/scanner/mod.rs

pub mod tcp;
pub mod types;

use crate::scanner::tcp::scan_tcp;
use crate::scanner::types::{ScanRequest, ScanResult};

#[tauri::command]
pub fn scan(request: ScanRequest) -> ScanResult {
    scan_tcp(request)
}