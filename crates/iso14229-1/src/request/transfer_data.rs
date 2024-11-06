//! request of Service 36

use crate::{Error, request::{Request, SubFunction}, Service, TransferData, RequestData, Configuration};

pub(crate) fn transfer_data(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = TransferData::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}