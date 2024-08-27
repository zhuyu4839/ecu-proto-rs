use crate::error::Error;
use crate::service::{Configuration, DataIdentifier, Placeholder, RequestData};
use crate::utils;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadScalingDID(pub DataIdentifier);

impl<'a> TryFrom<&'a [u8]> for ReadScalingDID {
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

impl Into<Vec<u8>> for ReadScalingDID {
    fn into(self) -> Vec<u8> {
        let did: u16 = self.0.into();
        did.to_be_bytes().to_vec()
    }
}

impl RequestData for ReadScalingDID {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
