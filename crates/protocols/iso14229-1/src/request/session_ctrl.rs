//! request of Service 10

use crate::{Configuration, Service, request::{Request, SubFunction}, RequestData, SessionType, UdsError, utils};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SessionCtrl {
    pub data: Vec<u8>,  // should empty
}

impl RequestData for SessionCtrl {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = SessionType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Request {
                    service: Service::SessionCtrl,
                    sub_func: Some(SubFunction::new(sub_func, Some(suppress_positive))),
                    data: data.to_vec(),
                })
            },
            None => Err(UdsError::SubFunctionError(Service::SessionCtrl)),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::SessionCtrl
            || request.sub_func.is_none() {
            return Err(UdsError::ServiceError(service))
        }

        Ok(Self { data: request.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}