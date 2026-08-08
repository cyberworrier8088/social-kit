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

    alert(
        "IP Address: " + (result.ip || "Not Found")
    );


}

document.addEventListener(
    "click",
    (event) => {

        if (event.target.id === "analyze-btn") {

            analyzeTarget();
        }
    }
);