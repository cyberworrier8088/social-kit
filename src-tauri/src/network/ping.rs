use std::net::IpAddr;
use std::time::Duration;


use surge_ping::{Client, Config, PingIdentifier, PingSequence};

pub async fn ping_target(
    ip: IpAddr,
) -> Result<f64, String> {

    let client = Client::new(&Config::default()).map_err(|error| error.to_string())?;

    let mut pinger = client.pinger(
        ip,
        PingIdentifier(1),
    ).await;


    pinger.timeout(
        Duration::from_secs(2)
    );

    let packet = pinger.ping(
        PingSequence(1),
        &[],
    ).await.map_err(|error| error.to_string())?;

    Ok(packet.1.as_secs_f64() * 1000.0)
}