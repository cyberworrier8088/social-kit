use tauri::Manager;
mod system;
mod scanner;
mod osint;
mod network;
mod phishing;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      app.manage(system::SystemState::new());
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      system::get_cpu_info,
      system::get_memory_info,
      system::get_disk_info,
      system::get_network_info,
      system::get_os_info,
      system::open_in_browser,
      scanner::scan,
      osint::search_git,
      osint::search_riddit,
      osint::search_reddit,
      osint::search_mastodon_command,
      osint::search_keybase_command,
      osint::search_devto_command,
      osint::search_instagram_command,
      osint::search_stackoverflow_command,
      osint::search_all_command,
      network::analyze_network,
      network::ping_network,
      phishing::start_phishing,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

