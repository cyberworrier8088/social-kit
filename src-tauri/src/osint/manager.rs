
use serde::Serialize;
use std::thread;


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

    SearchResults {

        github,

        reddit,
    }
}