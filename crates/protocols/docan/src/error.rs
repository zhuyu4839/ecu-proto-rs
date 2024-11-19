use iso15765_2::Iso15765Error;
use iso14229_1::{Iso14229Error, Service, response::Code};
use rs_can::CanError;

#[derive(thiserror::Error, Debug)]
pub enum DoCanError {
    #[error("{0}")]
    DeviceError(CanError),

    #[error("{0}")]
    ISO14229Error(Iso14229Error),

    #[error("DoCAN - service `{service}` got an unexpected sub-function(expect: {expect}, actual: {actual})")]
    UnexpectedSubFunction { service: Service, expect: u8, actual: u8 },

    #[error("DoCAN - service `{expect}` got an unexpect response `{actual}`")]
    UnexpectedResponse { expect: Service, actual: Service },

    #[error("DoCAN - block sequence number of response (0x{actual:02x}) does not match request block sequence number (0x{expect:02x})")]
    UnexpectedTransferSequence { expect: u8, actual: u8 },

    #[error("DoCAN - service `{service}` got a NRC({code:?})")]
    NRCError { service: Service, code: Code },

    #[error("{0}")]
    IsoTpError(Iso15765Error),

    #[error("DoCAN - security algorithm error: {0}")]
    SecurityAlgoError(String),

    #[error("DoCAN - other error: {0}")]
    OtherError(String),

    #[error("DoCAN - service: {0} is not implement")]
    NotImplement(Service),
}
