


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



function updateGitHubCard(profile) {
    document.getElementById("github-status").textContent = "Found!";

    document.getElementById("github-avatar").src = profile.avatar_url;

    document.getElementById("github-name").textContent = profile.name || "-";

    document.getElementById("github-username").textContent = "@" + profile.login;

    document.getElementById("github-bio").textContent = profile.bio || "No bio";

    document.getElementById("github-followers").textContent = profile.followers;

    document.getElementById("github-following").textContent = profile.following;

    document.getElementById("github-repos").textContent = profile.public_repos;

    document.getElementById("github-company").textContent = profile.company || "-";

    document.getElementById("github-location").textContent = profile.location || "-"

    document.getElementById("github-blog").textContent = profile.blog || "-";

    document.getElementById("github-created").textContent = profile.created_at;

    document.getElementById("github-profile").href = profile.html_url;


}

function setLoading(isLoading) {

    const button = document.getElementById("search-btn");

    if (!button) return;

    button.disabled = isLoading;

    button.textContent = isLoading
        ? "Searching..."
        : "Search";

}


function setStatus(message) {
    const status = document.getElementById("github-status");

    if (!status) return;

    status.textContent = message;
}



async function searchUsername() {

    setLoading("Searching ...");

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

        updateGitHubCard(result);

        setStatus("Profile Found");
    } catch (error) {

        console.error(error);

        setStatus("user not found");

        alert("user not found or network error")

    } finally {
        setLoading(false);
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