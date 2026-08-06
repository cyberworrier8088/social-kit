async function loadComponent(id, file) {
    const el = document.getElementById(id);
    if (!el) return;
    const response = await fetch(file);
    const html = await response.text();
    el.innerHTML = html;
}

async function loadStatusCard(id, title, description) {
    const el = document.getElementById(id);
    if (!el) return;
    const response = await fetch("components/status-card.html");
    let html = await response.text();
    html = html.replace("{{TITLE}}", title)
        .replace("{{VALUE}}", `<span id="${id}-value">Loading...</span>`)
        .replace("{{DESCRIPTION}}", description);
    el.innerHTML = html;
}

loadComponent("sidebar", "components/sidebar.html");
loadComponent("topbar", "components/topbar.html");
loadComponent("welcome-card", "components/welcome-card.html");
loadComponent("scan-form", "components/scan-form.html");
loadComponent("progress-card", "components/progress-card.html");
loadComponent("results-table", "components/results-table.html");
loadComponent("username-form", "components/username-form.html");
loadComponent("github-card-container", "components/github-card.html");
loadComponent("osint-results", "components/osint-results.html");
loadComponent("reddit-card-container", "components/reddit-card.html");

loadStatusCard("cpu-card", "CPU", "Processor Utilization");
loadStatusCard("memory-card", "Memory", "System RAM Usage");
loadStatusCard("disk-card", "Disk", "Total Storage Used");
loadStatusCard("network-card", "Network", "Up / Down / IP");
loadStatusCard("os-card", "System", "Operating System");
loadStatusCard("uptime-card", "Uptime", "Time since boot");

document.addEventListener("click", async (event) => {
    const link = event.target.closest("a");
    if (link && link.target === "_blank") {
        event.preventDefault();

        const rawHref = link.getAttribute("href");
        if (!rawHref || rawHref === "#") {
            alert("No profile URL available to copy yet!");
            return;
        }

        try {
            await navigator.clipboard.writeText(link.href);
            alert("URL copied to clipboard!");
        } catch (err) {
            console.error("Failed to copy URL: ", err);
            alert("Failed to copy URL: " + link.href);
        }
    }
});