use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct UsernameSearchRequest {
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct UsernameSearchResult {
    pub platform: String,
    pub found: bool,
    pub profile_url: String,
}