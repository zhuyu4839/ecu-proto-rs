use crate::error::Error;
use crate::service::{Configuration, MemoryLocation, Placeholder, RequestData, Service};

#[derive(Debug, Clone)]
pub struct ReadMemByAddrData(pub MemoryLocation);

impl<'a> TryFrom<&'a [u8]> for ReadMemByAddrData {
    type Error = Error;
    #[inline]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        // utils::data_length_check(data.len(), 3)?;
        Ok(Self(MemoryLocation::try_from(data)?))
    }
}

impl Into<Vec<u8>> for ReadMemByAddrData {
    fn into(self) -> Vec<u8> {
        self.0.into()
    }
}

impl RequestData for ReadMemByAddrData {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::{AddressAndLengthFormatIdentifier, MemoryLocation};
    use super::ReadMemByAddrData;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex!("2312481305").as_slice();
        let request = ReadMemByAddrData(
            MemoryLocation::new(AddressAndLengthFormatIdentifier::new(2, 1)?, 0x4813,0x05,)?);
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let request = ReadMemByAddrData::try_from(&source[1..])?;
        assert_eq!(request.0.memory_address(), 0x4813);
        assert_eq!(request.0.memory_size(), 0x05);

        let source = hex!("2324204813920103").as_slice();
        let request = ReadMemByAddrData(
            MemoryLocation::new(AddressAndLengthFormatIdentifier::new(4, 2)?,0x20481392,0x0103,)?);
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let request = ReadMemByAddrData::try_from(&source[1..])?;
        assert_eq!(request.0.memory_address(), 0x20481392);
        assert_eq!(request.0.memory_size(), 0x0103);

        Ok(())
    }
}

