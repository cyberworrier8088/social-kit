use reqwest::blocking::Client;
use serde_json::Value;

use super::types::RedditProfile;

pub fn search_riddit(
    username: &str,
) -> Result<RedditProfile, String> {

    let url = format!("https://www.reddit.com/user/{}/about.json", username);

    let client = Client::new();

    let response = client.get(&url)
        .header("User-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .send()
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Reddit return {}", response.status()));
    }

    let json: Value = response.json().map_err(|e| e.to_string())?;
    let profile = &json["data"];

    if profile.is_null() {
        return Err("User not found".into());
    }

    Ok(RedditProfile {
        name: profile["name"].as_str().unwrap_or(username).to_string(),
        icon_img: profile["icon_img"].as_str().unwrap_or("").to_string(),
        total_karma: profile["total_karma"].as_u64().unwrap_or(0) as u32,
        created_utc: profile["created_utc"].as_f64().unwrap_or(0.0),
        profile_url: format!("https://www.reddit.com/user/{}", username),
    })
}
