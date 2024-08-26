use crate::error::Error;
use crate::service::{DataIdentifier, DIDData, Placeholder, RequestData, Service, Configuration};
use crate::utils;

/// Service 2E
pub struct WriteDIDData(pub DIDData);

impl<'a> TryFrom<&'a [u8]> for WriteDIDData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 3, false)?;
        let mut offset = 0;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        Ok(Self(DIDData { did, data: data[offset..].to_vec() }))
    }
}

impl Into<Vec<u8>> for WriteDIDData {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.0.into()
    }
}

impl RequestData for WriteDIDData {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::{DataIdentifier, DIDData};
    use super::WriteDIDData;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex!("2ef1904441564443313030394e544c5036313338").as_slice();
        let request = WriteDIDData(
            DIDData {
                did: DataIdentifier::VIN,
                data: source[3..].to_vec(),  // 17 bytes
            }
        );
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let request = WriteDIDData::try_from(&source[1..])?;
        assert_eq!(request.0.did, DataIdentifier::VIN);
        assert_eq!(request.0.data, hex!("4441564443313030394e544c5036313338"));

        Ok(())
    }
}
