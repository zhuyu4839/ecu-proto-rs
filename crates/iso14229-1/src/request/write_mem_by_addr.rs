//! request of Service 3D


use crate::{AddressAndLengthFormatIdentifier, Configuration, Error, MemoryLocation, Placeholder, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone)]
pub struct WriteMemByAddr {
    pub mem_loc: MemoryLocation,
    pub data: Vec<u8>,
}

impl WriteMemByAddr {
    #[inline]
    pub fn new(
        alfi: AddressAndLengthFormatIdentifier,
        mem_addr: u128,
        mem_size: u128,
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        if data.len() != mem_size as usize {
            return Err(Error::InvalidParam("the length of data must be equal to mem_size and the mem_size must rather than 0".to_string()));
        }

        Ok(Self {
            mem_loc: MemoryLocation::new(alfi, mem_addr, mem_size)?,
            data,
        })
    }

    #[inline]
    pub fn memory_location(&self) -> &MemoryLocation {
        &self.mem_loc
    }

    #[inline]
    pub fn data_record(&self) -> &Vec<u8> {
        &self.data
    }
}

impl RequestData for WriteMemByAddr {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        utils::data_length_check(data.len(), 5, false)?;
        let mut offset = 0;
        let mem_loc = MemoryLocation::from_slice(data, cfg)?;
        offset += mem_loc.len();
        let data = data[offset..].to_vec();

        Ok(Self { mem_loc, data })
    }

    fn to_vec(mut self, cfg: &Configuration) -> Vec<u8> {
        let mut result: Vec<_> = self.mem_loc.to_vec(cfg);
        result.append(&mut self.data);

        result
    }
}

pub(crate) fn write_mem_by_addr(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = WriteMemByAddr::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}
