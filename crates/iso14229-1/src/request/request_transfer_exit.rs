//! request of Service 37

use crate::{UdsError, request::{Request, SubFunction}, Service, utils, Configuration, RequestData};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestTransferExit {
    pub data: Vec<u8>,
}

impl RequestData for RequestTransferExit {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::RequestTransferExit)),
            None => {
                // utils::data_length_check(data.len(), 0, true)?;

                Ok(Request {
                    service: Service::RequestTransferExit,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::RequestTransferExit
            || request.sub_func.is_some() {
            return Err(UdsError::ServiceError(service))
        }

        Ok(Self { data: request.data.clone() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
