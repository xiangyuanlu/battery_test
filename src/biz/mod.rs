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
use std::ptr::addr_of;
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
        let s_port = match port_manager::get_manager().get_port(&dev_1, baud_1, tm_1) {
            Ok(port) => port,
            Err(err) => {
                error!("failed to get port:{} err:{:?}", dev_1, err);
                break;
            }
        };

        let mut df = match s_port.Recv(2000) {
            Ok(df) => df,
            Err(err) => {
                error!("src port:{} recv error:{:?}", dev_1, err);
                continue;
            }
        };

        let buf_1 = df.buf.as_mut_slice();
        let buf = df.buf.as_slice();
        if !crc_check(buf) {
            error!("crc check failed buf:{:?}", buf);
            continue;
        }

        let mut vec = match serial_2_xbus(buf) {
            Ok(vec) => vec,
            Err(err) => {
                error!("failed to trans buf from serial to xbus: {:?}", err);
                continue;
            } // Handle the error case appropriately
        };

        let gate_addr = match vec.pop() {
            Some(addr) => addr,
            None => {
                error!("failed to get gate address");
                continue;
            }
        };
        let d_port = match port_manager::get_manager().get_port(&dev_2, baud_2, tm_2) {
            Ok(port) => port,
            Err(err) => {
                error!("failed to get port:{} err:{:?}", dev_2, err);
                break;
            }
        };

        let rs = match d_port.Send(vec.as_slice()) {
            Ok(r_len) => r_len,
            Err(err) => {
                error!("failed send to xbus");
                continue;
            }
        };

        let mut df = match d_port.Recv(2000) {
            Ok(df) => df,
            Err(_) => continue,
        };

        let buf = df.buf.as_slice();
        if !crc_check(buf) {
            error!("crc check failed buf:{:02x?}", buf);
            continue;
        }

        let mut vec = match xbus_2_serial(buf, gate_addr) {
            Ok(vec) => vec,
            Err(_) => continue, // Handle the error case appropriately
        };

        let rs = match s_port.Send(vec.as_slice()) {
            Ok(r_len) => r_len,
            Err(err) => {
                error!("failed send to xbus");
                continue;
            }
        };
    }
}

//02 02 01 00 00 2a f8 1a
pub fn crc_check(frm: &[u8]) -> bool {
    let mut ans = false;
    let crc_cal = calc_crc16(frm);
    let crc_val: u16 = (frm[frm.len() - 1] as u16 * 256) + frm[frm.len() - 2] as u16;
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
pub fn serial_2_xbus(buf: &[u8]) -> Result<Vec<u8>> {
    let mut ans: Vec<u8> = vec![];
    let plen = buf.len();
    if plen < 5 {
        error!("buf len too short, len is:{}", plen);
        return Err(error::PkgError::PackageLenTooShort(plen).into());
    }
    let mut cmd = buf[4];
    if let Some(cmd) = cmdBridge.get(&cmd) {
    } else {
        error!("cmd trans failed, can not get pair cmd:{}", cmd);
        return Err(error::PkgError::CmdCanNotTrans(cmd).into());
    }
    ans.push(buf[0]);
    //del gate addr buf[1]
    ans.put_slice(&buf[2..3]);
    ans.push(cmd);
    ans.put_slice(&buf[5..8]);

    let mut crc_cal = calc_crc16(ans.as_slice());
    crc_cal = crc_cal.swap_bytes();
    ans.put_u16(crc_cal);
    //gate_addr will be used in buf trans to serial
    ans.push(buf[1]);

    return Ok(ans);
}

pub fn xbus_2_serial(buf: &[u8], gate_addr: u8) -> Result<Vec<u8>> {
    let ans: usize = 0;
    todo!()
}

pub fn protocol_trans_bak(
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
                    if let Ok(mut vec) = nn {
                        let gate_addr = vec.pop().unwrap();
                        if let Ok(dst_port) =
                            port_manager::get_manager().get_port(&dev_2, baud_2, tm_2)
                        {
                            let _ = dst_port.Send(buf);
                            if let Ok(mut df) = dst_port.Recv(2000) {
                                let buf = df.buf.as_mut_slice();
                                if true == crc_check(buf) {
                                    let nn = xbus_2_serial(buf, 1);
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
                        //todo no trace
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
