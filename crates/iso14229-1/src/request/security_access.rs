//! request of Service 27

use crate::{Configuration, Error, request::{Request, SubFunction}, Service, SecurityAccessLevel};

pub(crate) fn security_access(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_none() {
        return Err(Error::SubFunctionError(service));
    }

    let level = sub_func.unwrap().function;
    let _ = SecurityAccessLevel::try_from(level)?;

    Ok(Request { service, sub_func, data })
}