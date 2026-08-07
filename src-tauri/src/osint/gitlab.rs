use reqwest::blocking::Client;
use serde_json::Value;

use super::types::GitLabProfile;

pub fn search_gitlab(
    username: &str,
) -> Result<GitLabProfile, String> {

    let url = format!(
        "https://gitlab.com/api/v4/users?username={}",
        username
    );

    let client = Client::new();


    let response = client.get(&url).header("User-Agent", "SocialKit").send().map_err(|e| e.to_string())?;


    if !response.status().is_success() {

        return Err(
            format!("GitLab returned {}", response.status())
        );
    }

    let users: Vec<Value> = response.json().map_err(|e| e.to_string())?;

    if users.is_empty() {

        return Err("User not found".into());

    }


    let user = &users[0];

    Ok(GitLabProfile {

        username: user["username"].as_str().unwrap_or("").to_string(),

        name: user["name"].as_str().unwrap_or("").to_string(),

        avatar_url: user["avatar_url"].as_str().unwrap_or("").to_string(),

        bio: user["bio"].as_str().unwrap_or("").to_string(),

        web_url: user["web_url"].as_str().unwrap_or("").to_string(),

        location: user["location"].as_str().unwrap_or("").to_string(),

        organization: user["organization"].as_str().unwrap_or("").to_string(),

        followers: 0,

        following: 0,
    })
}