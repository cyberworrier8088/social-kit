pub mod manager;
pub mod ping;
pub mod resolver;
pub mod dns;
pub mod ssl;
pub mod headers;
pub mod web_files;
pub mod geo;
pub mod hosting;
pub mod whois;
pub mod reverse_dns;
pub mod types;



use manager::analyze;
use types::*;

#[tauri::command]
pub async fn analyze_network(
    request: NetworkRequest,
) -> Result<NetworkResult, String> { 
    
    Ok(analyze(request).await)

}


#[tauri::command]
pub async fn ping_network(
    mut target: String,
) -> Result<NetworkResult, String> {
    target = target.trim().trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/').to_string();

    let ip = crate::network::resolver::resolve_target(
        &target
    )?;


    let stats = crate::network::ping::ping_target(ip).await.map_err(|error| error.to_string())?;


    Ok(NetworkResult {

        target,

        ip: ip.to_string(),

        online: stats.packets_received > 0,

        latency: stats.average_latency,

        packets_sent: stats.packets_sent,

        packets_received: stats.packets_received,

        packet_loss: stats.packet_loss,

        min_latency: stats.min_latency,

        max_latency: stats.max_latency,

        average_latency: stats.average_latency,

        dns: None,

        ssl: None,


        security_headers: None,


        web_files: None,


        geo: None,

        hosting: None,

        whois: None,

        reverse_dns: None,
        
    })


}