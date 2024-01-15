pub mod error;
use crate::serial_port::{port_manager::Port, *};
use crate::xutil::crc::{self, calc_crc16};
use anyhow::Result;
use parking_lot::Mutex;
use std::f32::consts::E;
use std::{error::Error, sync::Arc, thread, thread::JoinHandle};
use tracing::debug;
use tracing::error;
use tracing::info;

pub fn protocol_trans(
    dev_1: String,
    baud_1: u32,
    tm_1: u64,
    dev_2: String,
    baud_2: u32,
    tm_2: u64,
) {
    loop {
        if let Ok(s_port) = port_manager::get_manager().get_port(&dev_1, baud_1, tm_1) {
            if let Ok(mut df) = s_port.Recv(2000) {
                let buf = df.buf.as_mut_slice();
                if true == crc_check(buf) {
                    let nn = serial_2_xbus(buf);
                    if let Ok(dst_port) = port_manager::get_manager().get_port(&dev_2, baud_2, tm_2)
                    {
                        let _ = dst_port.Send(buf);
                        if let Ok(mut df) = dst_port.Recv(2000) {
                            let buf = df.buf.as_mut_slice();
                            if true == crc_check(buf) {
                                let nn = xbus_2_serial(buf);
                                let _ = s_port.Send(buf);
                            } else {
                                error!("crc check failed buf:{:?}", buf);
                                continue;
                            }
                        }
                    } else {
                        error!("Unable to get port dev/ttyS5");
                        break;
                    }
                } else {
                    error!("crc check failed buf:{:?}", buf);
                    continue;
                }
            }
        } else {
            error!("Unable to get port dev/ttyS2");
            break;
        }
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
    let thread_recv = thread::spawn({
        move || {
            protocol_trans(
                "/dev/ttyS2".to_string(),
                9600,
                2000,
                "/dev/ttyS5".to_string(),
                1200,
                2000,
            );
            // Channel::recv_loop(reqs_to_recv, port2, working_param_2);
        }
    });
    if let Err(err) = thread_recv.join() {
        return Err(error::BizError::WorkLoopFailed().into());
    }
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
