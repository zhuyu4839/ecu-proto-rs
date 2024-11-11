//! response of Service 37


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, UdsError, response::{Code, Response, SubFunction}, Service, ResponseData};

lazy_static!(
    pub static ref REQUEST_TRANSFER_EXIT_NEGATIVES: HashSet<Code>
    = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::GeneralProgrammingFailure,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestTransferExit {
    pub data: Vec<u8>,
}

impl ResponseData for RequestTransferExit {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::RequestTransferExit)),
            None => {
                
                Ok(Response {
                    service: Service::RequestTransferExit,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, UdsError> {
        let service = response.service();
        if service != Service::RequestTransferExit
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
