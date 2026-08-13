

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

    let url = format!("https://keybase.io/api/1.0/user/lookup?username={}", username);

    let client = Client::new();

    let response = client.get(&url).header("User-Agent", "SocialKit").send().map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("user not FOund on Keybase".into());

    }

    let json: Value = response.json().map_err(|e| e.to_string())?;

    let profile = &json["profile"];

    let avatar_url = if let Some(pic) = profile.get("pictures") {

        if let Some(primary) = pic.get("primary") {

            primary.get("url").and_then(|u| u.as_str()).unwrap_or("").to_string()

        } else {
            "".to_string()
        }

    } else {
        "".to_string()
    };


    Ok(KeybaseProfile {
        username: profile["basics"]["username"].as_str().unwrap_or("").to_string(),
        full_name: profile["profile"]["full_name"].as_str().unwrap_or("").to_string(),
        bio: profile["profile"]["bio"].as_str().unwrap_or("").to_string(),
        avatar_url,
        followers: 0,
        following: 0,
        profile_url: format!("https://keybase.io/{}", username),
        verified: profile["id"]["is_deleted"].as_bool().unwrap_or(false) == false,
    }) 
}