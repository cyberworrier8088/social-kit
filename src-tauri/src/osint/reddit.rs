use reqwest::blocking::Client;

use super::types::RedditProfile;

pub fn search_riddit(
    username: &str,
) -> Result<RedditProfile, String> {

    let url = format!("https://www.reddit.com/user/{}/", username);

    let client = Client::new();

    let response = client.get(&url)
        .header("User-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .send()
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() && response.status().as_u16() != 403 {
        return Err(format!("Reddit return {}", response.status()));
    }

    let text = response.text().unwrap_or_default();

    // Check if the user does not exist
    if text.contains("nobody on Reddit goes by that name") || text.contains("page not found") {
        return Err("User not found".into());
    }

    // Since we are not parsing the JSON anymore, return the URL with default values
    Ok(RedditProfile {
        name: username.to_string(),
        icon_img: "https://www.redditstatic.com/avatars/defaults/v2/avatar_default_1.png".to_string(),
        total_karma: 0,
        created_utc: 0.0,
        profile_url: format!("https://www.reddit.com/user/{}", username),
    })
}