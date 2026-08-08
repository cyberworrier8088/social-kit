pub mod manager;
pub mod ping;
pub mod resolver;
pub mod dns;
pub mod ssl;
pub mod types;



use manager::analyze;
use types::*;

#[tauri::command]
pub async fn analyze_network(
    request: NetworkRequest,
) -> Result<NetworkResult, String> { 
    
    Ok(analyze(request).await)

}