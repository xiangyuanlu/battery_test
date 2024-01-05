use tracing::info;

pub mod log;
pub mod serial_port;
fn main() {
    log::init_log();
    info!("Starting");
    println!("Hello, world!");
}
