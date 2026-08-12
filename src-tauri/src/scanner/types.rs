use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ScanRequest {
    pub target: String,
    pub port: u16,
    pub timeout: u64,
}

#[derive(Debug, Serialize)]
pub struct ScanResult {
    pub target: String,
    pub port: u16,
    pub open: bool,
    pub service: Option<String>,
}