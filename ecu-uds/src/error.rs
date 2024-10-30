use isotp_rs::error::Error as IsoTpError;
use crate::service::{DataIdentifier, Service};
use crate::service::response::Code;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("UDS - invalid parameter `{0}`")]
    InvalidParam(String),

    #[error("UDS - data: {0} is invalid")]
    InvalidData(String),

    #[error("UDS - received message doesn't correspond to expected length(expect: {expect}, actual: {actual})")]
    InvalidDataLength { expect: usize, actual: usize },

    #[error("UDS - the length of data identifier: {0:?} is not configured")]
    DidNotSupported(DataIdentifier),

    #[error("UDS - invalid dynamically defined data identifier: {0:x}")]
    InvalidDynamicallyDefinedDID(u16),

    #[error("UDS - invalid session data {0}")]
    InvalidSessionData(String),

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
