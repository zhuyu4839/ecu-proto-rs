//! response of Service 3E


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{utils, UdsError, response::{Code, Response, SubFunction}, Service, TesterPresentType, Configuration};

lazy_static!(
    pub static ref TESTER_PRESENT_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
    ]);
);

pub(crate) fn tester_present(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = TesterPresentType::try_from(sub_func.unwrap().0)?;
    utils::data_length_check(data.len(), 0, true)?;

    Ok(Response { service, negative: false, sub_func, data })
}
