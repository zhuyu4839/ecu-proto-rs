//! response of Service 27


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, Error, response::{Code, Response, SubFunction}, SecurityAccessLevel, Service, ResponseData};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityAccess {
    pub key: Vec<u8>
}

impl ResponseData for SecurityAccess {
    type SubFunc = SecurityAccessLevel;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if sub_func.is_none() {
            return Err(Error::SubFunctionError(Service::SecurityAccess));
        }

        let level = sub_func.unwrap().0;
        if level % 2 != 0 {
            return Err(Error::InvalidParam(format!("Security access level: {}", level)));
        }

        if data.is_empty() {
            return Err(Error::InvalidParam("Security access response does not contain a security key".to_owned()));
        }

        Ok(Self { key: data.to_vec() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.key
    }
}

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
