use serde::{Serialize, Deserialize};

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub target: String,
}