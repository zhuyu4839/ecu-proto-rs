use crate::error::Error;
use crate::service::{Configuration, DataFormatIdentifier, MemoryLocation, Placeholder, RequestData};
use crate::utils;

#[derive(Debug, Clone)]
pub struct RequestLoadData {
    pub dfi: DataFormatIdentifier,
    pub mem_loc: MemoryLocation,
}

impl<'a> TryFrom<&'a [u8]> for RequestLoadData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 2, false)?;

        let mut offset = 0;
        let dfi = DataFormatIdentifier(data[offset]);
        offset += 1;

        let mem_loc = MemoryLocation::try_from(&data[offset..])?;

        Ok(Self { dfi, mem_loc })
    }
}

impl Into<Vec<u8>> for RequestLoadData {
    #[inline]
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.dfi.0, ];
        result.append(&mut self.mem_loc.into());

        result
    }
}

impl RequestData for RequestLoadData {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
}

