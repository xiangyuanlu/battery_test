use thiserror::Error;
//为自定义error继承Error trait
#[derive(Error, Debug)]
pub enum BizError {
    #[error("serial port open failed name:{0} baud:{1}")]
    SerialPortOpenFailed(String, u32),
    #[error("mqtt params illegal:{0}")]
    MqttParamsIllegal(String),
    #[error("mqtt connection failed:{0}")]
    MqttConntionFailed(String),
    #[error("work loop start failed")]
    WorkLoopFailed(),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum PkgError {
    #[error("pkg len too short:{0}")]
    PackageLenTooShort(usize),
    #[error("cmd can trans, no pair cmd:{0}")]
    CmdCanNotTrans(u8),
    #[error("mqtt connection failed:{0}")]
    MqttConntionFailed(String),
    #[error("work loop start failed")]
    WorkLoopFailed(),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
