use tracing::info;

pub mod log;
pub mod serial_port;
pub mod webserver;

use crate::webserver::start_web;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (a, b) = (matrix.len(), matrix[0].len());

    let mut x = 0 as usize;
    let mut y = b - 1;

    while x < a && y >= 0 {
        if target == matrix[x][y] {
            return true;
        } else if target > matrix[x][y] {
            y -= 1;
        } else {
            x += 1;
        }
    }
    return false;
}
#[tokio::main]
async fn main() {
    log::init_log();
    start_web().await;
    info!("Starting");
}
