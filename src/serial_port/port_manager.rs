use std::{error, sync::Arc};

use once_cell::sync::OnceCell;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};

use anyhow::{Ok, Result};
use dashmap::DashMap;
use parking_lot::Mutex;
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
    ports: DashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>,
}

impl PortManager {
    pub fn get_port(&self, name: &str, rate: u32) -> Result<Arc<Mutex<Box<dyn SerialPort>>>> {
        let dst_port = self.ports.get(name);
        match dst_port {
            Some(port) => {
                let pt = port.value();
                return Ok(pt.clone());
            }
            None => {
                if let core::result::Result::Ok(port) =
                    serialport::new("/dev/ttyUSB0", 115_200).open()
                {
                    return Ok(Arc::new(Mutex::new(port)));
                } else {
                    return Err(anyhow::Error::msg("dummy errror"));
                }
            }
        }
    }
}
