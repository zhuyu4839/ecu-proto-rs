//! Commons of Service 83

use crate::{Configuration, enum_extend, UdsError, RequestData, ResponseData, Service};

enum_extend!(
    pub enum TimingParameterAccessType {
        ReadExtendedTimingParameterSet = 0x01,
        SetTimingParametersToDefaultValues = 0x02,
        ReadCurrentlyActiveTimingParameters = 0x03,
        SetTimingParametersToGivenValues = 0x04,
    }, u8);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimingParameter(pub Vec<u8>);

impl Into<Vec<u8>> for TimingParameter {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl ResponseData for TimingParameter {
    type SubFunc = TimingParameterAccessType;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        match sub_func {
            Some(sub_func) => match sub_func {
                TimingParameterAccessType::ReadExtendedTimingParameterSet => {
                    if data.is_empty() {
                        return Err(UdsError::InvalidData(hex::encode(data)));
                    }

                    Ok(Self(data.to_vec()))
                }
                _ => {
                    if !data.is_empty() {
                        return Err(UdsError::InvalidData(hex::encode(data)));
                    }

                    Ok(Self(data.to_vec()))
                }
            },
            None => Err(UdsError::SubFunctionError(Service::AccessTimingParam)),
        }
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

impl RequestData for TimingParameter {
    type SubFunc = TimingParameterAccessType;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        match sub_func {
            Some(sub_func) => match sub_func {
                TimingParameterAccessType::SetTimingParametersToGivenValues => {
                    if data.is_empty() {
                        return Err(UdsError::InvalidData(hex::encode(data)));
                    }

                    Ok(Self(data.to_vec()))
                }
                _ => {
                    if !data.is_empty() {
                        return Err(UdsError::InvalidData(hex::encode(data)));
                    }

                    Ok(Self(data.to_vec()))
                }
            },
            None => Err(UdsError::SubFunctionError(Service::AccessTimingParam)),
        }
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
