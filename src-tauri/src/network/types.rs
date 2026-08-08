use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkResult {

    pub target: String,

    pub ip: String,

    pub online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub target: String,
}