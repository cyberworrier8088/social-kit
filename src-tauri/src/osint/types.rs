use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct UsernameSearchRequest {
    pub username: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct GithubProfile {
    
    pub login: String,

    pub name: Option<String>,

    pub avatar_url: String,

    pub bio: Option<String>,

    pub followers: u32,

    pub following: u32,

    pub public_repos: u32,

    pub company: Option<String>,

    pub location: Option<String>,

    pub blog: String,

    pub html_url: String,

    pub created_at: String,
}