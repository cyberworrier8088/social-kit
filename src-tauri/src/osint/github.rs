use reqwest::blocking::Client;

use super::types::{
    GithubProfile,
    UsernameSearchRequest,
};

pub fn search_github(
    request: UsernameSearchRequest,
) -> Result<GithubProfile, String> {

    println!("1. Starting GitHub search");

    let url = format!(
        "https://api.github.com/users/{}",
        request.username
    );


    println!("2. URL: {}", url);

    let client = Client::new();

    let response = match client.get(&url).header("User-Agent", "SocialKit").send() {
        Ok(response) => {
            println!("3. Received response");
            response
        },

        Err(error) => {
            println!("Github Request Error: {:?}", error);

            return Err(error.to_string());
        }
    };

    if !response.status().is_success() {
        return Err(format!(
            "Github User {}",
            response.status()
        ));
    }

    println!("4. Status: {}", response.status());

    let profile = match response.json::<GithubProfile>() {

        Ok(profile) => profile,

        Err(error) => {
            println!("Json Error: {:?}", error);

            return Err(error.to_string());
        }
    };

    println!("6. Finished");

    Ok(profile)

}