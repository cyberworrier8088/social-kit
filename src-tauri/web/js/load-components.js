async function loadComponent(id, file) {
    const response = await fetch(file);
    const html = await response.text();
    document.getElementById(id).innerHTML = html;
}

async function loadStatusCard(id, title, description) {
    const response = await fetch("components/status-card.html");
    let html = await response.text();
    html = html.replace("{{TITLE}}", title)
               .replace("{{VALUE}}", `<span id="${id}-value">Loading...</span>`)
               .replace("{{DESCRIPTION}}", description);
    document.getElementById(id).innerHTML = html;
}

loadComponent("sidebar", "components/sidebar.html");
loadComponent("topbar", "components/topbar.html");
loadComponent("welcome-card", "components/welcome-card.html");

loadStatusCard("cpu-card", "CPU", "Processor Utilization");
loadStatusCard("memory-card", "Memory", "System RAM Usage");
loadStatusCard("disk-card", "Disk", "Total Storage Used");
loadStatusCard("network-card", "Network", "Up / Down / IP");
loadStatusCard("os-card", "System", "Operating System");
loadStatusCard("uptime-card", "Uptime", "Time since boot");