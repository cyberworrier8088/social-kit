use serde::{Serialize, Deserialize};

use super::dns::DnsResult;


#[derive(Debug, Serialize, Deserialize)]
pub struct SslResult {

    pub status: String,

    pub subject: String,

    pub issuer: String,

    pub valid_from: String,

    pub tls_version: String,

    pub expires: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkResult {

    pub target: String,

    pub ip: String,

    pub online: bool,

    pub latency: Option<f64>,

    pub packets_sent: u32,

    pub packets_received: u32,

    pub packet_loss: f64,

    pub min_latency: Option<f64>,

    pub max_latency: Option<f64>,

    pub average_latency: Option<f64>,

    pub dns: Option<DnsResult>,

    pub ssl: Option<SslResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub target: String,
}