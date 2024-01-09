use once_cell::sync::OnceCell;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::sync::{Arc, Mutex};

use dashmap::DashMap;
pub fn get_channel_manager() -> &'static PortManager {
    static INSTANCE: OnceCell<PortManager> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let m = PortManager {
            ports: DashMap::new(),
        };
        m
    })
}
pub struct PortManager {
    ports: DashMap<String,  <Box<dyn SerialPort>>,
}
