pub mod error;
use crate::serial_port::{port_manager::Port, *};
use crate::xutil::crc::{self, calc_crc16};
use anyhow::Result;
use bytes::BufMut;
use chrono::Utc;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::f32::consts::E;
use std::{error::Error, sync::Arc, thread, thread::JoinHandle};
use tracing::debug;
use tracing::error;
use tracing::info; // 1.4.0

// static cmd2xbus: std::collections::hash_map<u8, u8> = std::collections::HashMap::new();
lazy_static! {
    static ref cmdBridge: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        m.insert(128, 48);
        m
    };
}

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
                    let nn: std::prelude::v1::Result<Box<[u8]>, anyhow::Error> = serial_2_xbus(buf);
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
        }
    });
    if let Err(err) = thread_recv.join() {
        error!("Failed to join thread: {:?}", err);
        return Err(error::BizError::WorkLoopFailed().into());
    }
    Ok(true)
}

// G3000    cmd
// GetVersion      0xd1
// AutoAddress     0x30
// MeasureV        0x17
// MeasureT        0x12
// MeasureR        0x11
// MeasureRCali    0x14
// MeasureI        0x13
// MeasureV&T      0x19
// MeasureGV       0x16

// CaliV           0x20
// CaliR           0x21
// CaliT           0x22
// CaliIZero       0x23
// CaliISecond     0x24
// CaliGVzero      0x25
// CaliGVSecond    0x26

// RestoreR        0x20 0x21 0x22
// RestoreI        0x23 0x24
// RestoreGV       0x25 0x26

// SetBaud         0xc3
// SetSleep        0xb5
// SetWeak         0xb6
// GetSleep        0xb7
// SetAlarmLight   0xa1 - 0xa6
// SetAlarmClear   0xb0

// SetIRange       0x40
// SetVRange       0x44

//G3000 autoaddress
//  ab   addr cmd  raddr  iaddr crc
// 0xab 00 00  30  00 00  80 00  c1 c2
//tool autoaddress
// ab gate_addr addr  cmd   raddr iaddr crc
// ab ga       00 00  0x80
pub fn serial_2_xbus(buf: &[u8]) -> Result<Box<[u8]>> {
    let mut trans: Vec<u8> = vec![];
    let plen = buf.len();
    if plen < 5 {
        return Err(error::PkgError::PackageLenTooSmall(plen).into());
    }
    let mut cmd = buf[4];
    if let Some(cmd) = cmdBridge.get(&cmd) {
    } else {
        return Err(error::PkgError::CmdCanNotTrans(cmd).into());
    }
    trans.push(buf[0]);
    //del gate addr buf[1]
    trans.put_slice(&buf[2..3]);
    trans.push(cmd);
    trans.put_slice(&buf[5..8]);

    let crc_cal = calc_crc16(trans.as_slice());

    let ans: usize = 0;
    todo!()
}

pub fn xbus_2_serial(buf: &[u8]) -> Result<Box<[u8]>> {
    let ans: usize = 0;
    todo!()
}
