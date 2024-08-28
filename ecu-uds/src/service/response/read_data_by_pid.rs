//! response of Service 2A


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::service::response::Code;
use crate::service::{Configuration, Placeholder, ResponseData};
use crate::utils;

lazy_static!(
    pub static ref READ_DATA_BY_PERIOD_ID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ]);
);

#[derive(Debug, Clone)]
pub struct ReadByPeriodIdData {
    pub did: u8,
    pub record: Vec<u8>,
}

impl<'a> TryFrom<&'a [u8]> for ReadByPeriodIdData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;

        let did = data[offset];
        offset += 1;
        let record = data[offset..].to_vec();

        Ok(Self { did, record })
    }
}

impl Into<Vec<u8>> for ReadByPeriodIdData {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.did];
        result.append(&mut self.record);

        result
    }
}

impl ResponseData for ReadByPeriodIdData {
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

