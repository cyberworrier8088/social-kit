use super::ping::ping_target;
use super::resolver::resolve_target;
use super::dns::lookup_dns;
use super::ssl::check_ssl;
use super::headers::check_headers;
use super::types::*;



use super::web_files::check_web_files;
use super::geo::lookup_geo;
use super::hosting::lookup_hosting;

pub async fn analyze(
   mut request: NetworkRequest,
) -> NetworkResult {
   request.target = request.target.trim().trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/').to_string();

   let ip = match resolve_target(
      &request.target
   ) {

      Ok(ip) => ip,

      Err(error) => {

         println!("Resolve faild: {}", error);

         return NetworkResult {

            target: request.target,

            ip: String::new(),

            online: false,

            latency: None,

            packets_sent: 0,

            packets_received: 0,

            packet_loss: 100.0,
            
            min_latency: None,

            max_latency: None,

            average_latency: None,

            dns: None,

            ssl: None,

            security_headers: None,

            web_files: None,

            geo: None,

            hosting: None,
         };
      }
   };

   let dns = lookup_dns(&request.target).await.ok();

   let ssl = check_ssl(&request.target).ok();

   let security_headers = check_headers(&request.target).await.ok();

   let web_files = check_web_files(&request.target).await.ok();

   let geo = lookup_geo(&ip.to_string()).await.ok();

   let hosting = lookup_hosting(&ip.to_string()).await.ok();

   let stats = match ping_target(ip).await {

      Ok(stats) => stats,

      Err(error) => {

         println!("Ping faild: {}", error);

         return NetworkResult {

            target: request.target,

            ip: ip.to_string(),

            online: false,

            latency: None,

            packets_sent: 5,

            packets_received: 0,

            packet_loss: 100.0,

            min_latency: None,

            max_latency: None,

            average_latency: None,

            dns: None,

            ssl: None,

            security_headers: None,

            web_files: None,

            geo: None,

            hosting: None,
         };
      }
   };


   NetworkResult {

      target: request.target,

      ip: ip.to_string(),

      online: stats.packets_received > 0,

      latency: stats.average_latency,

      packets_sent: stats.packets_sent,

      packets_received: stats.packets_received,

      packet_loss: stats.packet_loss,

      min_latency: stats.min_latency,

      max_latency: stats.max_latency,

      average_latency: stats.average_latency,

      dns,

      ssl,

      security_headers,

      web_files,
      
      geo,

      hosting,
   }
}