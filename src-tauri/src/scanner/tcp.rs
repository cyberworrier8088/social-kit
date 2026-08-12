use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use super::types::{ScanRequest, ScanResult};

pub fn scan_tcp(request: ScanRequest) -> ScanResult {

    let address = format!("{}:{}", request.target, request.port);

    let mut open = false;

    if let Ok(mut addresses) = address.to_socket_addrs() {

        if let Some(socket) = addresses.next() {

            open = TcpStream::connect_timeout(&socket, Duration::from_millis(request.timeout)).is_ok();

        }
    } 

    let service = if open {
        service_name(request.port)
    } else {
        None
    };


    ScanResult {

        target: request.target,
        port: request.port,
        open,
        service,
    }


}


fn service_name(
    port: u16,
) -> Option<String> {

    let service = match port {

        20 => "FTP Date",
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        110 => "POP3",
        143 => "IMAP",
        443 => "HTTPS",
        445 => "SMB",
        587 => "SMTP",
        993 => "IMAPS",
        995 => "POP3S",
        1433 => "Microsoft SQL Server",
        1521 => "Oracle",
        2049 => "NFS",

        _ => return None,
        
    };

    Some(service.to_string())
}