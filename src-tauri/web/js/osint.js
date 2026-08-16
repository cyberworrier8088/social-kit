


const { invoke } = window.__TAURI__.core;
const platforms = ["github", "reddit", "instagram", "gitlab", "mastodon", "keybase", "devto", "stackoverflow"];

function clearResults() {
    const body = document.getElementById("results-body");
    if (body) body.innerHTML = "";
}

function addResult(platform, profile) {
    const body = document.getElementById("results-body");
    if (!body) return;

    const row = document.createElement("tr");
    const url = profile?.html_url || profile?.profile_url || profile?.web_url || profile?.link;

    for (const value of [platform, profile ? "Found" : "Not found"]) {
        const cell = document.createElement("td");
        cell.textContent = value;
        row.appendChild(cell);
    }

    const linkCell = document.createElement("td");
    if (url) {
        const link = document.createElement("a");
        link.href = url;
        link.target = "_blank";
        link.textContent = url;
        linkCell.appendChild(link);
    } else {
        linkCell.textContent = "-";
    }
    row.appendChild(linkCell);
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
    const username = document.getElementById("username").value.trim().toLowerCase();
    const platform = document.getElementById("platform").value;

    if (!/^[a-z0-9][a-z0-9._-]*$/.test(username)) {
        alert("Enter a username using letters, numbers, hyphens, underscores, or periods.");
        return;
    }

    const updates = {
        github: updateGitHubCard,
        reddit: updateRedditCard,
        instagram: updateInstagramCard,
        gitlab: updateGitLabCard,
        mastodon: updateMastodonCard,
        keybase: updateKeybaseCard,
        devto: updateDevtoCard,
        stackoverflow: updateStackOverflowCard,
    };

    setLoading(true);
    clearResults();

    for (const name of platforms) {
        const selected = platform === "all" || platform === name;
        const card = document.getElementById(`${name}-card`) || document.getElementById(name);
        const progress = document.getElementById(`${name}-progress`);
        if (card) card.hidden = !selected;
        if (progress) progress.textContent = `${name}: ${selected ? "Searching..." : "Not selected"}`;
    }

    try {
        const results = await invoke("search_all_command", { request: { username, platform } });

        for (const name of platforms) {
            if (platform !== "all" && platform !== name) continue;

            const profile = results[name];
            const card = document.getElementById(`${name}-card`) || document.getElementById(name);
            const status = document.getElementById(`${name}-status`);
            const progress = document.getElementById(`${name}-progress`);

            if (card) card.hidden = !profile;
            if (status) status.textContent = profile ? "Found" : "Not found";
            if (progress) progress.textContent = `${name}: ${profile ? "Found" : "Not found"}`;
            addResult(name, profile);
            if (profile) updates[name](profile);
        }
    } catch (error) {
        console.error(error);
        setStatus("Search failed");
        alert(`Search failed: ${error}`);
    } finally {
        setLoading(false);
    }
}


function clearForm() {
    document.getElementById("username").value = "";

    clearResults();

    for (const name of platforms) {
        const card = document.getElementById(`${name}-card`) || document.getElementById(name);
        if (card) card.hidden = true;
    }
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

    document.getElementById("gitlab-followers").textContent = profile.followers;

    document.getElementById("gitlab-following").textContent = profile.following;

    document.getElementById("gitlab-location").textContent = profile.location || "-";

    document.getElementById("gitlab-organization").textContent = profile.organization || "-";

    document.getElementById("gitlab-profile").href = profile.web_url;
}

function updateMastodonCard(profile) {

    document.getElementById("mastodon-status").textContent = "Found";
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
    document.getElementById("keybase-status").textContent = "Found";

    document.getElementById("keybase-avatar").src = profile.avatar_url;

    document.getElementById("keybase-name").textContent = profile.full_name || "-";

    document.getElementById("keybase-username").textContent = "@" + profile.username;

    document.getElementById("keybase-bio").textContent = profile.bio || "No Bio";

    document.getElementById("keybase-verified").textContent = profile.verified ? "Yes" : "No";

    document.getElementById("keybase-followers").textContent = profile.followers;

    document.getElementById("keybase-following").textContent = profile.following;

    document.getElementById("keybase-profile").href = profile.profile_url;

}


function updateDevtoCard(profile) {

    document.getElementById("devto-status").textContent = "Found";

    document.getElementById("devto-avatar").src = profile.avatar_url;

    document.getElementById("devto-name").textContent = profile.name || "-";

    document.getElementById("devto-username").textContent = "@" + profile.username;

    document.getElementById("devto-bio").textContent = profile.bio || 'No bio';

    document.getElementById("devto-followers").textContent = profile.followers;

    document.getElementById("devto-joined").textContent = profile.joined_date.split("T")[0] || "-";

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


function updateStackOverflowCard(profile) {
    document.getElementById("stackoverflow-status").textContent = "Found";

    document.getElementById("stackoverflow-avatar").src = profile.profile_image;

    document.getElementById("stackoverflow-name").textContent = profile.display_name || "Not found";

    document.getElementById("stackoverflow-location").textContent = profile.location || "Not specified";

    document.getElementById("stackoverflow-reputation").textContent = "Reputation: " + profile.reputation;

    document.getElementById("stackoverflow-gold").textContent = profile.badge_counts.gold;

    document.getElementById("stackoverflow-silver").textContent = profile.badge_counts.silver;

    document.getElementById("stackoverflow-bronze").textContent = profile.badge_counts.bronze;

    document.getElementById("stackoverflow-profile").href = profile.link;


}
// end
