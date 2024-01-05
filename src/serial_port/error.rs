use thiserror::Error;
//为自定义error继承Error trait
#[derive(Error, Debug)]
pub enum SerialPortError {
    #[error("north params illegal:{0}")]
    NorthParamsIllegal(String),
    #[error("mqtt params illegal:{0}")]
    MqttParamsIllegal(String),
    #[error("mqtt connection failed:{0}")]
    MqttConntionFailed(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
