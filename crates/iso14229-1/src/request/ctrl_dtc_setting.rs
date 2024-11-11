//! request of Service 85

use crate::{UdsError, request::{Request, SubFunction}, Service, Configuration, DTCSettingType, RequestData, utils};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CtrlDTCSetting {
    pub data: Vec<u8>,
}

impl RequestData for CtrlDTCSetting {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = DTCSettingType::try_from(sub_func)?;

                Ok(Request {
                    service: Service::CtrlDTCSetting,
                    sub_func: Some(SubFunction::new(sub_func, Some(suppress_positive))),
                    data: data.to_vec(),
                })
            },
            None => Err(UdsError::SubFunctionError(Service::CtrlDTCSetting)),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service;
        if service != Service::CtrlDTCSetting
            || request.sub_func.is_none() {
            return Err(UdsError::ServiceError(service));
        }

        // let sub_func: DTCSettingType = request.sub_function().unwrap().function()?;
        Ok(Self { data: request.data.clone() })
    }

    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
