use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
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
        .route("/hello", get(handler_hello))
        .route("/", get(handler))
        .route_service("/start", ServeFile::new("/xbrother/app/www/index.html"));

    let addr = SocketAddr::from(([192, 168, 2, 43], 3000));

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
