
use super::ping::ping_target;
use super::resolver::resolve_target;
use super::types::*;



pub async fn analyze(
   request: NetworkRequest,
) -> NetworkResult {

   let ip = match resolve_target(&request.target) {

      Ok(ip) => ip,

      Err(error) => {

         println!(
            "Resolve failed: {}",
            error
         );

         return NetworkResult {

            target: request.target,

            ip: String::new(),

            online: false,
         };
      }
   };

   let online = ping_target(ip).await.is_ok();


   NetworkResult {

      target: request.target,

      ip: ip.to_string(),

      online,
   }
}