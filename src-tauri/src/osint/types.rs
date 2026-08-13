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


#[derive(Debug, Serialize, Deserialize)]
pub struct RedditProfile {
    
    pub name: String,
    
    pub icon_img: String,

    pub total_karma: u32,

    pub created_utc: f64,

    pub profile_url: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct InstagramProfile {

    pub username: String,

    pub full_name: String,

    pub biography: String,

    pub profile_pic_url: String,

    pub followers: u64,

    pub following: u64,

    pub verified: bool,

    pub profile_url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GitLabProfile {

    pub username: String,

    pub name: String,

    pub avatar_url: String,

    pub bio: String,

    pub web_url: String,

    pub location: String,

    pub organization: String,

    pub followers: u32,

    pub following: u32,
}

