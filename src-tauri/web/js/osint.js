


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
        const ghProgress = document.getElementById("github-progress");
        const rdProgress = document.getElementById("reddit-progress");
        const igProgress = document.getElementById("instagram-progress");
        const glProgress = document.getElementById("gitlab-progress");
        const mtProgress = document.getElementById("mastodon-progress");
        const kbProgress = document.getElementById("keybase-progress");
        const dtProgress = document.getElementById("devto-progress");


        if (ghProgress) ghProgress.textContent = "Github: Searching...";
        if (rdProgress) rdProgress.textContent = "Reddit: Searching...";
        if (igProgress) igProgress.textContent = "Instagram: Searching...";
        if (glProgress) glProgress.textContent = "Gitlab: Searching...";
        if (mtProgress) mtProgress.textContent = "Mastodon: Searching...";
        if (kbProgress) kbProgress.textContent = "keybase: Searchinnggggg..";
        if (dtProgress) dtProgress.textContent = "dev.to: Searching..."

        const results = await invoke(
            "search_all_command",
            {
                request: {
                    username
                }
            }
        );

        console.log(results);

        if (ghProgress) ghProgress.textContent = results.github ? "Github: Found!" : "Github: Not Found";
        if (rdProgress) rdProgress.textContent = results.reddit ? "Reddit: Found!" : "Reddit: Not Found";
        if (igProgress) igProgress.textContent = results.instagram ? "Instagram: Found!" : "Instagram: Not Found";
        if (glProgress) glProgress.textContent = results.gitlab ? "Gitlab: Found!" : "Gitlab: Not Found";
        if (mtProgress) mtProgress.textContent = results.mastodon ? "Mastodon: Found!" : "Mastodon: Not Found";
        if (kbProgress) kbProgress.textContent = results.keybase ? "Keybase: Found!!!!!!!" : "Keybase: Not Found";

        if (results.github) {

            updateGitHubCard(results.github);

        }
        if (results.reddit) {

            updateRedditCard(results.reddit);
        }

        if (results.instagram) {

            updateInstagramCard(results.instagram);

        }

        if (results.gitlab) {

            updateGitLabCard(results.gitlab);
        }

        if (results.mastodon) {

            updateMastodonCard(results.mastodon);
        }

        if (results.keybase) {

            updateKeybaseCard(results.keybase);
        }

        if (results.devto) {

            updateDevtoCard(results.devto);
        }
    }


    catch (error) {
        console.error(error);
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

function updateRedditCard(profile) {
    document.getElementById("reddit-status").textContent = "FOUND!";

    document.getElementById("reddit-avatar").src = profile.icon_img;

    document.getElementById("reddit-name").textContent = profile.name;

    document.getElementById("reddit-karma").textContent = "Karma: " + profile.total_karma;

    document.getElementById("reddit-profile").href = profile.profile_url;


}



function updateGitLabCard(profile) {

    document.getElementById("gitlab-status").textContent = "Found";

    document.getElementById("gitlab-avatar").src = profile.avatar_url;

    document.getElementById("gitlab-name").textContent = profile.name || "-";

    document.getElementById("gitlab-username").textContent = "@" + profile.username;

    document.getElementById("gitlab-bio").textContent = profile.bio || "No bio";

    document.getElementById("gitlab-followers").textContent = profile.following;

    document.getElementById("gitlab-following").textContent = profile.following;

    document.getElementById("gitlab-location").textContent = profile.location || "-";

    document.getElementById("gitlab-organization").textContent = profile.organization || "-";

    document.getElementById("gitlab-profile").href = profile.web_url;
}

function updateMastodonCard(profile) {

    document.getElementById("mastodon-status").textContent = "Foundd";
    document.getElementById("mastodon-avatar").src = profile.avatar_url;
    document.getElementById("mastodon-name").textContent = profile.display_name || "-";
    document.getElementById("mastodon-username").textContent = "@" + profile.username;
    document.getElementById("mastodon-bio").textContent = profile.bio || "Noo BIooooo";
    document.getElementById("mastodon-followers").textContent = profile.followers_count;
    document.getElementById("mastodon-following").textContent = profile.following_count;
    document.getElementById("mastodon-posts").textContent = profile.statuses_count;
    document.getElementById("mastodon-profile").href = profile.profile_url;
}


function updateKeybaseCard(profile) {
    document.getElementById("keybase-status").textContent = "found";

    document.getElementById("keybase-avatar").src = profile.avatar_url;

    document.getElementById("keybase-name").textContent = profile.full_name || "-";

    document.getElementById("keybase-username").textContent = "@" + profile.username;

    document.getElementById("keybase-bio").textContent = profile.bio || "No Bio";

    document.getElementById("keybase-verified").textContent = profile.verified ? "yes" : "Nooo";

    document.getElementById("keybase-followers").textContent = profile.followers;

    document.getElementById("keybase-following").textContent = profile.following;

    document.getElementById("keybase-profile").href = profile.profile_url;

}


function updateDevtoCard(profile) {

    document.getElementById("devto-status").textContent = "FOund!";

    document.getElementById("devto-avatar").src = profile.avatar_url;

    document.getElementById("devto-name").textContent = profile.name || "-";

    document.getElementById("devto-username").textContent = "@" + profile.username;

    document.getElementById("devto-bio").textContent = profile.bio || 'No bio';

    document.getElementById("devto-followers").textContent = profile.followers;

    document.getElementById("devto-joined").textContent = profile.joined_date.split("T")[0];

    document.getElementById("devto-profile").href = profile.profile_url;
}



function updateInstagramCard(profile) {

    document.getElementById("instagram-status").textContent = "Found!";
    document.getElementById("instagram-avatar").src = profile.profile_pic_url;
    document.getElementById("instagram-name").textContent = profile.full_name || "-";

    document.getElementById("instagram-username").textContent = "@" + profile.username;

    document.getElementById("instagram-bio").textContent = profile.biography || "No Bio";

    document.getElementById("instagram-followers").textContent = profile.followers;

    document.getElementById("instagram-following").textContent = profile.following;

    document.getElementById("instagram-verified").textContent = profile.verified ? "Yes" : "No";

    document.getElementById("instagram-profile").href = profile.profile_url;

    document.getElementById("instagram-posts").textContent = profile.posts;
}

// end