//! response of Service 14


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, UdsError, response::{Code, Response, SubFunction}, Service, utils, ResponseData};

lazy_static!(
    pub static ref CLEAR_DIAGNOSTIC_INFO_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        Code::GeneralProgrammingFailure,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClearDiagnosticInfo {
    pub data: Vec<u8>,  // should empty
}

impl ResponseData for ClearDiagnosticInfo {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::ClearDiagnosticInfo)),
            None => {
                utils::data_length_check(data.len(), 0, true)?;

                Ok(Response {
                    service: Service::ClearDiagnosticInfo,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, UdsError> {
        let service = response.service();
        if service != Service::ClearDiagnosticInfo
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
