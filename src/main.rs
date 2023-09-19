use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use bytes::Bytes;
use clap::Parser;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use log::{debug, error};
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
    root: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_path = args.config;

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

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

    let root_path_buf = Path::new(&config.root).to_path_buf();

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
    root_path: &Path,
) -> hyper::Result<hyper::Response<Full<Bytes>>> {
    let method = req.method();
    let path_str = req.uri().path();
    let path = Path::new(path_str);

    let full_path = resolve_file_path(root_path, path);

    match method {
        &Method::GET => simple_file_send(full_path.as_path()).await,
        _ => Ok(not_found()),
    }
}

async fn simple_file_send(file_path: &Path) -> hyper::Result<hyper::Response<Full<Bytes>>> {
    let file_data = match resolve_file_data(file_path).await {
        Ok(file_data) => file_data,
        Err(_) => return Ok(not_found()),
    };

    let builder = Response::builder()
        .header("content-type", file_data.mime_type)
        .header("content-length", file_data.content_length)
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from(file_data.content)))
        .unwrap();

    Ok(builder)
}

fn not_found() -> hyper::Response<Full<Bytes>> {
    let body = Bytes::from("Not Found");
    hyper::Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(body))
        .unwrap()
}

fn internal_server_error() -> hyper::Response<Full<Bytes>> {
    let body = Bytes::from("Internal Server Error");
    hyper::Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::new(body))
        .unwrap()
}

struct ResolvedFileData {
    content: Vec<u8>,
    mime_type: String,
    content_length: usize,
}

async fn resolve_file_data(path: &Path) -> Result<ResolvedFileData, std::io::Error> {
    let mime_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let content_length = content.len();

    Ok(ResolvedFileData {
        content,
        mime_type,
        content_length,
    })
}

fn resolve_file_path(root_path: &Path, relative_path: &Path) -> PathBuf {
    let mut full_path = root_path.to_path_buf();
    full_path.push(relative_path);
    if full_path.is_dir() {
        full_path.push("index.html");
    }
    debug!("full_path after modify: {}", full_path.display());
    full_path
}
