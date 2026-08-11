use reqwest::Client;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct HostingResult {

    pub ip: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,
    pub asn: Option<String>,
    pub as_name: Option<String>,
}


#[derive(Debug, Deserialize)]
struct IpApiResponse {

    query: Option<String>,
    isp: Option<String>,
    org: Option<String>,
    
    #[serde(rename = "as")]
    as_value: Option<String>,
}

pub async fn lookup_hosting(
    ip: &str,
) -> Result<HostingResult, String> {

    let url = format!("http://ip-api.com/json/{}", ip);

    let client = Client::new();

    let response = client.get(&url).header("User-Agent", "SocialKit/1.0").send().await.map_err(|error| error.to_string())?;

    if !response.status().is_success() {

        return Err(format!(
            "Hosting lookup failed: {}",
            response.status()
        ));
    }


    let data = response.json::<IpApiResponse>().await.map_err(|error| error.to_string())?;

    let as_value = data.as_value.unwrap_or_default();

        let mut asn = None;

        let mut as_name = None;

        if !as_value.is_empty() {
            
        let mut parts = as_value.splitn(2, ' ');

        let first = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("");

            if first.starts_with("AS") {

                asn = Some(first.to_string());

                if !rest.is_empty() {

                    as_name = Some(rest.to_string());

                }
            } else {

                as_name = Some(as_value);
            }
        }

    Ok(HostingResult {
        ip: data.query,

        isp: data.isp,

        organization: data.org,

        asn,
        
        as_name,
    })
}