use tracing::info;

pub mod biz;
pub mod log;
pub mod serial_port;
pub mod webserver;
pub mod xutil;
use crate::webserver::start_web;

#[tokio::main]
async fn main() {
    let v = [1];
    let t = 1;
    // log::init_log();
    // start_web().await;
    // info!("Starting");
}
