//! request of Service 23


use crate::error::Error;
use crate::service::{Configuration, MemoryLocation, Placeholder, RequestData, Service};

#[derive(Debug, Clone)]
pub struct ReadMemByAddr(pub MemoryLocation);

impl RequestData for ReadMemByAddr {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        Ok(Self(MemoryLocation::from_slice(data, cfg)?))
    }
    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        self.0.to_vec(cfg)
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::{AddressAndLengthFormatIdentifier, Configuration, MemoryLocation, RequestData};
    use super::ReadMemByAddr;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex!("2312481305").as_slice();
        let cfg = Configuration::default();
        let request = ReadMemByAddr(
            MemoryLocation::new(AddressAndLengthFormatIdentifier::new(2, 1)?, 0x4813,0x05,)?);
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[1..].to_vec());

        let cfg = Configuration::default();
        let request = ReadMemByAddr::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(request.0.memory_address(), 0x4813);
        assert_eq!(request.0.memory_size(), 0x05);

        let source = hex!("2324204813920103").as_slice();
        let request = ReadMemByAddr(
            MemoryLocation::new(AddressAndLengthFormatIdentifier::new(4, 2)?,0x20481392,0x0103,)?);
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[1..].to_vec());

        let request = ReadMemByAddr::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(request.0.memory_address(), 0x20481392);
        assert_eq!(request.0.memory_size(), 0x0103);

        Ok(())
    }
}

