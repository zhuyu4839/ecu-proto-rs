//! request of Service 34


use crate::{Configuration, DataFormatIdentifier, Iso14229Error, MemoryLocation, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestDownload {
    pub dfi: DataFormatIdentifier,
    pub mem_loc: MemoryLocation,
}

impl RequestData for RequestDownload {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::RequestDownload)),
            None => {
                utils::data_length_check(data.len(), 2, false)?;

                Ok(Request { service: Service::RequestDownload, sub_func: None, data: data.to_vec(), })
            }
        }
    }

    fn try_parse(request: &Request, cfg: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::RequestDownload
            || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service))
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
