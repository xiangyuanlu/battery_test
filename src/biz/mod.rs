pub mod error;
use crate::serial_port::{port_manager::Port, *};
use crate::xutil::crc::calc_crc16;
use anyhow::Result;
use parking_lot::Mutex;
use std::f32::consts::E;
use std::{error::Error, sync::Arc, thread, thread::JoinHandle};
use tracing::debug;
use tracing::error;
use tracing::info;

pub fn protocol_trans(mut port: Arc<Mutex<Port>>) {
    let mut buf = [0u8; 256];

    loop {
        let pt = port.lock().read(&mut buf);
        match pt {
            Ok(n) => {
                if true == crc_check(&buf[..n]) {
                    let nn = serial_2_xbus(&mut buf[..n]);

                    if let Ok(dst_port) = port_manager::get_manager().get_port("dev/ttyS5", 1200) {
                        let wt = dst_port.lock().write(&mut buf[..nn]);
                        match wt {
                            Ok(rt) => {
                                debug!("tx to xbus success buf:{:?} tx_len:{}", &buf[..nn], rt);
                                let rt = dst_port.lock().read(&mut buf);
                                match rt {
                                    Ok(n) => {
                                        if true == crc_check(&buf[..n]) {
                                            let nn = xbus_2_serial(&mut buf[..n]);
                                            let rt = port.lock().write(&buf[..nn]);
                                            match rt {
                                                Ok(n) => {
                                                    debug!(
                                                        "tx to serial success:{:?} tx_len:{}",
                                                        &buf[..n],
                                                        n
                                                    )
                                                }
                                                Err(e) => {
                                                    error!(
                                                        "tx to serial failed:{:?}",
                                                        e.to_string()
                                                    );
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    Err(ref e) => {
                                        error!("rx from xbus failed:{:?}", e.to_string())
                                    }
                                }
                            }
                            Err(e) => error!("tx to xbus failed:{:?}", e.to_string()),
                        }
                    } else {
                        error!("dst xbus port:{} get failed", "dev/ttyS5"); //
                    }
                }
            }
            Err(ref e) => {
                if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                    error!(
                        "read error port:{}  kind:{}  msg:{:?}",
                        port.lock().get_name(),
                        io_err.kind(),
                        io_err
                    );
                } else {
                    error!("read error:{:?}", e.to_string());
                }
            }
        }
        // let rt = port.read(&mut serial_buf);
    }
}

pub fn crc_check(frm: &[u8]) -> bool {
    let mut ans = false;
    let crc_cal = calc_crc16(frm);
    let crc_val: u16 = (frm[frm.len() - 2] as u16 * 256) + frm[frm.len() - 2] as u16;
    if crc_val == crc_cal {
        ans = true;
    }
    ans
}

pub fn workloop() -> Result<bool> {
    let port = port_manager::get_manager().get_port("dev/ttyS2", 9600)?;
    let thread_recv = thread::spawn({
        move || {

            // Channel::recv_loop(reqs_to_recv, port2, working_param_2);
        }
    });
    Ok(true)
}

pub fn serial_2_xbus(buf: &mut [u8]) -> usize {
    let ans: usize = 0;
    ans
}

pub fn xbus_2_serial(buf: &mut [u8]) -> usize {
    let ans: usize = 0;
    ans
}
