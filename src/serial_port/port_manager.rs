use std::{error, rc::Rc, sync::Arc};

use super::error::SerialPortError;
use once_cell::sync::OnceCell;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits, TTYPort};

use anyhow::{Ok, Result};
use dashmap::DashMap;
use parking_lot::Mutex;

pub struct Port {
    name: String,
    baud: u32,
    port: Box<dyn SerialPort + Sync>,
}

impl Port {
    pub fn read(&mut self, buf: &mut Vec<u8>) -> Result<u32> {
        let t = self.port.read(buf)?;
        Ok(t as u32)
    }
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
    pub fn get_baud(&self) -> u32 {
        self.baud
    }
}

pub fn get_manager() -> &'static PortManager {
    static INSTANCE: OnceCell<PortManager> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let m = PortManager {
            ports: DashMap::new(),
        };
        m
    })
}
pub struct PortManager {
    ports: DashMap<String, Arc<Mutex<Port>>>,
}

impl PortManager {
    pub fn get_port(&self, name: &str, rate: u32) -> Result<Arc<Mutex<Port>>> {
        let dst_port = self.ports.get(name);
        match dst_port {
            Some(port) => {
                let pt = port.value();
                return Ok(pt.clone());
            }
            None => {
                if let core::result::Result::Ok(port) = serialport::new(name, rate).open_native() {
                    let arc_port = Arc::new(Mutex::new(Port {
                        name: name.to_string(),
                        baud: rate,
                        port: Box::new(port),
                    }));
                    self.ports.insert(name.to_string(), arc_port.clone());
                    return Ok(arc_port);
                } else {
                    return Err(
                        SerialPortError::SerialPortOpenFailed(name.to_string(), rate).into(),
                    );
                }
            }
        }
    }
}
