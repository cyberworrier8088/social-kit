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

        pingStatus.textContent = "ICMP reachable";


        pingStatus.setAttribute(
            "data-state",
            "found"
        );
    } else {

        pingValue.textContent = "ICMP blocked / timeout";

        pingStatus.textContent = "Host resolved";

        pingStatus.setAttribute(
            "data-state",
            "warning"
        );
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




    /// web fiesssss like robots.txt and sitemap.xml :D


    const webFiles = result.web_files;

    if (webFiles) {

        const robotsStatus = document.getElementById("robots-status");
        const robotsUrl = document.getElementById("robots-url");

        if (webFiles.robots_found) {

            robotsStatus.textContent = "Foundddddddd";
            robotsStatus.textContent = "found";

            robotsUrl.textContent = webFiles.robots_url || "";

        } else {

            robotsStatus.textContent = "Nooooooooooooooo found";

            robotsStatus.dataset.state = "not-found";

            robotsUrl.textContent = "";
        }


        const sitemapStatus = document.getElementById("sitemap-status");

        const sitemapUrl = document.getElementById("sitemap-url");


        if (webFiles.sitemap_found) {

            sitemapStatus.textContent = "Foundddddddder";

            sitemapStatus.dataset.state = "found";

            sitemapUrl.textContent = webFiles.sitemap_url || "";
        } else {

            sitemapStatus.textContent = "✗ Not found";
            sitemapStatus.dataset.state = "not-found";


            sitemapUrl.textContent = "";
        }
    }


    /// geo locationnn

    const geo = result.geo;

    if (geo) {

        document.getElementById("geo-country").textContent = geo.country || "-";

        document.getElementById("geo-region").textContent = geo.region || "-";

        document.getElementById("geo-city").textContent = geo.city || "-";

        document.getElementById("geo-latitude").textContent = geo.latitude || "-";

        document.getElementById("geo-longitude").textContent = geo.longitude || "-";

        document.getElementById("geo-timezone").textContent = geo.timezone || "-";

        document.getElementById("geo-isp").textContent = geo.isp || "-";

        document.getElementById("geo-organization").textContent = geo.organization || "-";

    }


    // hosting detailsss css

    const hosting = result.hosting;

    if (hosting) {

        document.getElementById("hosting-ip").textContent = hosting.ip || "-";

        document.getElementById("hosting-isp").textContent = hosting.isp || "-";

        document.getElementById("hosting-organization").textContent = hosting.organization || "-";

        document.getElementById("hosting-asn").textContent = hosting.asn || "-";

        document.getElementById("hosting-as-name").textContent = hosting.as_name || "-";
    }


    // Whois/ rdap

    const whois = result.whois;

    if (whois) {

        document.getElementById("whois-registrar").textContent = whois.registrar || "-";

        document.getElementById("whois-creation").textContent = whois.creation_date || "-";

        document.getElementById("whois-updated").textContent = whois.updated_date || "-";

        document.getElementById("whois-expiration").textContent = whois.expiration_date || "-";

        // domain age

        const ageElement = document.getElementById("domain-age");

        if (whois.creation_date) {

            const creation = new Date(whois.creation_date);

            if (!Number.isNaN(creation.getTime())) {

                const now = new Date();

                let years = now.getFullYear() - creation.getFullYear();

                let months = now.getMonth() - creation.getMonth();

                if (now.getDate() < creation.getDate()) {

                    months--;
                }

                if (months < 0) {

                    years--;
                    months += 12;

                }

                ageElement.textContent = '${years} years, ${months} months';

            } else {

                ageElement.textContent = "Unable to calculate";
            }
        } else {

            ageElement.textContent = "Unavailable";
        }


        // nammeeeservers

        const nameservers = whois.nameservers || [];

        document.getElementById("whois-nameservers").textContent = nameservers.length ? nameservers.join(", ") : "-";


        // Domain status

        const statuses = whois.domain_status || [];

        document.getElementById("whois-status").textContent = statuses.length ? statuses.join(", ") : "-";

    }


    // reverse DNS jssssss

    const reverseDns = result.reverse_dns;

    if (reverseDns) {

        document.getElementById("reverse-dns-ip").textContent = reverseDns.ip || "-";

        const hostnames = reverseDns.hostnames || [];


        document.getElementById("reverse-dns-hostnames").textContent = hostnames.length ? hostnames.join(", ") : "No PTR record";
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

            pingValue.textContent =
                "Online";

            pingStatus.textContent =
                "ICMP reachable";

            pingStatus.setAttribute(
                "data-state",
                "found"
            );

        } else {

            pingValue.textContent =
                "ICMP blocked / timeout";

            pingStatus.textContent =
                "Host resolved";

            pingStatus.setAttribute(
                "data-state",
                "warning"
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
    document.getElementById("target").disabled = true;

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
    document.getElementById("target").disabled = false;
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