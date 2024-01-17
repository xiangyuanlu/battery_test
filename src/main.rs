use bytes::BufMut;
use tracing::info;

pub mod biz;
pub mod log;
pub mod serial_port;
pub mod webserver;
pub mod xutil;
use crate::webserver::start_web;
use crate::xutil::crc::{self, calc_crc16};

#[tokio::main]
async fn main() {
    let v = [1];
    let t = 1;
    let mut vv = vec![];
    let buf = [0x02, 0x02, 0x01, 0x00, 0x00, 0x2a];
    let crc16 = calc_crc16(&buf);

    vv.put_u16(crc16);
    println!("{:02x?}", crc16);
    println!("--{:02x?}", vv);

    let crcswap = crc16.swap_bytes();
    println!("16 {:02x?}", crc16);
    println!("swap {:02x?}", crcswap);
    vv.put_u16(crcswap);
    println!("++{:02x?}", vv);

    // log::init_log();
    // start_web().await;
    // info!("Starting");
}
