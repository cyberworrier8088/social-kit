use std::net::TcpStream;
use std::sync::Arc;

use x509_parser::prelude::*;



use rustls::{
    ClientConfig,
    ClientConnection,
    RootCertStore,
    StreamOwned,
    pki_types::ServerName,
};


use super::types::SslResult;


pub fn check_ssl(
    domain: &str,
) -> Result<SslResult, String> {

    let _ = rustls::crypto::ring::default_provider().install_default();
    let address = format!("{}:443", domain);

    let stream = TcpStream::connect(&address).map_err(|error| error.to_string())?;

    let mut roots = RootCertStore::empty();

    let native_certs = rustls_native_certs::load_native_certs();

    for cert in native_certs.certs {

        roots.add(cert).map_err(|error| error.to_string())?;
    }


    let config = ClientConfig::builder().with_root_certificates(roots).with_no_client_auth();

    let server_name = ServerName::try_from(
        domain.to_string()
    ).map_err(|error| error.to_string())?;

    let connection = ClientConnection::new(
        Arc::new(config),
        server_name,
    ).map_err(|error| error.to_string())?;

    let mut tls = StreamOwned::new(connection, stream);

    tls.conn.complete_io(&mut tls.sock).map_err(|error| error.to_string())?;

    let tls_version = tls.conn.protocol_version().map(|version| format!("{:?}", version)).unwrap_or_else(|| "Unknown".to_string());

    let certificates = tls.conn.peer_certificates().ok_or_else(|| {
        "Server did not provide a certificate".to_string()
    })?;


    let certificate = certificates.first().ok_or_else(|| {
        "Certificate list is empty".to_string()
    })?;


    let (_, parsed) = parse_x509_certificate(
        certificate.as_ref()
    ).map_err(|error| error.to_string())?;

    let subject = parsed.subject().to_string();

    let issuer = parsed.issuer().to_string();

    let valid_from = parsed.validity().not_before.to_string();

    let expires = parsed.validity().not_after.to_string();

    Ok(SslResult {

        status: "Valid".to_string(),

        subject,

        issuer,

        valid_from,

        expires,

        tls_version,
    })

}
