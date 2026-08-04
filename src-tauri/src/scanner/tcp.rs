use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use super::types::{ScanRequest, ScanResult};

pub fn scan_tcp(request: ScanRequest) -> ScanResult {
    let mut open = false;

    let address = format!("{}:{}", request.target, request.port);

    if let Ok(mut addresses) = address.to_socket_addrs() {
        if let Some(socket) = addresses.next() {
            open = TcpStream::connect_timeout(
                &socket,
                Duration::from_millis(request.timeout)
            ).is_ok();
        }
    }

    ScanResult {
        target: request.target,
        port: request.port,
        open,
    }
}