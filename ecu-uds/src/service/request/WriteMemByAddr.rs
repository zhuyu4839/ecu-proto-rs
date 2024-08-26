
use crate::error::Error;
use crate::service::{AddressAndLengthFormatIdentifier, MemoryLocation};
use crate::utils;

#[derive(Debug, Clone)]
pub struct WriteMemByAddrData {
    pub(crate) mem_loc: MemoryLocation,
    pub(crate) data: Vec<u8>,
}

impl WriteMemByAddrData {
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

impl<'a> TryFrom<&'a [u8]> for  WriteMemByAddrData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 5, false)?;
        let mut offset = 0;
        let mem_loc = MemoryLocation::try_from(data)?;
        offset += mem_loc.len();
        let data = data[offset..].to_vec();

        Ok(Self { mem_loc, data })
    }
}

impl Into<Vec<u8>> for WriteMemByAddrData {
    fn into(mut self) -> Vec<u8> {
        let mut result: Vec<_> = self.mem_loc.into();
        result.append(&mut self.data);

        result
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::AddressAndLengthFormatIdentifier;
    use super::WriteMemByAddrData;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex!("3D4420481213051122334455").as_slice();
        let request = WriteMemByAddrData::new(
            AddressAndLengthFormatIdentifier::new(4, 4)?,
            0x20481213,
            0x05,
            hex!("1122334455").to_vec(),
        )?;
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let request = WriteMemByAddrData::try_from(&source[1..])?;
        assert_eq!(request.mem_loc.memory_address(), 0x20481213);
        assert_eq!(request.mem_loc.memory_size(), 0x05);
        assert_eq!(request.data, hex!("1122334455"));

        Ok(())
    }
}
