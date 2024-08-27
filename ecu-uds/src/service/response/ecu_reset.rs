/// Service 11

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::service::response::Code;
use crate::service::{Configuration, ECUResetType, ResponseData};

lazy_static!(
    pub static ref ECU_RESET_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::AuthenticationRequired,
    ]);
);

#[derive(Debug, Default, Clone)]
pub struct PowerDownSeconds(Option<u8>);

impl PowerDownSeconds {
    #[inline]
    pub fn new(seconds: Option<u8>) -> Self {
        Self(seconds)
    }

    #[inline]
    pub fn seconds(&self) -> Option<u8> {
        self.0
    }
}

impl<'a> TryFrom<&'a [u8]> for PowerDownSeconds {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        match data.len() {
            0 => Ok(Self(None)),
            1 => Ok(Self(Some(data[1]))),
            actual => Err(Error::InvalidDataLength { expect: 1, actual }),
        }
    }
}

impl Into<Vec<u8>> for PowerDownSeconds {
    fn into(self) -> Vec<u8> {
        match self.0 {
            Some(v) => vec![v, ],
            None => vec![],
        }
    }
}

impl ResponseData for PowerDownSeconds {
    type SubFunc = ECUResetType;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
