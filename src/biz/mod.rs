pub mod error;
use crate::serial_port::*;
use anyhow::{Ok, Result};
use std::{thread, thread::JoinHandle};

pub fn workloop() -> Result<bool> {
    let port = port_manager::get_manager().get_port("dev/ttyS2", 9600)?;
    let thread_recv = thread::spawn({
        move || {
            // Channel::recv_loop(reqs_to_recv, port2, working_param_2);
        }
    });
    Ok(true)
}

pub fn serial_2_xbus(buf: &mut Vec<u8>) -> Result<u32> {
    Ok(1)
}

pub fn xbus_2_serial(buf: &mut Vec<u8>) -> Result<u32> {
    Ok(1)
}
