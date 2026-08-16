// instgram

use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InstagramProfile {
    pub username: String,
    pub full_name: String,
    pub biography: String,
    pub profile_pic_url: String,
    pub followers: u64,
    pub following: u64,
    pub verified: bool,
    pub profile_url: String,
    pub posts: u64,
}

pub fn search_instagram(username: &str) -> Result<InstagramProfile, String> {

    let url = format!("https://www.instagram.com/api/v1/users/web_profile_info/?username={}", username);

    let client = Client::builder().timeout(std::time::Duration::from_secs(10)).build().map_err(|e| e.to_string())?;

    let response = client.get(&url).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36").header("X-IG-App-ID", "936619743392459").header("Accept", "application/json").send().map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Instagram API error: {}", response.status()));
    }

    let json: Value = response.json().map_err(|e| e.to_string())?;

    let user_data = &json["data"]["user"];

    if user_data.is_null() {
        return Err("User not found or API structure changed".into());
    }

    Ok(InstagramProfile {
        username: user_data["username"].as_str().unwrap_or("").to_string(),
        full_name: user_data["full_name"].as_str().unwrap_or("Unknown").to_string(),
        biography: user_data["biography"].as_str().unwrap_or("").to_string(),
        profile_pic_url: user_data["profile_pic_url"].as_str().unwrap_or("").to_string(),
        followers: user_data["edge_followed_by"]["count"].as_u64().unwrap_or(0),
        following: user_data["edge_follow"]["count"].as_u64().unwrap_or(0),
        verified: user_data["is_verified"].as_bool().unwrap_or(false),
        profile_url: format!("https://www.instagram.com/{}/", username),
        posts: user_data["edge_owner_to_timeline_media"]["count"].as_u64().unwrap_or(0),
    })
}
