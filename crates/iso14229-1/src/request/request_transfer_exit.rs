//! request of Service 37

use crate::{Error, request::{Request, SubFunction}, Service, utils, Configuration};

pub(crate) fn request_transfer_exit(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    Ok(Request { service, sub_func, data })
}