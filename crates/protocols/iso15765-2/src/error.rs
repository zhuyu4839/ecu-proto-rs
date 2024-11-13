#[derive(Debug, Clone, thiserror::Error)]
pub enum IsoTpError {
    #[error("ISO 15765-2 - device error")]
    DeviceError,

    #[error("ISO 15765-2 - the pdu(protocol data unit) is empty")]
    EmptyPdu,

    #[error("ISO 15765-2 - invalid pdu(protocol data unit): {0:?}")]
    InvalidPdu(Vec<u8>),

    #[error("ISO 15765-2 - invalid parameter: {0}")]
    InvalidParam(String),

    #[error("ISO 15765-2 - invalid data length: {actual}, expect: {expect}")]
    InvalidDataLength { actual: usize, expect: usize, },

    #[error("ISO 15765-2 - data length: {0} is out of range")]
    LengthOutOfRange(usize),

    #[error("ISO 15765-2 - invalid st_min: {0:02X}")]
    InvalidStMin(u8),

    #[error("ISO 15765-2 - invalid sequence: {actual}, expect: {expect}")]
    InvalidSequence{ actual: u8, expect: u8, },

    #[error("ISO 15765-2 - mixed frames")]
    MixFramesError,

    #[error("ISO 15765-2 - timeout when time({value}{unit})")]
    Timeout { value: u64, unit: &'static str },

    #[error("ISO 15765-2 - error when converting {src:?} to {target:?}")]
    ConvertError { src: &'static str, target: &'static str, },

    #[error("ISO 15765-2 - ECU has overload flow control response")]
    OverloadFlow,

    #[error("ISO 15765-2 - context error when {0}")]
    ContextError(String),
}
