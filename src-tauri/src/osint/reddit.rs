use reqwest::blocking::Client;

use super::types::RedditProfile;

pub fn search_riddit(
    username: &str,
) -> Result<RedditProfile, String> {

    let url = format!("https://www.reddit.com/user/{}/about.json", username);

    let client = Client::new();

    let response = client.get(&url).header("User-agent", "SocialKit/1.0").send().map_err(|e| e.to_string())?;

    if response.status().is_success() {
        return Err(format!("Reddit return {}", response.status()));
    }


    let json: serde_json::Value = response.json().map_err(|e| e.to_string())?;

    let data = &json["data"];

    Ok(RedditProfile {
        name: data["name"].as_str().unwrap_or("").to_string(),

        icon_img: data["icon_img"].as_str().unwrap_or("").to_string(),

        total_karma: data["total_karma"].as_u64().unwrap_or(0) as u32,

        created_utc: data["created_utc"].as_f64().unwrap_or(0.0),

        profile_url: format!(
            "https://reddit.com/u/{}",
            username
        ),
    })
}