use std::net::IpAddr;
use std::time::Duration;


use surge_ping::{Client, Config, PingIdentifier, PingSequence};


pub struct PingStats {
    pub packets_sent: u32,

    pub packets_received: u32,

    pub packet_loss: f64,

    pub min_latency: Option<f64>,

    pub max_latency: Option<f64>,

    pub average_latency: Option<f64>,
}

pub async fn ping_target(
    ip: IpAddr,
) -> Result<PingStats, String> {

    let client = Client::new(&Config::default())
        .map_err(|e| e.to_string())?;

    
    let mut pinger = client.pinger(
        ip,
        PingIdentifier(1),
    ).await;

    pinger.timeout(
        Duration::from_secs(2)
    );

    let mut times = Vec::new();

    let packets_sent: u32 = 5;

    for sequence in 0..packets_sent {

        match pinger.ping(
            PingSequence(sequence as u16),
            &[],
        ).await {

            Ok((_, duration)) => {

                let ms = duration.as_secs_f64() * 1000.0;

                times.push(ms);
            }

            Err(error) => {

                println!("Ping {} failed: {}", sequence + 1, error);
            }
        }
    }


    let packets_received = times.len() as u32;

    let packet_loss = if packets_sent == 0 {
        
        0.0
    } else {
        (

            (packets_sent - packets_received) as f64 / packets_sent as f64
        ) * 100.0
    };


    let min_latency = times.iter().copied().reduce(f64::min);

    let max_latency = times.iter().copied().reduce(f64::max);

    let average_latency = if times.is_empty() {

        None
    } else {
        Some(
            times.iter().sum::<f64>() / times.len() as f64
        )
    };

    Ok(PingStats {
        packets_sent,
        packets_received,
        packet_loss,
        min_latency,
        max_latency,
        average_latency,
    })
}