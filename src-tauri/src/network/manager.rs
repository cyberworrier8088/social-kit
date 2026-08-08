use super::resolver::resolve_target;
use super::types::*;


pub fn analyze(
   request: NetworkRequest,
) -> NetworkResult {

   let ip = match resolve_target(&request.target) {

      Ok(ip) => ip.to_string(),

      Err(error) => {

         println!("Resolve faild: {}", error);

         String::new()
      }
   };


   NetworkResult {

      target: request.target,

      ip: ip.clone(),

      online: !ip.is_empty(),

   }
}