


const { invoke } = window.__TAURI__.core;

function clearResults() {
    document.getElementById("results-body").innerHTML = "";

}

function addResult(result) {
    const body = document.getElementById("results-body");

    const row = document.createElement("tr");

    row.innerHTML = `
        <td>${result.platform}</td>
        <td>${result.found ? "Found" : "Not Found"}</td>
        <td>
            ${result.found
            ? `<a href="${result.profile_url}" target="_blank">${result.profile_url}</a>`
            : "-"
        }
        </td>
    `;

    body.appendChild(row);
}


async function searchUsername() {
    console.log("Search button clicked");
    clearResults();

    const username = document.getElementById("username").value.trim();

    if (username === "") {
        alert("Enter a Username. ");

        return;
    }

    try {
        const result = await invoke("search", {
            request: {
                username
            }
        });

        console.log(result);

        addResult(result);
    } catch (error) {
        console.error(error);

        alert("Search failed");


    }
}


function clearForm() {
    document.getElementById("username").value = "";

    clearResults();
}


document.addEventListener("click", (event) => {
    if (event.target.id === "search-btn") {
        searchUsername();
    }

    if (event.target.id === "clear-btn") {
        clearForm();
    }
});