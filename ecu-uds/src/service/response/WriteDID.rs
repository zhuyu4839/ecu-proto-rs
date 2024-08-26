/// Service 2E

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::service::{DataIdentifier, Service};
use crate::service::response::Code;
use crate::utils;

lazy_static!(
    pub static ref WRITE_DID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::GeneralProgrammingFailure,
    ]);
);


pub struct WriteDIDData(pub DataIdentifier);

impl<'a> TryFrom<&'a [u8]> for WriteDIDData {
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

impl Into<Vec<u8>> for WriteDIDData {
    #[inline]
    fn into(self) -> Vec<u8> {
        let mut result = vec![utils::positive(Service::WriteDID), ];
        let did: u16 = self.0.into();
        result.extend(did.to_be_bytes());

        result
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::DataIdentifier;
    use super::WriteDIDData;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex!("6EF190").as_slice();
        let response = WriteDIDData(DataIdentifier::VIN);
        let result: Vec<_> = response.into();
        assert_eq!(result, source);

        let response = WriteDIDData::try_from(&source[1..])?;
        assert_eq!(response.0, DataIdentifier::VIN);

        Ok(())
    }
}




