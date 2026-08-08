use std::net::{IpAddr, ToSocketAddrs};


pub fn resolve_target(target: &str) -> Result<IpAddr, String> {

    println!("Resolving target: {}", target);


    if let Ok(ip) = target.parse::<IpAddr>() {

        println!("Target is already an IP: {}", ip);


        return Ok(ip);
    }

    let address = format!("{}:443", target);

    println!("Resolving address: {}", address);

    let mut address = address.to_socket_addrs().map_err(|error| {
        println!("DNS error: {}", error);

        error.to_string()
    })?;


    match address.next() {
        
        Some(socket) => {

            println!("Resolved IP: {}", socket.ip());

            Ok(socket.ip())
        }

        None => {

            println!("No IP address found");
            Err("No IP address Found".to_string())

        }
    }
}