use reqwest::Client;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct WebFileResult {

    pub robots_found: bool,

    pub robots_url: Option<String>,

    pub sitemap_found: bool,

    pub sitemap_url: Option<String>,
}

pub async fn check_web_files(
    domain: &str,
) -> Result<WebFileResult, String> {

    let domain = domain.trim().trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');


    let client = Client::builder().redirect(
        reqwest::redirect::Policy::limited(5)
    ).build().map_err(|error| error.to_string())?;

    let robots_url = format!("https://{}/robots.txt", domain);

    let robots_response = client.get(&robots_url).header("User-Agent", "SocialKit/1.0").send().await;

    let robots_found = matches!(
        robots_response,
        Ok(response) if response.status().is_success()
    );



    let sitemap_url = format!("https://{}/sitemap.xml", domain);

    let sitemap_response = client.get(&sitemap_url).header("User-Agent", "SocialKit/1.0").send().await;


    let sitemap_found = matches!(
        sitemap_response,
        Ok(response) if response.status().is_success()
    );

    Ok(WebFileResult {
        
        robots_found,

        robots_url: if robots_found {
            Some(robots_url)
        } else {
            None
        },

        sitemap_found,

        sitemap_url: if sitemap_found {
            Some(sitemap_url)
        } else {
            None
        },
    })
}