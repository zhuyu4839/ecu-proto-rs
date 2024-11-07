//! request of Service 10

use crate::request::{Request, SubFunction};
use crate::{Configuration, UdsError, Service};

pub(crate) fn session_ctrl(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let data_len = data.len();
    if data_len != 0 {
        return Err(UdsError::InvalidData(hex::encode(data)));
    }

    Ok(Request { service, sub_func, data })
}
