//! request of Service 23


use crate::{Configuration, UdsError, MemoryLocation, request::{Request, SubFunction}, RequestData, Service, utils};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadMemByAddr(pub MemoryLocation);

impl RequestData for ReadMemByAddr {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::ReadMemByAddr)),
            None => {
                utils::data_length_check(data.len(), 3, false)?;

                Ok(Request { service: Service::ReadMemByAddr, sub_func: None, data: data.to_vec() })
            }
        }
    }

    fn try_parse(request: &Request, cfg: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::ReadMemByAddr
            || request.sub_func.is_some() {
            return Err(UdsError::ServiceError(service))
        }

        let data = &request.data;
        Ok(Self(MemoryLocation::from_slice(data, cfg)?))
    }

    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        self.0.to_vec(cfg)
    }
}
