//! response of Service 87

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{utils, Configuration, UdsError, LinkCtrlType, response::{Code, Response, SubFunction}, Service, ResponseData};

lazy_static!(
    pub static ref LINK_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinkCtrl {
    pub data: Vec<u8>,  // should empty
}

impl ResponseData for LinkCtrl {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, UdsError> {
        match sub_func {
            Some(sub_func) => {
                let _ = LinkCtrlType::try_from(sub_func)?;
                let data_len = data.len();
                utils::data_length_check(data_len, 0, true)?;

                Ok(Response {
                    service: Service::LinkCtrl,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            },
            None => Err(UdsError::SubFunctionError(Service::LinkCtrl)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, UdsError> {
        let service = response.service();
        if service != Service::LinkCtrl
            || response.sub_func.is_none() {
            return Err(UdsError::ServiceError(service))
        }

        // let sub_func: LinkCtrlType = response.sub_function().unwrap().function()?;
        Ok(Self { data: response.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
