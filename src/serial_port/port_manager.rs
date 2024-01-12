use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{
    error,
    rc::Rc,
    sync::Arc,
    thread::{self, Thread},
};

use super::error::SerialPortError;
use once_cell::sync::OnceCell;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits, TTYPort};

use anyhow::{Ok, Result};
use dashmap::DashMap;
use parking_lot::Mutex;
use tracing::debug;
use tracing::error;

pub struct Port {
    name: String,
    baud: u32,
    tx: crossbeam::channel::Sender<DataFrame>,
    rx: crossbeam::channel::Receiver<DataFrame>,
    // port: Box<dyn SerialPort + Sync>,
    trd_rx: Option<JoinHandle<()>>,
    trd_tx: Option<JoinHandle<()>>,

    runing: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct DataFrame {
    buf: Vec<u8>,
}

impl Drop for Port {
    fn drop(&mut self) {
        self.runing
            .store(false, std::sync::atomic::Ordering::SeqCst);
        if let Some(trd) = self.trd_rx.take() {
            trd.join().unwrap();
        }
        if let Some(trd) = self.trd_rx.take() {
            trd.join().unwrap();
        }
    }
}

impl Port {
    pub fn new(name: String, baud: u32, tout: u64) -> Result<Self> {
        let (tx, rx) = crossbeam::channel::bounded(12);

        let mut port = serialport::new(name, baud)
            .timeout(Duration::from_millis(tout))
            .open_native()?;

        let runing = Arc::new(AtomicBool::new(true));
        let trd_rx = {
            let mut port = port.try_clone()?;
            let runing = runing.clone();
            let mut buffer = [0u8; 512];

            thread::spawn(move || loop {
                match port.read(&mut buffer[..]) {
                    Result::Ok(size) => {
                        debug!("{} rx:{:02X?}", name, &buffer[..size]);
                        let df = DataFrame {
                            buf: buffer.to_vec(),
                        };
                        tx.send(df);
                    }
                    Err(err) => {
                        error!("{} rx error: {}", name, err);
                    }
                }
            })
        };
        let trd_tx = {
            let mut buffer = [0u8; 512];
            let mut port = port.try_clone()?;

            thread::spawn(move || loop {
                match rx.recv() {
                    Result::Ok(df) => {
                        if let Err(err) = port.write(&df.buf[..]) {
                            error!("{} tx error: {}", name, err);
                        } else {
                            debug!("{} tx:{:02X?}", name, &df.buf[..]);
                        }
                    }
                    Err(err) => {}
                }

                match port.read(&mut buffer[..]) {
                    Result::Ok(size) => {
                        debug!("{} rx:{:02X?}", name, &buffer[..size]);
                        let df = DataFrame {
                            buf: buffer.to_vec(),
                        };
                        tx.send(df);
                    }
                    Err(err) => {
                        error!("{} rx error: {}", name, err);
                    }
                }
            })
        };
        let port = Port {
            name: "".to_string(),
            baud: 1200,
            tx,
            rx,
            trd_rx: Some(trd_rx),
            trd_tx: Some(trd_tx),
            runing,
        };
        Ok(port)
    }

    fn Recv(&self) -> Result<DataFrame> {
        let df = self.rx.recv()?;
        Ok(df)
    }

    fn Send(&self, buf: &[u8]) -> Result<usize> {
        let df = DataFrame { buf: buf.to_vec() };
        let sz = self.tx.send(df)?;
        Ok(buf.len())
    }
}

impl Port {
    // pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    //     let tt = self.port.read(buf);
    //     if let Err(e) = tt {
    //         print!("Error reading:{}", e.kind());
    //     }
    //     let rt = self.port.read(buf)?;
    //     Ok(rt)
    // }

    // pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
    //     let rt = self.port.write(buf)?;
    //     Ok(rt)
    // }

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
        return Err(SerialPortError::SerialPortOpenFailed(name.to_string(), rate).into());
    }
    //     let dst_port = self.ports.get(name);
    //     match dst_port {
    //         Some(port) => {
    //             let pt = port.value();
    //             return Ok(pt.clone());
    //         }
    //         None => {
    //             if let core::result::Result::Ok(port) = serialport::new(name, rate).open_native() {
    //                 let arc_port = Arc::new(Mutex::new(Port {
    //                     name: name.to_string(),
    //                     baud: rate,
    //                     port: Box::new(port),
    //                 }));
    //                 self.ports.insert(name.to_string(), arc_port.clone());
    //                 return Ok(arc_port);
    //             } else {
    //                 return Err(
    //                     SerialPortError::SerialPortOpenFailed(name.to_string(), rate).into(),
    //                 );
    //             }
    //         }
    //     }
    // }
}
