pub mod github;
pub mod types;
pub mod reddit;


use github::search_github;
use types::{
    UsernameSearchRequest,
    GithubProfile,
};

use reddit::search_riddit as fetch_reddit_profile;
use types::RedditProfile;

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