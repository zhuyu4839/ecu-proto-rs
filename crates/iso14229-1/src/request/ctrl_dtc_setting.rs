//! request of Service 85

use crate::{UdsError, request::{Request, SubFunction}, Service, Configuration, DTCSettingType};

pub(crate) fn ctrl_dtc_setting(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = DTCSettingType::try_from(sub_func.unwrap().function)?;

    Ok(Request { service, sub_func, data })
}
