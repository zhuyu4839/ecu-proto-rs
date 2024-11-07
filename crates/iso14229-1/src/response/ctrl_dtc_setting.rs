//! response of Service 85

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{DTCSettingType, UdsError, response::{Code, Response, SubFunction}, Service, utils, Configuration};

lazy_static!(
    pub static ref CTRL_DTC_SETTING_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
);

pub(crate) fn ctrl_dtc_setting(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = DTCSettingType::try_from(sub_func.unwrap().0)?;
    utils::data_length_check(data.len(), 0, true)?;

    Ok(Response { service, negative: false, sub_func, data })
}
