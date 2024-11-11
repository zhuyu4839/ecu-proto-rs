//! request of Service 24


use crate::{Configuration, DataIdentifier, UdsError, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadScalingDID(pub DataIdentifier);

impl RequestData for ReadScalingDID {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::ReadScalingDID)),
            None => {
                utils::data_length_check(data.len(), 2, true)?;

                Ok(Request { service: Service::ReadScalingDID, sub_func: None, data: data.to_vec(), })
            }
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::ReadScalingDID
            || request.sub_func.is_some() {
            return Err(UdsError::ServiceError(service))
        }

        let data = &request.data;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[0], data[1]])
        );

        Ok(Self(did))
    }

    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        let did: u16 = self.0.into();
        did.to_be_bytes().to_vec()
    }
}
