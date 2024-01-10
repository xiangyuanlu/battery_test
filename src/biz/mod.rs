pub mod error;
use crate::serial_port::{port_manager::Port, *};
use anyhow::{Ok, Result};
use parking_lot::Mutex;
use std::{sync::Arc, thread, thread::JoinHandle};

pub fn protocol_trans(mut port: Arc<Mutex<Port>>) {
    let mut serial_buf: Vec<u8> = vec![0; 256];

    loop {
        let pt = port.lock().read(&mut serial_buf);
        // let rt = port.read(&mut serial_buf);
    }
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

pub fn serial_2_xbus(buf: &mut Vec<u8>) -> Result<u32> {
    Ok(1)
}

pub fn xbus_2_serial(buf: &mut Vec<u8>) -> Result<u32> {
    Ok(1)
}
