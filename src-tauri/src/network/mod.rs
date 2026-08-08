pub mod manager;
// pub mod ping;
pub mod resolver;
pub mod types;


use manager::analyze;
use types::*;

#[tauri::command]
pub fn analyze_network(
    request: NetworkRequest,
) -> NetworkResult { 
    
    analyze(request)

}