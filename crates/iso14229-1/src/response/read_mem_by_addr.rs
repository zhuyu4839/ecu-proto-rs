//! response of Service 23


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, UdsError, response::{Code, Response, SubFunction}, Service};

lazy_static!(
    pub static ref READ_MEM_BY_ADDR_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ResponseTooLong,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ]);
);

pub(crate) fn read_mem_by_addr(
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
