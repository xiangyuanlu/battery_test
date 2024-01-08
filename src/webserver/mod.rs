use axum::{http::StatusCode, routing::get, Router};
use axum_server::{service, tls_rustls::RustlsConfig};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
pub async fn start_web() {
    println!("path: {:?}", PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();
    let app = Router::new()
        .nest_service("/", ServeDir::new("www"))
        .route("/hello", get(handler_hello))
        .route_service("/index", ServeFile::new("www/index.html"))
        .route_service("/index_test", ServeFile::new("www/static.html"));

    // let addr = SocketAddr::from(([192, 168, 2, 43], 3000));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, root!"
}

async fn handler_hello() -> &'static str {
    "Hello, hello!"
}
