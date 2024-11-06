//! response of Service 27


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, Error, response::{Code, Response, SubFunction}, SecurityAccessLevel, Service};

lazy_static!(
    pub static ref SECURITY_ACCESS_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::InvalidKey,
        Code::ExceedNumberOfAttempts,
        Code::RequiredTimeDelayNotExpired,
    ]);
);

pub(crate) fn security_access(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Response, Error> {
    if sub_func.is_none() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = SecurityAccessLevel::try_from(sub_func.unwrap().0)?;

    Ok(Response { service, negative: false, sub_func, data })
}
