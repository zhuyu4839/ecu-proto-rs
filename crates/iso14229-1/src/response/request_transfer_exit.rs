//! response of Service 37


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, UdsError, response::{Code, Response, SubFunction}, Service};

lazy_static!(
    pub static ref REQUEST_TRANSFER_EXIT_NEGATIVES: HashSet<Code>
    = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::GeneralProgrammingFailure,
    ]);
);

pub(crate) fn request_transfer_exit(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    Ok(Response { service, negative: false, sub_func, data })
}
