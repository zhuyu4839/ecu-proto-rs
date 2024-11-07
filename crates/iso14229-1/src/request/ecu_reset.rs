//! request of Service 11

use crate::request::{Request, SubFunction};
use crate::{Configuration, UdsError, Service, utils, ECUResetType};

pub(crate) fn ecu_reset(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = ECUResetType::try_from(sub_func.unwrap().function)?;

    let data_len = data.len();
    utils::data_length_check(data_len, 0, true)?;

    Ok(Request { service, sub_func, data })
}
