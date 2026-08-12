use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpInfoResult {

    pub url: String,

    pub status_code: u16,

    pub final_url: String,

    pub server: Option<String>,

    pub powered_by: Option<String>,

    pub content_type: Option<String>,

    pub content_length: Option<String>,

    pub technologies: Vec<String>,
}


pub async fn check_http_info(
    domain: &str,
) -> Result<HttpInfoResult, String> {

    let domain = domain.trim().trim_start_matches("https://").trim_start_matches("http://").trim_start_matches('/');

    let url = format!("https://{}", domain);


    let client = Client::builder().redirect(
        reqwest::redirect::Policy::limited(10)
    ).build().map_err(|error| error.to_string())?;


    let response = client.get(&url).header("User-Agent", "SocialKit/1.0").send().await.map_err(|error| error.to_string())?;

    let status_code = response.status().as_u16();

    let final_url = response.url().to_string();

    let server = response.headers().get("server").and_then(|value| value.to_str().ok()).map(String::from);

    let powered_by = response.headers().get("x-powered-by").and_then(|value| value.to_str().ok()).map(String::from);

    let content_type = response.headers().get("content-type").and_then(|value| value.to_str().ok()).map(String::from);

    let content_length = response.headers().get("content-length").and_then(|value| value.to_str().ok()).map(String::from);


    let mut technologies = Vec::new();

    if let Some(server_value) = &server {

        let lower = server_value.to_lowercase();

        if lower.contains("cloudflare") {

            technologies.push(
                "Cloudflare".to_string()
            );
        }

        if lower.contains("nginx") {

            technologies.push(
                "Nginx".to_string()
            );
        }

        if lower.contains("apache") {

            technologies.push(
                "Apache".to_string()
            );
        }

        if lower.contains("vercel") {

            technologies.push(
                "Vercel".to_string()
            );
        }

        if lower.contains("netlify") {

            technologies.push(
                "Netlify".to_string()
            );
        }
    }

    if let Some(powered_value) = &powered_by {

        let lower = powered_value.to_lowercase();

        if lower.contains("express") {

            technologies.push(
                "Expiress".to_string()
            );

        }

        if lower.contains("php") {

            technologies.push(
                "PHP".to_string()
            );
        }
    }

    // remove duplicatesssssss

    technologies.sort();
    technologies.dedup();

    Ok(HttpInfoResult {

        url,

        status_code,

        final_url,

        server,

        powered_by,

        content_type,

        content_length,

        technologies,
    })
}