//! request of Service 11

use crate::{Configuration, ECUResetType, request::{Request, SubFunction}, RequestData, Service, UdsError, utils};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ECUReset {
    pub data: Vec<u8>,  // should empty
}

impl RequestData for ECUReset {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = ECUResetType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Request {
                    service: Service::ECUReset,
                    sub_func: Some(SubFunction::new(sub_func, Some(suppress_positive))),
                    data: data.to_vec(),
                })
            },
            None => Err(UdsError::SubFunctionError(Service::ECUReset)),
        }
    }
    
    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::ECUReset
            || request.sub_func.is_none() {
            return Err(UdsError::ServiceError(service))
        }

        Ok(Self { data: request.data.clone() })
    }

    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
