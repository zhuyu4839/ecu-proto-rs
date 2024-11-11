//! request of Service 36

use crate::{UdsError, request::{Request, SubFunction}, Service, RequestData, Configuration, utils, SessionType};

#[derive(Debug, Clone)]
pub struct TransferData {
    pub sequence: u8,
    pub data: Vec<u8>,
}

impl RequestData for TransferData {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::TransferData)),
            None => {
                utils::data_length_check(data.len(), 1, false)?;

                Ok(Request { service: Service::TransferData, sub_func: None, data: data.to_vec(), })
            },
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::TransferData
            || request.sub_func.is_some() {
            return Err(UdsError::ServiceError(service))
        }

        let data = &request.data;
        let mut offset = 0;
        let sequence = data[offset];
        offset += 1;

        Ok(Self { sequence, data: data[offset..].to_vec() })
    }

    #[inline]
    fn to_vec(mut self, _: &Configuration) -> Vec<u8> {
        let mut result = vec![self.sequence];
        result.append(&mut self.data);
        result
    }
}
