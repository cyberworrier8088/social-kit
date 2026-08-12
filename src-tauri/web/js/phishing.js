const { invoke } = window.__TAURI__.core;

const platform = document.getElementById("platform");

const generateButton = document.getElementById("genarte-demo");

const openButton = document.getElementById("open-demo");

const status = document.getElementById("server-status");

const urlElement = document.getElementById("demo-url");

const discordWebhook = document.getElementById("discord-webhook");


let demoUrl = null;

generateButton.addEventListener(
    "click",
    async () => {

        generateButton.disabled = true;

        status.textContent = "Creating web local server";

        try {

            const result = await invoke("start_phishing",
                {
                    platform: platform.value,
                    webhookUrl: discordWebhook.value
                }
            );

            demoUrl = result.url;

            urlElement.textContent = demoUrl;

            status.textContent = "Local demo run";

            openButton.disabled = false;
        } catch (error) {

            console.error(error);

            status.textContent = "Failed to start demo";

            urlElement.textContent = "-";

            openButton.disabled = true;
        } finally {

            generateButton.disabled = false;

        }
    }
);

openButton.addEventListener(
    "click",
    async () => {

        if (!demoUrl) {
            return;
        }

        await invoke(
            "open_in_browser",
            {
                url: demoUrl
            }
        );
    }
);