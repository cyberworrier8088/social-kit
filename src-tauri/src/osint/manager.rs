
use serde::Serialize;
use std::thread;


use super::{
    github::search_github,
    reddit::search_riddit,
    instagram::{search_instagram, InstagramProfile},
    gitlab::search_gitlab,
    keybase::{search_keybase, KeybaseProfile},
    mastodon::{search_mastodon, MastodonProfile},
    devto::{search_devto, DevtoProfile},
    stackoverflow::{search_stackoverflow, StackOverFlowProfile},
    types::{
        GithubProfile,
        RedditProfile,
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

    pub keybase: Option<KeybaseProfile>,

    pub devto: Option<DevtoProfile>,

    pub stackoverflow: Option<StackOverFlowProfile>,
}

pub fn search_all(
    request: UsernameSearchRequest,
) -> SearchResults {

    let username = request.username;
    let platform = request.platform.unwrap_or_else(|| "all".into()).trim().to_ascii_lowercase();
    let selected = |name: &str| platform == "all" || platform == name;

    let github_thread = selected("github").then(|| thread::spawn({

        let username = username.clone();

        move || {

            search_github(
                UsernameSearchRequest {
                    username,
                    platform: None,
                }
            ).ok()
        }
    }));

    let reddit_thread = selected("reddit").then(|| thread::spawn({

        let username = username.clone();

        move || {

            search_riddit(&username).ok()
        }
    }));

    let mastodon_thread = selected("mastodon").then(|| thread::spawn({

        let username = username.clone();
        move || {
            search_mastodon(&username).ok()
        }
    }));

    let keybase_thread = selected("keybase").then(|| thread::spawn({
        let username = username.clone();
        move || {
            search_keybase(&username).ok()
        }
    }));


    let devto_thread = selected("devto").then(|| thread::spawn({
        let username = username.clone();

        move || {
            search_devto(&username).ok()
        }
    }));

    let instagram_thread = selected("instagram").then(|| thread::spawn({
        let username = username.clone();

        move || {
            search_instagram(&username).ok()
        }
    }));


    let stackoverflow_thread = selected("stackoverflow").then(|| thread::spawn({

        let username = username.clone();

        move || {
            search_stackoverflow(&username).ok()
        }
    }));



    let gitlab_thread = selected("gitlab").then(|| thread::spawn({
        let username = username.clone();
        move || search_gitlab(&username).ok()
    }));

    let github = github_thread.and_then(|thread| thread.join().ok().flatten());

    let reddit = reddit_thread.and_then(|thread| thread.join().ok().flatten());

    let instagram = instagram_thread.and_then(|thread| thread.join().ok().flatten());

    let gitlab = gitlab_thread.and_then(|thread| thread.join().ok().flatten());

    let mastodon = mastodon_thread.and_then(|thread| thread.join().ok().flatten());
    
    let keybase = keybase_thread.and_then(|thread| thread.join().ok().flatten());

    let devto = devto_thread.and_then(|thread| thread.join().ok().flatten());


    let stackoverflow = stackoverflow_thread.and_then(|thread| thread.join().ok().flatten());



    SearchResults {

        github,

        reddit,

        gitlab,
        
        instagram,

        mastodon,
        
        keybase,

        devto,

        stackoverflow,
        
    }
}
