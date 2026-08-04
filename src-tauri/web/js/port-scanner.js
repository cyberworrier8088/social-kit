const { invoke } = window.__TAURI__.core;

let isScanning = false;
let startTime = null;
let timerInterval = null;

function updateTimer() {
    if (!startTime) return;
    const now = Date.now();
    const diff = new Date(now - startTime);
    const timeString = diff.toISOString().substring(11, 19);
    document.getElementById("elapsed-time").innerText = timeString;
}

function addResult(port, status, service) {
    const body = document.getElementById("results-body");

    const row = document.createElement("tr");

    row.innerHTML = `
    <td>${port}</td>
    <td>${status}</td>
    <td>${service}</td>`;

    body.appendChild(row);
}

function clearResults() {

    document.getElementById("results-body").innerHTML = "";

}
async function startScan() {
    if (isScanning) return;
    clearResults();

    const target = document.getElementById("target").value || "127.0.0.1";
    const startPort = Number(document.getElementById("start-port").value) || 1;
    const endPort = Number(document.getElementById("end-port").value) || 1024;
    const timeout = Number(document.getElementById("timeout").value) || 1000;

    isScanning = true;
    startTime = Date.now();
    timerInterval = setInterval(updateTimer, 1000);

    const threads = Number(document.getElementById("threads")?.value) || 50;

    document.getElementById("progress-target").innerText = target;
    document.getElementById("progress-status").innerText = "Scanning...";

    const totalPorts = endPort - startPort + 1;
    const progressBar = document.getElementById("scan-progress");
    progressBar.max = totalPorts;
    progressBar.value = 0;

    document.getElementById("progress-percent").innerText = "0%";

    let openPorts = [];
    let currentPort = startPort;
    let completed = 0;

    async function scanWorker() {
        while (currentPort <= endPort && isScanning) {
            const port = currentPort++;
            document.getElementById("current-port").innerText = port;

            try {
                const result = await invoke("scan", {
                    request: {
                        target,
                        port,
                        timeout: timeout
                    }
                });

                if (result.open) {
                    openPorts.push(port);
                    addResult(port, "Open", "Unknown");
                    console.log(`Port ${port} is OPEN!`);
                }
            } catch (error) {
                console.error(`Error scanning port ${port}:`, error);
            }

            completed++;
            progressBar.value = completed;
            document.getElementById("progress-percent").innerText = Math.round((completed / totalPorts) * 100) + "%";
        }
    }

    const workers = [];
    for (let i = 0; i < threads; i++) {
        workers.push(scanWorker());
    }

    await Promise.all(workers);

    if (isScanning) {
        document.getElementById("progress-status").innerText = "Completed. Open ports: " + openPorts.join(", ");
    }

    isScanning = false;
    clearInterval(timerInterval);
}

function stopScan() {
    isScanning = false;
}

// wait for load-components.js to insert the HTML, then bind events
setTimeout(() => {
    const startBtn = document.getElementById("start-scan");
    const stopBtn = document.getElementById("stop-scan");
    if (startBtn) startBtn.addEventListener("click", startScan);
    if (stopBtn) stopBtn.addEventListener("click", stopScan);

    const profileSel = document.getElementById("profile");
    if (profileSel) {
        profileSel.addEventListener("change", (e) => {
            const start = document.getElementById("start-port");
            const end = document.getElementById("end-port");
            switch (e.target.value) {
                case "quick": start.value = 1; end.value = 1024; break;
                case "top100": start.value = 1; end.value = 100; break;
                case "top1000": start.value = 1; end.value = 1000; break;
                case "full": start.value = 1; end.value = 65535; break;
            }
        });
    }
}, 200);