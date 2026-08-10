const { invoke } = window.__TAURI__.core;

let monitoring = false;

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
        ipStatus.textContent = "found";
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

    // Populate DNS Records
    const dnsStatus = document.getElementById("dns-status");
    if (result.dns) {
        dnsStatus.textContent = "Resolved";
        dnsStatus.setAttribute("data-state", "found");

        const updateList = (id, items) => {
            const ul = document.getElementById(id);
            ul.innerHTML = "";
            if (items && items.length > 0) {
                items.forEach(item => {
                    const li = document.createElement("li");
                    li.textContent = item;
                    ul.appendChild(li);
                });
            } else {
                ul.innerHTML = "<li>None</li>";
            }
        };

        updateList("dns-a-list", result.dns.a);
        updateList("dns-aaaa-list", result.dns.aaaa);
        updateList("dns-mx-list", result.dns.mx);
        updateList("dns-txt-list", result.dns.txt);
        updateList("dns-ns-list", result.dns.ns);
        updateList("dns-cname-list", result.dns.cname);
    } else {
        dnsStatus.textContent = "No DNS Data";
        dnsStatus.setAttribute("data-state", "not-found");
    }


    const sslStatus = document.getElementById("ssl-status");

    if (result.ssl) {

        sslStatus.textContent = result.ssl.status;

        sslStatus.setAttribute(
            "data-state",
            "found"
        );

        document.getElementById("ssl-subject").textContent = result.ssl.subject;

        document.getElementById("ssl-issuer").textContent = result.ssl.issuer;

        document.getElementById("ssl-valid-from").textContent = result.ssl.valid_from;

        document.getElementById("ssl-expires").textContent = result.ssl.expires;

        document.getElementById("ssl-version").textContent = result.ssl.tls_version;
    } else {

        sslStatus.textContent = "No SSL data";

        sslStatus.setAttribute(
            "data-state",
            "not-found"
        );
    }


    // securty headerssss :0
    const securityHeaders = result.security_headers;

    if (securityHeaders) {

        setSecurityHeader(
            "header-hsts",
            securityHeaders.hsts
        );


        setSecurityHeader(
            "header-csp",
            securityHeaders.csp
        );

        setSecurityHeader(
            'header-frame',
            securityHeaders.x_frame_options
        );

        setSecurityHeader(
            "header-content-type",
            securityHeaders.x_content_type_options
        );

        setSecurityHeader(
            "header-referrer",
            securityHeaders.referrer_policy
        );

        setSecurityHeader(
            "header-permissions",
            securityHeaders.permissions_policy
        );

    }
}


async function livePing() {

    const target = document.getElementById("target").value.trim();

    if (target === "") {
        return;
    }

    try {

        const result = await invoke(
            "ping_network",
            {
                target
            }
        );

        console.log("Live ping: ", result);

        const pingValue = document.getElementById("ping-value");

        const pingStatus = document.getElementById("ping-status");

        if (result.online) {

            pingValue.textContent = "online";

            pingStatus.textContent = "Success";

            pingStatus.setAttribute(
                "data-state",
                "found"
            );


        } else {

            pingValue.textContent = "Offline";

            pingStatus.textContent = "Failed";

            pingStatus.setAttribute(
                "data-state",
                "not-found"
            );


        }

        document.getElementById("ping-packets").textContent = result.packets_received + " / " + result.packets_sent;

        document.getElementById("ping-loss").textContent = result.packet_loss.toFixed(1) + "%";

        document.getElementById("ping-min").textContent = result.min_latency !== null ? result.min_latency.toFixed(2) + " ms" : "-";

        document.getElementById("ping-average").textContent = result.average_latency !== null ? result.average_latency.toFixed(2) + " ms" : "-";

        document.getElementById("ping-max").textContent = result.max_latency !== null ? result.max_latency.toFixed(2) + " ms" : "-";
    } catch (error) {

        console.error(
            "Live ping failed:",
            error
        );
    }
}


async function startMonitoring() {

    if (monitoring) {
        return;
    }

    monitoring = true;

    updateLiveStatus();

    document.getElementById("analyze-btn").disabled = true;
    document.getElementById("stop-btn").disabled = false;

    await analyzeTarget();

    while (monitoring) {

        await livePing();

        updateLiveStatus();

        if (!monitoring) {

            break;

        }

        await new Promise(
            resolve => setTimeout(resolve, 2000)
        );
    }

}

function stopMonitoring() {

    monitoring = false;

    updateLiveStatus();

    document.getElementById("analyze-btn").disabled = false;

    document.getElementById("stop-btn").disabled = true;
}




function updateLiveStatus() {

    const dot = document.getElementById("live-dot");

    const text = document.getElementById("live-text");

    const lastCheck = document.getElementById("last-check");

    if (monitoring) {

        dot.classList.add("live");

        text.textContent = "LIVE";

        lastCheck.textContent = "Last Check: " + new Date().toLocaleTimeString();
    } else {

        dot.classList.remove("live");

        text.textContent = "Not Monitoring";

        lastCheck.textContent = "-";
    }
}

document.addEventListener(
    "click",
    (event) => {

        if (event.target.id === "analyze-btn") {

            startMonitoring();

        }

        if (event.target.id === "stop-btn") {

            stopMonitoring();

        }
    }
);


/// update securty header function
function setSecurityHeader(
    elementId,
    value
) {

    const element = document.getElementById(elementId);


    if (!element) {

        return;

    }


    if (value) {

        element.textContent = "Pressent"

        element.title = value;

        element.dataset.state = "found";
    } else {

        element.textContent = "Missing"

        element.title = ""

        element.dataset.state = "not-found";
    }
}