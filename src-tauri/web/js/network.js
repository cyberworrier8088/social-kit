const { invoke } = window.__TAURI__.core;

async function analyzeTarget() {

    const target = document.getElementById("target").value.trim();


    if (target === "") {

        alert("Enter a domain or IP");

        return;
    }


    const result = await invoke(
        "analyze_network",
        {
            request: {
                target
            }
        }
    );

    console.log(result);


    const ipValue = document.getElementById("ip-value");

    const ipStatus = document.getElementById("ip-status");

    if (result.ip) {

        ipValue.textContent = result.ip;
        ipStatus.textContent = "Found";
        ipStatus.setAttribute("data-state", "found");
    } else {

        ipValue.textContent = "-";
        ipStatus.textContent = "Not Found";
        ipStatus.setAttribute("data-state", "not-found");
    }

    const pingValue = document.getElementById("ping-value");
    const pingStatus = document.getElementById("ping-status");

    if (result.online) {
        pingValue.textContent = "Online";
        pingStatus.textContent = "Success";
        pingStatus.setAttribute("data-state", "found");
    } else {
        pingValue.textContent = "Offline";
        pingStatus.textContent = "Failed";
        pingStatus.setAttribute("data-state", "not-found");
    }


    const pingPackets = document.getElementById("ping-packets");

    const pingLoss = document.getElementById("ping-loss");

    const pingMin = document.getElementById("ping-min");

    const pingAverage = document.getElementById("ping-average");

    const pingMax = document.getElementById("ping-max");

    pingPackets.textContent = result.packets_received + " / " + result.packets_sent;

    pingLoss.textContent = result.packet_loss.toFixed(1) + "%";

    pingMin.textContent = result.min_latency !== null ? result.min_latency.toFixed(2) + " ms" : "-";

    pingAverage.textContent = result.average_latency !== null ? result.average_latency.toFixed(2) + " ms" : "-";

    pingMax.textContent = result.max_latency !== null ? result.max_latency.toFixed(2) + " ms" : "-";



}

document.addEventListener(
    "click",
    (event) => {

        if (event.target.id === "analyze-btn") {

            analyzeTarget();
        }
    }
);