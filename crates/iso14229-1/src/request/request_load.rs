//! request of Service 34|35


use crate::{Configuration, DataFormatIdentifier, Error, MemoryLocation, Placeholder, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone)]
pub struct RequestLoadData {
    pub dfi: DataFormatIdentifier,
    pub mem_loc: MemoryLocation,
}

impl RequestData for RequestLoadData {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        utils::data_length_check(data.len(), 2, false)?;

        let mut offset = 0;
        let dfi = DataFormatIdentifier(data[offset]);
        offset += 1;

        let mem_loc = MemoryLocation::from_slice(&data[offset..], cfg)?;

        Ok(Self { dfi, mem_loc })
    }
    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        let mut result = vec![self.dfi.0, ];
        result.append(&mut self.mem_loc.to_vec(cfg));

        result
    }
}

pub(crate) fn request_download(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = RequestLoadData::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}

pub(crate) fn request_upload(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = RequestLoadData::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}

