use super::ping::ping_target;
use super::resolver::resolve_target;
use super::dns::lookup_dns;
use super::types::*;

pub async fn analyze(
   request: NetworkRequest,
) -> NetworkResult {

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
         };
      }
   };

   let dns = lookup_dns(&request.target).await.ok();

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
   }
}