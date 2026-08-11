use reqwest::Client;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct WhoisResult {

    pub registrar: Option<String>,

    pub creation_date: Option<String>,

    pub updated_date: Option<String>,

    pub expiration_date: Option<String>,

    pub nameservers: Vec<String>,

    pub domain_status: Vec<String>,

}

#[derive(Debug, Deserialize)]
struct WhoisResponse {

    registrar: Option<String>,

    #[serde(rename = "creationDate")]
    creation_date: Option<String>,

    #[serde(rename = "updatedDate")]
    updated_date: Option<String>,

    #[serde(rename = "expirationDate")]
    expiration_date: Option<String>,

    name_servers: Option<Vec<String>>,

    status: Option<Vec<String>>,
}

pub async fn lookup_whois(domain: &str,) -> Result<WhoisResult, String> {

    let domain = domain.trim().trim_start_matches("https://").trim_start_matches("http://").trim_start_matches('/');


    let url = format!(
        "https://rdap.org/domain/{}",
        domain
    );


    let client = Client::new();

    let response = client.get(&url).header("User-Agent", "ScoialKit/1.0").send().await.map_err(|error| error.to_string())?;

    if !response.status().is_success() {

        return Err(format!(
            "WHOIS/RDAP lookup failed: {}",
            response.status()
        ));

    }

    let data = response.json::<WhoisResponse>().await.map_err(|error| error.to_string())?;

    Ok(WhoisResult {

        registrar: data.registrar,

        creation_date: data.creation_date,

        updated_date: data.updated_date,

        expiration_date: data.expiration_date,


        nameservers: data.name_servers.unwrap_or_default(),

        domain_status: data.status.unwrap_or_default(),
    })
}