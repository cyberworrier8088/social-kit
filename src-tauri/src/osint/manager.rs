
use serde::Serialize;


use super::{
    github::search_github,
    reddit::search_riddit,
    types::{
        GithubProfile,
        RedditProfile,
        UsernameSearchRequest,
    },
};




#[derive(Serialize)]
pub struct SearchResults {
    pub github: Option<GithubProfile>,

    pub reddit: Option<RedditProfile>,
}

pub fn search_all(
    request: UsernameSearchRequest,
) -> SearchResults {

    let github = search_github(
        UsernameSearchRequest {
            username: request.username.clone(),
        }
    ).ok();


    let reddit = search_riddit(
        &request.username
    ).ok();

    SearchResults {

        github,

        reddit,
    }
}