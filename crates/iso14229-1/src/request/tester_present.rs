//! request of Service 3E

use crate::{UdsError, request::{Request, SubFunction}, Service, TesterPresentType, utils, Configuration};

pub(crate) fn tester_present(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = TesterPresentType::try_from(sub_func.unwrap().function)?;
    utils::data_length_check(data.len(), 0, true)?;

    Ok(Request { service, sub_func, data })
}