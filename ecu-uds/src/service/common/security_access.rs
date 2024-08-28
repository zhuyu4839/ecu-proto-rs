//! Commons of Service 27

use crate::error::Error;
use crate::service::{Configuration, Placeholder, RequestData, ResponseData};
use crate::utils;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SecurityAccessLevel(u8);

impl SecurityAccessLevel {
    pub fn new(level: u8) -> Result<Self, Error> {
        if level < 1 || level > 0x7D {
            return Err(Error::InvalidParam(format!("access level: {}", level)));
        }

        Ok(Self(level))
    }
}

impl TryFrom<u8> for SecurityAccessLevel {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Into<u8> for SecurityAccessLevel {
    fn into(self) -> u8 {
        self.0
    }
}

/// Table 42 — Request message SubFunction parameter definition
#[derive(Debug, Clone)]
pub struct SecurityAccessData(pub Vec<u8>);

impl<'a> TryFrom<&'a [u8]> for SecurityAccessData {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.to_vec()))
    }
}

impl Into<Vec<u8>> for SecurityAccessData {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl RequestData for SecurityAccessData {
    type SubFunc = SecurityAccessLevel;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

impl ResponseData for SecurityAccessData {
    type SubFunc = SecurityAccessLevel;

    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
