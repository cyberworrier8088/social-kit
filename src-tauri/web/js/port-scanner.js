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

async function startScan() {
    if (isScanning) return;
    
    const target = document.getElementById("target").value || "127.0.0.1";
    const startPort = Number(document.getElementById("start-port").value) || 1;
    const endPort = Number(document.getElementById("end-port").value) || 1024;
    const timeout = Number(document.getElementById("timeout").value) || 1000;
    
    if (startPort > endPort) {
        alert("Start port must be less than or equal to end port");
        return;
    }

    isScanning = true;
    startTime = Date.now();
    timerInterval = setInterval(updateTimer, 1000);
    
    document.getElementById("progress-target").innerText = target;
    document.getElementById("progress-status").innerText = "Scanning...";
    
    const totalPorts = endPort - startPort + 1;
    const progressBar = document.getElementById("scan-progress");
    progressBar.max = totalPorts;
    progressBar.value = 0;
    
    document.getElementById("progress-percent").innerText = "0%";

    let openPorts = [];

    for (let port = startPort; port <= endPort; port++) {
        if (!isScanning) {
            document.getElementById("progress-status").innerText = "Stopped";
            break;
        }

        document.getElementById("current-port").innerText = port;
        
        try {
            // Note: timeout here is passed in ms, but Rust code expects secs, so we'll convert or just pass ms and handle in rust.
            // Wait, the Rust code: Duration::from_secs(request.timeout) expects seconds. So if the dropdown has 1000, 1000s is too long!
            // Let's pass timeout / 1000 to Rust, but ensure at least 1s.
            const timeoutSecs = Math.max(1, Math.round(timeout / 1000));
            
            const result = await invoke("scan", {
                request: {
                    target,
                    port,
                    timeout: timeoutSecs
                }
            });

            if (result.open) {
                openPorts.push(port);
                console.log(`Port ${port} is OPEN!`);
            }
        } catch (error) {
            console.error(`Error scanning port ${port}:`, error);
        }

        // Update progress
        const completed = port - startPort + 1;
        progressBar.value = completed;
        document.getElementById("progress-percent").innerText = Math.round((completed / totalPorts) * 100) + "%";
    }

    if (isScanning) {
        document.getElementById("progress-status").innerText = "Completed. Open ports: " + openPorts.join(", ");
    }
    
    isScanning = false;
    clearInterval(timerInterval);
}

function stopScan() {
    isScanning = false;
}

// Wait for load-components.js to insert the HTML, then bind events
setTimeout(() => {
    const startBtn = document.getElementById("start-scan");
    const stopBtn = document.getElementById("stop-scan");
    if (startBtn) startBtn.addEventListener("click", startScan);
    if (stopBtn) stopBtn.addEventListener("click", stopScan);
}, 200);