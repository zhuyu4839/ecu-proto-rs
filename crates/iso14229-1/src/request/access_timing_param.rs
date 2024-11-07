//! request of Service 83

use crate::{UdsError, request::{Request, SubFunction}, Service, TimingParameterAccessType, TimingParameter, Configuration, RequestData};

pub(crate) fn access_timing_param(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let sf = TimingParameterAccessType::try_from(sub_func.unwrap().function)?;
    let _ = TimingParameter::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Request { service, sub_func, data })
}
