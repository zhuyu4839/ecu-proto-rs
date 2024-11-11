//! request of Service 34


use crate::{Configuration, DataFormatIdentifier, UdsError, MemoryLocation, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestDownload {
    pub dfi: DataFormatIdentifier,
    pub mem_loc: MemoryLocation,
}

impl RequestData for RequestDownload {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::RequestDownload)),
            None => {
                utils::data_length_check(data.len(), 2, false)?;

                Ok(Request { service: Service::RequestDownload, sub_func: None, data: data.to_vec(), })
            }
        }
    }

    fn try_parse(request: &Request, cfg: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::RequestDownload
            || request.sub_func.is_some() {
            return Err(UdsError::ServiceError(service))
        }

        let data = &request.data;
        let mut offset = 0;
        let dfi = DataFormatIdentifier(data[offset]);
        offset += 1;

        let mem_loc = MemoryLocation::from_slice(&data[offset..], cfg)?;

        Ok(Self { dfi, mem_loc })
    }

    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        let mut result = vec![self.dfi.0, ];
        result.append(&mut self.mem_loc.to_vec(cfg));

        result
    }
}
