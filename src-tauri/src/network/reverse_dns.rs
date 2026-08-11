use std::net::IpAddr;

use hickory_resolver::Resolver;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReverseDnsResult {

    pub ip: String,
    pub hostnames: Vec<String>,
}

pub async fn lookup_reverse_dns(ip: &str) -> Result<ReverseDnsResult, String> {

    let ip: IpAddr = ip.parse::<IpAddr>().map_err(|error| error.to_string())?;

    let resolver = Resolver::builder_tokio().map_err(|error| error.to_string())?.build();


    let response = resolver.reverse_lookup(ip).await.map_err(|error| error.to_string())?;

    let hostnames = response.iter().map(|name| name.to_string()).collect();

    Ok(ReverseDnsResult {
        ip: ip.to_string(),
        hostnames,
    })
}