//! response of Service 27


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, Iso14229Error, response::{Code, Response, SubFunction}, SecurityAccessLevel, Service, ResponseData};

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
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(level) => {
                if level % 2 != 0
                    && data.is_empty() {
                    return Err(Iso14229Error::InvalidParam("Security access response does not contain a security key".to_owned()));
                }

                Ok(Response {
                    service: Service::SecurityAccess,
                    negative: false,
                    sub_func: Some(SubFunction::new(level)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::SecurityAccess)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::SecurityAccess
            || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service))
        }

        Ok(Self { key: response.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.key
    }
}
