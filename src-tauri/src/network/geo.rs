use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoResult {

    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub timezone: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,


}


#[derive(Debug, Deserialize)]
struct IpApiResponse {

    country: Option<String>,
    #[serde(rename = "regionName")]
    region_name: Option<String>,
    city: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
    timezone: Option<String>,
    isp: Option<String>,
    org: Option<String>,
}


pub async fn lookup_geo(
    ip: &str,
) -> Result<GeoResult, String> {

    let url = format!("http://ip-api.com/json/{}", ip);

    let client = Client::new();

    let response = client.get(&url).header("User-Agent", "SocialKit/1.0").send().await.map_err(|error| error.to_string())?;


    
    if !response.status().is_success() {
        return Err(format!(
            "GeoIP request failed: {}",
            response.status()
        ));

    }

    let data = response.json::<IpApiResponse>().await.map_err(|error| error.to_string())?;

    Ok( GeoResult {

        country: data.country,

        region: data.region_name,

        city: data.city,

        latitude: data.lat.map(|value| value.to_string()),

        longitude: data.lon.map(|value| value.to_string()),

        timezone: data.timezone,

        isp: data.isp,

        organization: data.org,
    })
}