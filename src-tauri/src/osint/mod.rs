pub mod github;
pub mod types;


use github::search_github;
use types::{
    UsernameSearchRequest,
    GithubProfile,
};

#[tauri::command]
pub fn search(
    request: UsernameSearchRequest,
) -> Result<GithubProfile, String> {


    println!("OSINT Search callled; {}", request.username);
    
    search_github(request)

}