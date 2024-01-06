use tracing::info;

pub mod log;
pub mod serial_port;
pub mod webserver;

use crate::webserver::start_web;
#[tokio::main]

async fn main() {
    log::init_log();
    start_web().await;
    info!("Starting");
    println!("Hello, world!");
}
