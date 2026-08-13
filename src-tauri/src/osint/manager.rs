
use serde::Serialize;
use std::thread;


use super::{
    github::search_github,
    reddit::search_riddit,
    instagram::search_instagram,
    gitlab::search_gitlab,
    mastodon::{search_mastodon, MastodonProfile},
    types::{
        GithubProfile,
        RedditProfile,
        InstagramProfile,
        GitLabProfile,
        UsernameSearchRequest,
    },
};




#[derive(Serialize)]
pub struct SearchResults {
    pub github: Option<GithubProfile>,

    pub reddit: Option<RedditProfile>,

    pub instagram: Option<InstagramProfile>,

    pub gitlab: Option<GitLabProfile>,

    pub mastodon: Option<MastodonProfile>,
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

    let mastodon_thread = thread::spawn({

        let username = username.clone();
        move || {
            search_mastodon(&username).ok()
        }
    });



    let github = github_thread.join().unwrap();

    let reddit = reddit_thread.join().unwrap();

    let instagram = search_instagram(&request.username).ok();

    let gitlab = search_gitlab(&request.username).ok();

    let mastodon = mastodon_thread.join().unwrap();

    SearchResults {

        github,

        reddit,

        gitlab,
        
        instagram,

        mastodon,
    }
}