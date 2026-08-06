
use serde::Serialize;
use std::thread;


use super::{
    github::search_github,
    reddit::search_riddit,
    instagram::search_instagram,
    types::{
        GithubProfile,
        RedditProfile,
        InstagramProfile,
        UsernameSearchRequest,
    },
};




#[derive(Serialize)]
pub struct SearchResults {
    pub github: Option<GithubProfile>,

    pub reddit: Option<RedditProfile>,

    pub instagram: Option<InstagramProfile>,
}

pub fn search_all(
    request: UsernameSearchRequest,
) -> SearchResults {

    let username = request.username.clone();

    let github_thread = thread::spawn({

        let username = username.clone();

        move || {

            search_github(
                UsernameSearchRequest {
                    username,
                }
            ).ok()
        }
    });

    let reddit_thread = thread::spawn({

        let username = username.clone();

        move || {

            search_riddit(&username).ok()
        }
    });



    let github = github_thread.join().unwrap();

    let reddit = reddit_thread.join().unwrap();

    let instagram = search_instagram(&request.username).ok();

    SearchResults {

        github,

        reddit,

        instagram,
    }
}