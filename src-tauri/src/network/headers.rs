use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityHeaders {

    pub hsts: Option<String>,

    pub csp: Option<String>,

    pub x_frame_options: Option<String>,

    pub x_content_type_options: Option<String>,

    pub referrer_policy: Option<String>,

    pub permissions_policy: Option<String>,
}


pub async fn check_headers(
    domain:&str,
) -> Result<SecurityHeaders, String> {

    let url = if domain.starts_with("http://") || domain.starts_with("https://") {
    
        domain.to_string()
    } else {
        format!("https://{}", domain)
    };

    let client = Client::builder().redirect(
        reqwest::redirect::Policy::limited(5)
    ).build().map_err(|error| error.to_string())?;


    let response = client.get(&url).header("User-Agent", "SocialKit/1.0").send().await.map_err(|error| error.to_string())?;

    let headers = response.headers();

    Ok(SecurityHeaders {
        hsts: headers.get("strict-transport-security").and_then(|v| v.to_str().ok()).map(String::from),

        csp: headers.get("content-security-policy").and_then(|v| v.to_str().ok()).map(String::from),

        x_frame_options: headers.get("x-frame-options").and_then(|v| v.to_str().ok()).map(String::from),

        x_content_type_options: headers.get("x-content-type-options").and_then(|v| v.to_str().ok()).map(String::from),

        referrer_policy: headers.get("referrer-policy").and_then(|v| v.to_str().ok()).map(String::from),

        permissions_policy: headers.get("permissions-policy").and_then(|v| v.to_str().ok()).map(String::from),

    })
}