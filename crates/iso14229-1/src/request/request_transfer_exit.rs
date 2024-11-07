//! request of Service 37

use crate::{UdsError, request::{Request, SubFunction}, Service, utils, Configuration};

pub(crate) fn request_transfer_exit(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    Ok(Request { service, sub_func, data })
}