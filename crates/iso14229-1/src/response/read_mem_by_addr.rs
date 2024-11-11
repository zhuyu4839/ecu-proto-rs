//! response of Service 23


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, UdsError, response::{Code, Response, SubFunction}, Service, ResponseData};

lazy_static!(
    pub static ref READ_MEM_BY_ADDR_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ResponseTooLong,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadMemByAddr {
    pub data: Vec<u8>,
}

impl ResponseData for ReadMemByAddr {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::ReadMemByAddr)),
            None => {

                Ok(Response {
                    service: Service::ReadMemByAddr,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, UdsError> {
        let service = response.service();
        if service != Service::ReadMemByAddr
            || response.sub_func.is_some() {
            return Err(UdsError::ServiceError(service))
        }

        Ok(Self { data: response.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
