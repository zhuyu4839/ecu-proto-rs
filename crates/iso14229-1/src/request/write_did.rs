//! request of Service 2E


use crate::{Configuration, DataIdentifier, DIDData, UdsError, request::{Request, SubFunction}, RequestData, Service, utils};

/// Service 2E
pub struct WriteDID(pub DIDData);

impl RequestData for WriteDID {
    fn request(data: &[u8], sub_func: Option<u8>, cfg: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::WriteDID)),
            None => {
                utils::data_length_check(data.len(), 3, false)?;
                let mut offset = 0;
                let did = DataIdentifier::from(
                    u16::from_be_bytes([data[offset], data[offset + 1]])
                );
                offset += 2;
                let &did_len = cfg.did_cfg.get(&did)
                    .ok_or(UdsError::DidNotSupported(did))?;

                utils::data_length_check(data.len(), offset + did_len, true)?;

                Ok(Request {
                    service: Service::WriteDID,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::WriteDID
            || request.sub_func.is_some() {
            return Err(UdsError::ServiceError(service))
        }

        let data = &request.data;
        let mut offset = 0;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        Ok(Self(DIDData { did, data: data[offset..].to_vec() }))
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.0.into()
    }
}
