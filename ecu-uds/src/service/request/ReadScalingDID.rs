use crate::error::Error;
use crate::service::DataIdentifier;
use crate::utils;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadScalingDIDData(pub DataIdentifier);

impl<'a> TryFrom<&'a [u8]> for ReadScalingDIDData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 2, true)?;

        let did = DataIdentifier::from(
            u16::from_be_bytes(
                data.try_into()
                .map_err(|_| Error::InvalidData(utils::hex_slice_to_string(data)))?
            )
        );

        Ok(Self(did))
    }
}

impl Into<Vec<u8>> for ReadScalingDIDData {
    fn into(self) -> Vec<u8> {
        let did: u16 = self.0.into();
        did.to_be_bytes().to_vec()
    }
}
