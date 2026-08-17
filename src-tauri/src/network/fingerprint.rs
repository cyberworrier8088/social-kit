
// this module not working. pls wait
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FingerprintResult {

    pub technologies: Vec<Technology>,
    pub confidence: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Technology {
    pub name: String,
    pub category: String,
    pub version: Option<String>,
    pub confidence: f64,
}


pub async fn fingerprint_technologies(
    domain: &str,
) -> Result<FingerprintResult, String> {
    let domain = domain.trim().trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');

    let url = format!("https://{}", domain);

    let client = Client::builder().redirect(reqwest::redirect::Policy::limited(5)).build().map_err(|e| e.to_string())?;

    let response = client.get(&url).header("User-Agent", "SocialKit/1.0").send().await.map_err(|e| e.to_string())?;

    let headers = response.headers();
    let body = response.text().await.map_err(|e| e.to_string())?;

    let mut technologies: Vec<Technology> = Vec::new();


    if let Some(server) = headers.get("server") {
        if let Ok(server_str) = server.to_str() {

            let lower = server_str.to_lowercase();

            if lower.contains("apache") {
                technologies.push(Technology {
                    name: "Apache".to_string(),
                    category: "Web Server".to_string(),
                    version: extract_version(&lower, "apache"),
                    confidence: 0.95,
                });
            }
            if lower.contains("nginx") {

                technologies.push(Technology {
                    name: "Nginx".to_string(),
                    category: "Web Server".to_string(),
                    version: extract_version(&lower, "nginx"),
                    confidence: 0.95,
                });

            }

            if lower.contains("microsoft-iis") {
                technologies.push(Technology{
                    name: "Microsoft IIS".to_string(),
                    category: "Web Server".to_string(),
                    version: extract_version(&lower, "iis"),
                    confidence: 0.95,
                });
            }
        }
    }


    // check X-powered-by checker
    if let Some(powered_by) = headers.get("x-powered-by") {
        if let Ok(powered_str) = powered_by.to_str() {

            let lower = powered_str.to_lowercase();

            if lower.contains("express") {
                technologies.push(Technology {
                    name: "Express.js".to_string(),
                    category: "Framework".to_string(),
                    version: extract_version(&lower, "express"),
                    confidenceL 0.9,
                });
            }

            if lower.contains("php") {
                technologies.push(Technology{
                    name: "PHP".to_string(),
                    category: "Language".to_string(),
                    version: extract_version(&lower, "php"),
                    confidence: 0.85,
                });
            }
            if lower.contains("asp.net") {

                technologies.push(Technology {
                    name: "ASP.NET".to_string(),
                    category: "Framework".to_string(),
                    version: extract_version(&lower, "asp.net"),
                    confidence: 0.9,
                })
            }

        }
        
    }

    if let Some(aspnet) = headers.get("x-aspnet-version") {

        if let Ok(v) = aspnet.to_str() {

            technologies.push(Technology {
                name: "ASP.NET".to_string(),
                category: "Framework".to_string(),
                version: Some(v.to_string()),
                confidence: 0.99,
            }
            );
        }
    }

    let body_lower = body.to_lowercase();

    if body_lower.contains("wp-content") || body_lower.contains("wordpress") {

        technologies.push(Technology {
            name: "WordPress".to_string(),
            category: "CMS".to_string(),
            version: detect_wordpress_version(&body),
            confidence: 0.85,
        });
    }


    if body_lower.contains("drupal") || body_lower.contains("/sites/default/") {

        technologies.push(Technology {
            name: "Drupal".to_string(),
            category: "CMS".to_string(),
            version: detect_drupal_version(&body),
            confidence: 0.8,
        });
    }

    if body_lower.contains("joomla") || body_lower.contains("mosaic") {

        technologies.push(Technology {
            name: "Joomla".to_string(),
            category: "CMS".to_string(),
            version: None,
            confidence: 0.75,
        });
    }


}