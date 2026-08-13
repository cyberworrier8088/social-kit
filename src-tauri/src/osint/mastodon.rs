// src-tauri/src/osint/mastodon.rs


use reqwest::blocking::Client;
use serde_json::{Value};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MastodonProfile {

    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub profile_url: String,
}


pub fn search_mastodon(username: &str) -> Result<MastodonProfile, String> {

    let url = format!("https://mastodon.social/api/v1/accounts/lookup?acct={}", username);

    let client = Client::new();

    let response = client.get(&url).header("User-Agent", "SocialKit").send().map_err(|e| e.to_string())?;


    if !response.status().is_success() {

        return Err("User not found on Nastodon".into());
    }

    let json: Value = response.json().map_err(|e| e.to_string())?;


    Ok(MastodonProfile {

        username: json["username"].as_str().unwrap_or("").to_string(),
        display_name: json["display_name"].as_str().unwrap_or("").to_string(),
        avatar_url: json["avatar"].as_str().unwrap_or("").to_string(),
        bio: json["note"].as_str().unwrap_or("").to_string(),
        followers_count: json["followers_count"].as_u64().unwrap_or(0) as u32,
        following_count: json["following_count"].as_u64().unwrap_or(0) as u32,
        statuses_count: json["statuses_count"].as_u64().unwrap_or(0) as u32,
        profile_url: json["url"].as_str().unwrap_or("").to_string(),
    })
}