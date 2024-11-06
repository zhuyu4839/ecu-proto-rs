//! request of Service 10

use crate::request::{Request, SubFunction};
use crate::{Configuration, Error, Service};

pub(crate) fn session_ctrl(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_none() {
        return Err(Error::SubFunctionError(service));
    }

    let data_len = data.len();
    if data_len != 0 {
        return Err(Error::InvalidData(hex::encode(data)));
    }

    Ok(Request { service, sub_func, data })
}
