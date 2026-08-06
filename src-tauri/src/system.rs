// src/system.rs
// this for get system info
// for dashboard
// this is eductional purpose only
use serde::Serialize;
use std::sync::Mutex;
use sysinfo::System;

pub struct SystemState(pub Mutex<System>);

impl SystemState {
    pub fn new() -> Self {
        Self(Mutex::new(System::new_all()))
    }
}

#[derive(Serialize)]
pub struct CpuInfo {
    usage: f32,
    model: String,
}

#[derive(Serialize)]
pub struct MemoryInfo {
    used: u64,
    total: u64,
}

#[tauri::command]
pub fn get_cpu_info(state: tauri::State<SystemState>) -> CpuInfo {
    let mut sys = state.0.lock().unwrap();
    sys.refresh_cpu_all();
    
    let cpus = sys.cpus();
    let usage = sys.global_cpu_usage();
    let model = if let Some(cpu) = cpus.first() {
        cpu.brand().to_string()
    } else {
        "Unknown".to_string()
    };

    CpuInfo { usage, model }
}

#[tauri::command]
pub fn get_memory_info(state: tauri::State<SystemState>) -> MemoryInfo {
    let mut sys = state.0.lock().unwrap();
    sys.refresh_memory();

    MemoryInfo {
        used: sys.used_memory(),
        total: sys.total_memory(),
    }
}

#[derive(Serialize)]
pub struct DiskInfo {
    used: u64,
    total: u64,
}

#[tauri::command]
pub fn get_disk_info() -> DiskInfo {
    use sysinfo::Disks;
    let disks = Disks::new_with_refreshed_list();
    
    let mut used = 0;
    let mut total = 0;
    
    for disk in disks.list() {
        total += disk.total_space();
        used += disk.total_space().saturating_sub(disk.available_space());
    }
    
    DiskInfo { used, total }
}

#[derive(Serialize)]
pub struct NetworkInfo {
    download: u64,
    upload: u64,
    ip: String,
}

#[tauri::command]
pub fn get_network_info() -> NetworkInfo {
    use sysinfo::Networks;
    let mut networks = Networks::new_with_refreshed_list();
    
    std::thread::sleep(std::time::Duration::from_millis(200)); // Sleep slightly to get a delta for speeds
    networks.refresh(true);

    let mut download = 0;
    let mut upload = 0;
    let mut ip = String::from("N/A"); // For simplicity without extra crates, sysinfo doesn't easily provide local IP across all OSs natively in a uniform way without mac address filtering.
    
    // As per user rule: "Prefer the standard library whenever possible. No Node.js packages."
    // Actually, local IP isn't available directly in `sysinfo` Networks struct (it has MAC).
    // Let's just retrieve speeds.
    for (name, data) in networks.iter() {
        download += data.received();
        upload += data.transmitted();
        // Just grab the first non-loopback name as a placeholder for IP or interface if IP is too hard
        if ip == "N/A" && !name.contains("Loopback") {
            ip = name.clone();
        }
    }
    
    // It's better to fetch actual IP using standard UdpSocket hack:
    if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                ip = addr.ip().to_string();
            }
        }
    }

    NetworkInfo { download, upload, ip }
}

#[derive(Serialize)]
pub struct OsInfo {
    os_name: String,
    uptime: u64,
}

#[tauri::command]
pub fn get_os_info() -> OsInfo {
    OsInfo {
        os_name: sysinfo::System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string()),
        uptime: sysinfo::System::uptime(),
    }
}

#[tauri::command]
pub fn open_in_browser(url: String) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd").args(["/C", "start", "", &url]).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
    }
}