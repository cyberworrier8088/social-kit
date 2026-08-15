// stackoverflow


use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StackOverFlowProfile {

    pub display_name: String,
    pub reputation: u32,
    pub user_id: u32,
    pub badge_counts: BadgeCounts,
    pub profile_image: String,
    pub location: String,
    pub link: String,
    pub created_date: String,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BadgeCounts {
    pub gold: u32,
    pub silver: u32,
    pub bronze: u32,
}

pub fn search_stackoverflow(username: &str) -> Result<StackOverFlowProfile, String> {
    let url = format!(
        "https://api.stackexchange.com/2.3/users?inname={}&site=stackoverflow",
        username
    );

    let client = Client::new();

    let response = client.get(&url).header("User-Agent", "SocialKit").send().map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("Stack OverFlow API Error".into());
    }

    let json: Value = response.json().map_err(|e| e.to_string())?;

    let items = &json["items"];

    if items.is_null() || items.as_array().unwrap_or(&vec![]).is_empty() {
        return Err("User not found on Stack Overflow".into());
    }


    let user = &items[0];

    Ok(StackOverFlowProfile {
        display_name: user["display_name"].as_str().unwrap_or("").to_string(),
        reputation: user["reputation"].as_u64().unwrap_or(0) as u32,
        user_id: user["user_id"].as_u64().unwrap_or(0) as u32,
        badge_counts: BadgeCounts {
            gold: user["badge_counts"]["gold"].as_u64().unwrap_or(0) as u32,
            silver: user["badge_counts"]["silver"].as_u64().unwrap_or(0) as u32,
            bronze: user["badge_counts"]["bronze"].as_u64().unwrap_or(0) as u32,
        },
        profile_image: user["profile_image"].as_str().unwrap_or("").to_string(),
        location: user["location"].as_str().unwrap_or("-").to_string(),
        link: user["link"].as_str().unwrap_or("").to_string(),
        created_date: user["creation_date"].as_u64().unwrap_or(0).to_string(),
    })
}