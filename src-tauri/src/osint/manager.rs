
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

    let keybase_thread = thread::spawn({
        let username = username.clone();
        move || {
            search_keybase(&username).ok()
        }
    });


    let devto_thread = thread::spawn({
        let username = username.clone();

        move || {
            search_devto(&username).ok()
        }
    });

    let instagram_thread = thread::spawn({
        let username = username.clone();

        move || {
            search_instagram(&username).ok()
        }
    });


    let stackoverflow_thread = thread::spawn({

        let username = username.clone();

        move || {
            search_stackoverflow(&username).ok()
        }
    });



    let github = github_thread.join().unwrap();

    let reddit = reddit_thread.join().unwrap();

    let instagram = instagram_thread.join().unwrap();

    let gitlab = search_gitlab(&request.username).ok();

    let mastodon = mastodon_thread.join().unwrap();
    
    let keybase = keybase_thread.join().unwrap();

    let devto = devto_thread.join().unwrap();


    let stackoverflow = stackoverflow_thread.join().unwrap();



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