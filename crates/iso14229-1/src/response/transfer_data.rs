//! response of Service 36


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, Error, response::{Code, Response, SubFunction}, Service, utils};

lazy_static!(
    pub static ref TRANSFER_DATA_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::TransferDataSuspended,
        Code::GeneralProgrammingFailure,
        Code::WrongBlockSequenceCounter,
        Code::VoltageTooHigh,
        Code::VoltageTooLow,
    ]);
);

pub(crate) fn transfer_data(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Response, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    utils::data_length_check(data.len(), 1, false)?;

    Ok(Response { service, negative: false, sub_func, data })
}
