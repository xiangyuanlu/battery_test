use tracing::info;

pub mod biz;
pub mod log;
pub mod serial_port;
pub mod webserver;
pub mod xutil;
use crate::webserver::start_web;

//nums = [5,7,7,8,8,10], target = 8

#[tokio::main]
async fn main() {
    let v = [1];
    let t = 1;
    // log::init_log();
    // start_web().await;
    // info!("Starting");
}
