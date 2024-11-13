//! response of Service 83

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{UdsError, response::{Code, Response, SubFunction}, Service, TimingParameterAccessType, Configuration, ResponseData};

lazy_static!(
    pub static ref ACCESS_TIMING_PARAM_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccessTimingParameter {
    pub data: Vec<u8>,
}

impl ResponseData for AccessTimingParameter {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, UdsError> {
        match sub_func {
            Some(sub_func) => {
                match TimingParameterAccessType::try_from(sub_func)? {
                    TimingParameterAccessType::ReadExtendedTimingParameterSet => {
                        if data.is_empty() {
                            Err(UdsError::InvalidData(hex::encode(data)))
                        }
                        else {
                            Ok(())
                        }
                    }
                    _ => {
                        if !data.is_empty() {
                            Err(UdsError::InvalidData(hex::encode(data)))
                        }
                        else {
                            Ok(())
                        }
                    }
                }?;

                Ok(Response {
                    service: Service::AccessTimingParam,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            },
            None => Err(UdsError::SubFunctionError(Service::AccessTimingParam)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, UdsError> {
        let service = response.service();
        if service != Service::AccessTimingParam
            || response.sub_func.is_none() {
            return Err(UdsError::ServiceError(service))
        }

        Ok(Self { data: response.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
