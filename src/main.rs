use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bytes::Bytes;
use clap::Parser;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use log::{debug, error};
use tokio::net::TcpListener;

mod hyper_response_util;
pub mod tls;
use hyper_response_util::not_found;

mod config;
use config::{read_config_from_file, Config, ConfigHeaders};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "madoka.conf.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_path = args.config;

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = read_config_from_file(config_path.as_str())?;
    let port = config.port;
    let config_arc = Arc::new(config);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port {}", port);
    loop {
        let config_arc = Arc::clone(&config_arc);
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let service = service_fn(move |req| {
            let config_arc = Arc::clone(&config_arc);
            async move { router(req, &Arc::clone(&config_arc)).await }
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
    config: &Arc<Config>,
) -> hyper::Result<hyper::Response<Full<Bytes>>> {
    let method = req.method();
    let host_header_value = req.headers().get("host").unwrap().to_str().unwrap();

    let uri = host_header_value.parse::<hyper::Uri>().unwrap();
    let req_host = uri.host().unwrap().to_string();
    let hosts = &config.hosts;
    let target_host = hosts
        .iter()
        .find(|host| host.host_name == req_host)
        .unwrap();
    let root_path_str = target_host.root.clone();
    let root_path = Path::new(&root_path_str);

    let appended_headers: Option<ConfigHeaders> = target_host.add_header.clone();

    let mut path_str = req.uri().path();
    if path_str == "/" {
        path_str = "index.html";
    }
    let path = Path::new(path_str);

    let full_path = resolve_file_path(root_path, path);

    match method {
        &Method::GET => simple_file_send(full_path.as_path(), appended_headers).await,
        _ => Ok(not_found()),
    }
}

async fn simple_file_send(
    file_path: &Path,
    res_headers: Option<ConfigHeaders>,
) -> hyper::Result<hyper::Response<Full<Bytes>>> {
    let file_data = match resolve_file_data(file_path).await {
        Ok(file_data) => file_data,
        Err(e) => {
            error!("Error reading file: {}", e);
            return Ok(not_found());
        }
    };

    let mut builder = Response::builder()
        .header("content-type", file_data.mime_type)
        .header("content-length", file_data.content_length)
        .status(StatusCode::OK);

    if let Some(headers) = res_headers {
        builder = set_headers_to_builder(builder, headers);
    }

    let res = builder
        .body(Full::new(Bytes::from(file_data.content)))
        .unwrap();

    Ok(res)
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
    let canonicalized_relative_path = relative_path.strip_prefix("/").unwrap_or(relative_path);
    let mut full_path = root_path.to_path_buf();
    full_path.push(canonicalized_relative_path);
    if full_path.is_dir() {
        full_path.push("index.html");
    }
    full_path
}

fn set_headers_to_builder(
    builder: hyper::http::response::Builder,
    headers: ConfigHeaders,
) -> hyper::http::response::Builder {
    let mut builder = builder;
    for header in headers {
        for (key, value) in header {
            builder = builder.header(key, value);
        }
    }
    builder
}
