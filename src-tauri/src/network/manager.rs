use super::types::*;
 pub fn analyze(
    request: NetworkRequest,
 ) -> NetworkResult {
    NetworkResult {
        
        target: request.target,

        ip: String::new(),

        online: false,
    }
 }