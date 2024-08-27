/// Commons of Service 83

use crate::enum_to_vec;
use crate::error::Error;
use crate::service::{Configuration, RequestData, ResponseData};

enum_to_vec!(
    pub enum TimingParameterAccessType {
        ReadExtendedTimingParameterSet = 0x01,
        SetTimingParametersToDefaultValues = 0x02,
        ReadCurrentlyActiveTimingParameters = 0x03,
        SetTimingParametersToGivenValues = 0x04,
    }, u8, Error, InvalidParam
);

#[derive(Debug, Clone)]
pub struct TimingParameter(pub Vec<u8>);

impl Into<Vec<u8>> for TimingParameter {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl<'a> TryFrom<&'a [u8]> for TimingParameter {
    type Error = Error;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.to_vec()))
    }
}

impl ResponseData for TimingParameter {
    type SubFunc = TimingParameterAccessType;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

impl RequestData for TimingParameter {
    type SubFunc = TimingParameterAccessType;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
