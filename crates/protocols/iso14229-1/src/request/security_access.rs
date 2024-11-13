//! request of Service 27

use crate::{Configuration, UdsError, request::{Request, SubFunction}, Service, SecurityAccessLevel, RequestData};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityAccess {
    pub data: Vec<u8>
}

impl RequestData for SecurityAccess {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(level) => {
                Ok(Request {
                    service: Service::SecurityAccess,
                    sub_func: Some(SubFunction::new(level, None)),
                    data: data.to_vec(),
                })
            }
            None => Err(UdsError::SubFunctionError(Service::SecurityAccess)),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::SecurityAccess
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
