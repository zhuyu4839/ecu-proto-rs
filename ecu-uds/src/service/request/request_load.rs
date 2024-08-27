use crate::error::Error;
use crate::service::{Configuration, DataFormatIdentifier, MemoryLocation, Placeholder, RequestData};
use crate::utils;

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

