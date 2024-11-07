//! response of Service 83

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{UdsError, response::{Code, Response, SubFunction}, Service, TimingParameterAccessType, Configuration, TimingParameter, ResponseData};

lazy_static!(
    pub static ref ACCESS_TIMING_PARAM_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
);

pub(crate) fn access_timing_param(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let sf = TimingParameterAccessType::try_from(sub_func.unwrap().0)?;
    let _ = TimingParameter::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
