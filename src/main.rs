use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::path::Path;

use bytes::Bytes;
use clap::Parser;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "madoka.conf.yaml")]
    config: String,
    root: String,
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

    let config: Config = match serde_yaml::from_str(&contents) {
        Ok(config) => config,
        Err(e) => panic!("Error parsing config file: {}", e),
    };

    let root_path_buf = Path::new(&args.root).to_path_buf();

    let port = config.port;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port {}", port);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let root_path = root_path_buf.clone();
        let service = service_fn(move |req| {
            let root_path = root_path.clone();
            async move { router(req, &root_path).await }
        });
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("Failed to serve connection: {}", err);
            }
        });
    }
}
async fn router(
    req: hyper::Request<hyper::body::Incoming>,
    _root_path: &Path,
) -> hyper::Result<hyper::Response<Full<Bytes>>> {
    let method = req.method();
    let path = req.uri().path();

    match (method, path) {
        (&Method::GET, "/") => simple_file_send("index.html").await,
        _ => Ok(not_found()),
    }
}

async fn simple_file_send(filename: &str) -> hyper::Result<hyper::Response<Full<Bytes>>> {
    if let Ok(contents) = tokio::fs::read(filename).await {
        let body = contents.into();
        return Ok(Response::new(Full::new(body)));
    }
    Ok(not_found())
}

fn not_found() -> hyper::Response<Full<Bytes>> {
    let body = Bytes::from("Not Found");
    hyper::Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(body))
        .unwrap()
}
