//! response of Service 85

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{DTCSettingType, UdsError, response::{Code, Response, SubFunction}, Service, utils, Configuration, ResponseData};

lazy_static!(
    pub static ref CTRL_DTC_SETTING_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CtrlDTCSetting {
    pub data: Vec<u8>,  // should empty
}

impl ResponseData for CtrlDTCSetting {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, UdsError> {
        match sub_func {
            Some(sub_func) => {
                let _ = DTCSettingType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Response {
                    service: Service::CtrlDTCSetting,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: vec![],
                })
            },
            None => Err(UdsError::SubFunctionError(Service::CtrlDTCSetting)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, UdsError> {
        let service = response.service;
        if service != Service::CtrlDTCSetting
            || response.sub_func.is_none() {
            return Err(UdsError::ServiceError(service));
        }

        // let sub_func: DTCSettingType = request.sub_function().unwrap().function()?;
        Ok(Self { data: response.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
