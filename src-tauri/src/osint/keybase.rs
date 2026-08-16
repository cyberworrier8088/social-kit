

use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct KeybaseProfile {

    pub username: String,
    pub full_name: String,
    pub bio: String,
    pub avatar_url: String,
    pub followers: u32,
    pub following: u32,
    pub profile_url: String,
    pub verified: bool,

}

pub fn search_keybase(username: &str) -> Result<KeybaseProfile, String> {

    let url = format!("https://keybase.io/_/api/1.0/user/lookup.json?usernames={}", username);

    let client = Client::new();

    let response = client.get(&url).header("User-Agent", "SocialKit").send().map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("user not FOund on Keybase".into());

    }

    let json: Value = response.json().map_err(|e| e.to_string())?;

    let profile = json["them"].as_array().and_then(|users| users.first()).ok_or("User not found on Keybase")?;


    Ok(KeybaseProfile {
        username: profile["basics"]["username"].as_str().unwrap_or("").to_string(),
        full_name: profile["profile"]["full_name"].as_str().unwrap_or("").to_string(),
        bio: profile["profile"]["bio"].as_str().unwrap_or("").to_string(),
        avatar_url: profile["pictures"]["primary"]["url"].as_str().unwrap_or("").to_string(),
        followers: 0,
        following: 0,
        profile_url: format!("https://keybase.io/{}", username),
        verified: profile["proofs_summary"]["all"].as_array().is_some_and(|proofs| !proofs.is_empty()),
    }) 
}
