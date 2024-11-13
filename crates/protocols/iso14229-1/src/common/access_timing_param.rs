//! Commons of Service 83

use crate::{Configuration, enum_extend, UdsError, RequestData, ResponseData, Service};

enum_extend!(
    pub enum TimingParameterAccessType {
        ReadExtendedTimingParameterSet = 0x01,
        SetTimingParametersToDefaultValues = 0x02,
        ReadCurrentlyActiveTimingParameters = 0x03,
        SetTimingParametersToGivenValues = 0x04,
    }, u8);
