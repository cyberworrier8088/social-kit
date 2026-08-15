pub mod github;
pub mod types;
pub mod reddit;
pub mod instagram;
pub mod gitlab;
pub mod mastodon;
pub mod keybase;
pub mod devto;
pub mod manager;



use github::search_github;
use types::{
    UsernameSearchRequest,
    GithubProfile,
};

use reddit::search_riddit as fetch_reddit_profile;
use types::RedditProfile;
use manager::{search_all, SearchResults};

#[tauri::command]
pub fn search_git(
    request: UsernameSearchRequest,
) -> Result<GithubProfile, String> {


    println!("OSINT Search callled; {}", request.username);
    
    search_github(request)

}


#[tauri::command]
pub fn search_riddit (
    username: String,
) -> Result<RedditProfile, String> {

    fetch_reddit_profile(&username)
}

#[tauri::command]
pub fn search_mastodon_command(
    username: String,
) -> Result<mastodon::MastodonProfile, String> {

    mastodon::search_mastodon(&username)
}

#[tauri::command]
pub fn search_keybase_command(
    username: String,
) -> Result<keybase::KeybaseProfile, String> {
    keybase::search_keybase(&username)
}


#[tauri::command]
pub fn search_devto_command(
    username: String,
) -> Result<devto::DevtoProfile, String> {

    devto::search_devto(&username)
}

#[tauri::command]
pub fn search_instagram_command(
    username: String,
) -> Result<instagram::InstagramProfile, String> {

    instagram::search_instagram(&username)
}


#[tauri::command]
pub fn search_all_command(
    request: UsernameSearchRequest,
) -> SearchResults {

    search_all(request)
}