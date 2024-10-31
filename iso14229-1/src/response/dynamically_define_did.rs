//! response of Service 2C

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, DefinitionType, DynamicallyDID, Error, Placeholder, response::Code, ResponseData};

lazy_static!(
    pub static ref DYNAMICAL_DID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ]);
);

#[derive(Debug, Clone)]
pub struct DynamicallyDefineDID(pub Option<DynamicallyDID>);

impl<'a> TryFrom<&'a [u8]> for DynamicallyDefineDID {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let offset = 0;

        let dynamic = match data_len {
            0 => Ok(None),
            2 => Ok(Some(DynamicallyDID::try_from(
                u16::from_be_bytes([data[offset], data[offset + 1]])
            )?)),
            v => Err(Error::InvalidDataLength { expect: 2, actual: v })
        }?;

        Ok(Self(dynamic))
    }
}

impl Into<Vec<u8>> for DynamicallyDefineDID {
    fn into(self) -> Vec<u8> {
        match self.0 {
            Some(v) => v.into(),
            None => vec![],
        }
    }
}

impl ResponseData for DynamicallyDefineDID {
    type SubFunc = DefinitionType;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
