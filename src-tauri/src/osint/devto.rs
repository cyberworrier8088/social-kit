// src/devto.rs

use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DevtoProfile {

    pub username: String,
    pub name: String,
    pub bio: String,
    pub avatar_url: String,
    pub followers: u32,
    pub profile_url: String,
    pub joined_date: String,

}

pub fn search_devto(username: &str) -> Result<DevtoProfile, String> {

    let url = format!("https://dev.to/api/users/by_username?url={}", username);

    let client = Client::new();

    let response = client.get(&url).header("User-Agent", "SocialKit").send().map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("User not found on Dev.to".into());

    }

    let json: Value = response.json().map_err(|e| e.to_string())?;

    Ok(DevtoProfile {

        username: json["username"].as_str().unwrap_or("").to_string(),
        name: json["name"].as_str().unwrap_or("").to_string(),
        bio: json["summary"].as_str().unwrap_or("").to_string(),
        avatar_url: json["profile_image"].as_str().unwrap_or("").to_string(),
        followers: 0,
        profile_url: format!("https://dev.to/{}", username),
        joined_date: json["joined_at"].as_str().unwrap_or("").to_string(),
    })
}