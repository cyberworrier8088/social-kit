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


}

document.addEventListener(
    "click",
    (event) => {

        if (event.target.id === "analyze-btn") {

            analyzeTarget();
        }
    }
);