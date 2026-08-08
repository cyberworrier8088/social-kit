use std::net::{IpAddr, ToSocketAddrs};



pub fn resolve_target(
    target: &str,
) -> Result<IpAddr, String> {

    println!("Resolving Target: {}", target);

    if let Ok(ip) = target.parse::<IpAddr>() {

        println!("Target is already an IP: {}", ip);

        return Ok(ip);
    }

    let address = format!("{}:443", target);

    println!("Resolving Address: {}", address);

    
    let address= address.to_socket_addrs().map_err(|error| {
        println!("DNS error: {}", error);

        error.to_string()
    })?;



    let mut ipv6 = None;

    for socket in address {

        match socket.ip() {

            IpAddr::V4(ip) => {

                println!("Resolved IPv4: {}", ip);

                return Ok(IpAddr::V4(ip));
            }

            IpAddr::V6(ip) => {

                ipv6 = Some(IpAddr::V6(ip));
            }
        }
    }


    if let Some(ip) = ipv6 {

        println!("No IPV4 found, using Ipv6: {}", ip);

        return Ok(ip);
    }

    Err(
        "No IP address found".to_string()
    )

}