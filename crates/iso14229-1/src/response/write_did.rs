//! response of Service 2E

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{DataIdentifier, Error, response::Code, Service, utils};

lazy_static!(
    pub static ref WRITE_DID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::GeneralProgrammingFailure,
    ]);
);


pub struct WriteDID(pub DataIdentifier);

impl<'a> TryFrom<&'a [u8]> for WriteDID {
    type Error = Error;
    #[inline]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 2, true)?;
        let offset = 0;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );

        Ok(Self(did))
    }
}

impl Into<Vec<u8>> for WriteDID {
    #[inline]
    fn into(self) -> Vec<u8> {
        let did: u16 = self.0.into();

        did.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::DataIdentifier;
    use super::WriteDID;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex::decode("6EF190")?;
        let response = WriteDID(DataIdentifier::VIN);
        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let response = WriteDID::try_from(&source[1..])?;
        assert_eq!(response.0, DataIdentifier::VIN);

        Ok(())
    }
}




