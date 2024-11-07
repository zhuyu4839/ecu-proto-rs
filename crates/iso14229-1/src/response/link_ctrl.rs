//! response of Service 87

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{utils, Configuration, UdsError, LinkCtrlType, response::{Code, Response, SubFunction}, Service};

lazy_static!(
    pub static ref LINK_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
    ]);
);

pub(crate) fn link_ctrl(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = LinkCtrlType::try_from(sub_func.unwrap().0)?;
    utils::data_length_check(data.len(), 0, true)?;

    Ok(Response { service, negative: false, sub_func, data })
}
