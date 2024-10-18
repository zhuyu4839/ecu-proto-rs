//! response of Service 10


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::docan::constant::{P2_MAX, P2_STAR_MAX, P2_STAR_MAX_MS};
use crate::error::Error;
use crate::service::response::Code;
use crate::service::{Configuration, ResponseData, SessionType};
use crate::utils;

lazy_static!(
    pub static ref SESSION_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
    ]);
);

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SessionTiming {
    pub(crate) p2:      u16,
    pub(crate) p2_star: u16,
}

impl SessionTiming {
    #[inline]
    pub fn new(
        p2_ms: u16,
        p2_star_ms: u32,
    ) -> Result<Self, Error> {
        if p2_ms > P2_MAX || p2_star_ms > P2_STAR_MAX_MS {
            return Err(Error::InvalidData(format!("P2: {} or P2*: {}", p2_ms, p2_star_ms)));
        }
        let p2_star = (p2_star_ms / 10) as u16;
        Ok(Self { p2: p2_ms, p2_star })
    }

    #[inline]
    pub fn p2_ms(&self) -> u16 {
        self.p2
    }

    #[inline]
    pub fn p2_star_ms(&self) -> u32 {
        self.p2_star as u32 * 10
    }
}

impl<'a> TryFrom<&'a [u8]> for SessionTiming {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 4, true)?;

        let mut offset = 0;

        let p2 = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let p2_star = u16::from_be_bytes([data[offset], data[offset + 1]]);
        if p2 > P2_MAX || p2_star > P2_STAR_MAX {
            #[cfg(not(feature = "session_data_check"))]
            log::warn!("UDS - invalid session data P2: {}, P2*: {}", p2, p2_star);
            #[cfg(feature = "session_data_check")]
            return Err(Error::InvalidSessionData(format!("P2: {}, P2*: {}", p2, p2_star)));
        }

        Ok(Self { p2, p2_star })
    }
}

impl Into<Vec<u8>> for SessionTiming {
    fn into(self) -> Vec<u8> {
        let mut result = self.p2.to_be_bytes().to_vec();
        result.extend(self.p2_star.to_be_bytes());
        result
    }
}

impl ResponseData for SessionTiming {
    type SubFunc = SessionType;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
