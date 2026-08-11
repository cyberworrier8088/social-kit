use serde::{Serialize, Deserialize};

use super::dns::DnsResult;
use super::headers::SecurityHeaders;
use super::web_files::WebFileResult;
use super::geo::GeoResult;


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

    pub security_headers: Option<SecurityHeaders>,

    pub web_files: Option<WebFileResult>,

    pub geo: Option<GeoResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub target: String,
}