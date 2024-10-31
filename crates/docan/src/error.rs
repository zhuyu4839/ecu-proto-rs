use isotp_rs::error::Error as IsoTpError;
use iso14229_1::{Service, response::Code};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ISO14229Error(iso14229_1::Error),

    #[error("UDS - service `{service}` got an unexpected sub-function(expect: {expect}, actual: {actual})")]
    UnexpectedSubFunction { service: Service, expect: u8, actual: u8 },

    #[error("UDS - service `{expect}` got an unexpect response `{actual}`")]
    UnexpectedResponse { expect: Service, actual: Service },

    #[error("UDS - block sequence number of response (0x{actual:02x}) does not match request block sequence number (0x{expect:02x})")]
    UnexpectedTransferSequence { expect: u8, actual: u8 },

    #[error("UDS - service `{service}` got a NRC({code:?})")]
    NRCError { service: Service, code: Code },

    #[error("UDS - security algorithm error: {0}")]
    SecurityAlgoError(String),

    #[error("{0}")]
    IsoTpError(IsoTpError),

    #[error("UDS - other error: {0}")]
    OtherError(String),

    #[error("UDS - service: {0} is not implement")]
    NotImplement(Service),
}