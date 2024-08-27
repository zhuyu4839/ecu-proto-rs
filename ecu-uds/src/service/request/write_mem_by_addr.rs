
use crate::error::Error;
use crate::service::{AddressAndLengthFormatIdentifier, Configuration, MemoryLocation, Placeholder, RequestData};
use crate::utils;

#[derive(Debug, Clone)]
pub struct WriteMemByAddr {
    pub(crate) mem_loc: MemoryLocation,
    pub(crate) data: Vec<u8>,
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

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::{AddressAndLengthFormatIdentifier, Configuration, RequestData};
    use super::WriteMemByAddr;

    #[test]
    fn new() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex!("3D4420481213000000051122334455").as_slice();
        let request = WriteMemByAddr::new(
            AddressAndLengthFormatIdentifier::new(4, 4)?,
            0x20481213,
            0x05,
            hex!("1122334455").to_vec(),
        )?;
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[1..].to_vec());

        let request = WriteMemByAddr::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(request.mem_loc.memory_address(), 0x20481213);
        assert_eq!(request.mem_loc.memory_size(), 0x05);
        assert_eq!(request.data, hex!("1122334455"));

        Ok(())
    }
}
