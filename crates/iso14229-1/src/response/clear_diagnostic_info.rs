//! response of Service 14


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, Error, response::{Code, Response, SubFunction}, Service, utils};

lazy_static!(
    pub static ref CLEAR_DIAGNOSTIC_INFO_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        Code::GeneralProgrammingFailure,
    ]);
);

pub(crate) fn clear_diag_info(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Response, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    utils::data_length_check(data.len(), 0, true)?;

    Ok(Response { service, negative: false, sub_func, data })
}
