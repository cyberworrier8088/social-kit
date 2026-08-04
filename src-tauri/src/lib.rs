use tauri::Manager;
mod system;
mod scanner;

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
      scanner::scan,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
