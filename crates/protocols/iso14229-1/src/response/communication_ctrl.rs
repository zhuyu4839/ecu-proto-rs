//! response of Service 28

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::response::{Code, Response, SubFunction};
use crate::{utils, CommunicationCtrlType, Configuration, UdsError, Service, ResponseData};

lazy_static!(
    pub static ref COMMUNICATION_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommunicationCtrl {
    pub data: Vec<u8>,  // should empty
}

impl ResponseData for CommunicationCtrl {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, UdsError> {
        match sub_func {
            Some(sub_func) => {
                let _ = CommunicationCtrlType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Response {
                    service: Service::CommunicationCtrl,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: vec![],
                })
            },
            None => Err(UdsError::SubFunctionError(Service::CommunicationCtrl))
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, UdsError> {
        let service = response.service;
        if service != Service::CommunicationCtrl
            || response.sub_func.is_none() {
            return Err(UdsError::ServiceError(service));
        }

        // let sub_func: CommunicationCtrlType = response.sub_function().unwrap().function()?;

        Ok(Self { data: response.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
