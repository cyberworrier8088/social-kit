use reqwest::blocking::Client;

use super::types::{
    UsernameSearchRequest,
    UsernameSearchResult,
};

pub fn search_github(
    request: UsernameSearchRequest,
) -> UsernameSearchResult {

    let url = format!("https://github.com/{}", request.username);

    let client = Client::new();

    let found = client.head(&url).send().map(|response| response.status().is_success()).unwrap_or(false);

    UsernameSearchResult {
        platform: "GitHub".to_string(),
        found,

        profile_url: url,
    }
}

