const { invoke } = window.__TAURI__.core;

// Helper to update text if element exists and is loaded
function setText(id, text) {
    const el = document.getElementById(id);
    if (el) el.textContent = text;
}

// Convert bytes to GB
function toGb(bytes) {
    return (bytes / 1024 / 1024 / 1024).toFixed(2);
}

// Format bytes for network speed
function formatNetworkSpeed(bytes) {
    if (bytes > 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB/s';
    if (bytes > 1024) return (bytes / 1024).toFixed(1) + ' KB/s';
    return bytes + ' B/s';
}

async function updateCpu() {
    const info = await invoke("get_cpu_info");
    setText("cpu-card-value", `${info.usage.toFixed(1)}%`);
    // Optionally we could show info.model somewhere, but value is main focus
}

async function updateMemory() {
    const info = await invoke("get_memory_info");
    setText("memory-card-value", `${toGb(info.used)} GB / ${toGb(info.total)} GB`);
}

async function updateDisk() {
    const info = await invoke("get_disk_info");
    setText("disk-card-value", `${toGb(info.used)} GB / ${toGb(info.total)} GB`);
}

async function updateNetwork() {
    const info = await invoke("get_network_info");
    setText("network-card-value", `↓ ${formatNetworkSpeed(info.download)} | ↑ ${formatNetworkSpeed(info.upload)} | ${info.ip}`);
}

async function updateOsAndUptime() {
    const info = await invoke("get_os_info");
    setText("os-card-value", info.os_name);
    
    const hours = Math.floor(info.uptime / 3600);
    const minutes = Math.floor((info.uptime % 3600) / 60);
    setText("uptime-card-value", `${hours}h ${minutes}m`);
}

// Wait for load-components.js to inject elements, then do initial fetch
setTimeout(() => {
    updateCpu();
    updateMemory();
    updateDisk();
    updateNetwork();
    updateOsAndUptime();

    // Set intervals as requested: 
    // CPU: 1s, Network: 1s
    setInterval(updateCpu, 1000);
    setInterval(updateNetwork, 1000);
    
    // Memory: 2s
    setInterval(updateMemory, 2000);
    
    // Disk: 10s
    setInterval(updateDisk, 10000);
    
    // Uptime: 30s
    setInterval(updateOsAndUptime, 30000);
}, 200);