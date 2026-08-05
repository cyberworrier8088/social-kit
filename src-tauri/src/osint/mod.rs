pub mod github;
pub mod types;


use github::search_github;
use types::{
    UsernameSearchRequest,
    UsernameSearchResult,
};

#[tauri::command]
pub fn search(
    request: UsernameSearchRequest,
) -> UsernameSearchResult {
    search_github(request)
}