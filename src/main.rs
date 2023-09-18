use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;

use bytes::Bytes;
use clap::Parser;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "madoka.conf.yaml")]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_path = args.config;

    let mut file = match File::open(config_path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening config file: {}", e),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => panic!("Error reading config file: {}", e),
    };

    let config: Config = match serde_yaml::from_str(&mut contents) {
        Ok(config) => config,
        Err(e) => panic!("Error parsing config file: {}", e),
    };

    let port = config.port;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port {}", port);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, service_fn(service)).await {
                eprintln!("Failed to serve connection: {}", err);
            }
        });
    }
}
async fn service(
    _req: hyper::Request<hyper::body::Incoming>,
) -> Result<hyper::Response<Full<Bytes>>, hyper::Error> {
    let body = Full::new(Bytes::from("Hello, World!"));
    Ok(hyper::Response::new(body))
}
