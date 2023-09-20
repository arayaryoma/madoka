use bytes::Bytes;
use http_body_util::Full;
use hyper::StatusCode;

pub fn not_found() -> hyper::Response<Full<Bytes>> {
    let body = Bytes::from("Not Found");
    hyper::Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(body))
        .unwrap()
}

pub fn internal_server_error() -> hyper::Response<Full<Bytes>> {
    let body = Bytes::from("Internal Server Error");
    hyper::Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::new(body))
        .unwrap()
}
