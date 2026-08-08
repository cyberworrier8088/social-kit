use hickory_resolver::Resolver;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct DnsResult {

    pub a: Vec<String>,
    pub aaaa: Vec<String>,
    pub mx: Vec<String>,
    pub txt: Vec<String>,
    pub ns: Vec<String>,
    pub cname: Vec<String>,

}

pub async fn lookup_dns(
    domain: &str,
) -> Result<DnsResult, String> {

    let resolver = Resolver::builder_tokio().map_err(|e| e.to_string())?.build();

    let mut result = DnsResult {
        a: Vec::new(),
        aaaa: Vec::new(),
        mx: Vec::new(),
        txt: Vec::new(),
        ns: Vec::new(),
        cname: Vec::new(),
    };

    if let Ok(response) = resolver.ipv4_lookup(domain).await {

        for ip in response.iter() {

            result.a.push(ip.to_string());
        }
    }

    if let Ok(response) = resolver.ipv6_lookup(domain).await {

        for ip in response.iter() {

            result.aaaa.push(ip.to_string());
        }
    }

    if let Ok(response) = resolver.mx_lookup(domain).await {
        for record in response.iter() {
            result.mx.push(record.exchange().to_string());
        }
    }

    if let Ok(response) = resolver.txt_lookup(domain).await {
        for record in response.iter() {
            for data in record.txt_data() {
                result.txt.push(
                    String::from_utf8_lossy(data).to_string()
                );
            }
        }
    }

    if let Ok(response) = resolver.ns_lookup(domain).await {

        for record in response.iter() {

            result.ns.push(
                record.to_string()
            );
        }
    }


    if let Ok(response) = resolver.lookup(domain, hickory_resolver::proto::rr::RecordType::CNAME).await {

        for record in response.iter() {

            result.cname.push(
                record.to_string()
            );
        }
    }
    
    Ok(result)
}