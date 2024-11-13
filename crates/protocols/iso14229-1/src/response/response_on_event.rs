//! response of Service 86


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, error::UdsError, response::{Code, Response, SubFunction}, ResponseData, Service};

lazy_static!(
    pub static ref RESPONSE_ON_EVENT_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
);

#[derive(Debug, Clone)]
pub struct ResponseOnEvent {
    pub data: Vec<u8>,
}

#[allow(unused_variables)]
impl ResponseData for ResponseOnEvent {
    fn response(data: &[u8], sub_func: Option<u8>, cfg: &Configuration) -> Result<Response, UdsError> {
        match sub_func {
            Some(sub_func) => Err(UdsError::SubFunctionError(Service::ResponseOnEvent)),
            None => {

                Ok(Response {
                    service: Service::ResponseOnEvent,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(response: &Response, cfg: &Configuration) -> Result<Self, UdsError> {
        Err(UdsError::NotImplement)
    }

    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        self.data
    }
}