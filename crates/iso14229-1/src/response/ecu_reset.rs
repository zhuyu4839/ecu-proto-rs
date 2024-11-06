//! response of Service 11

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, ECUResetType, Error, response::{Code, Response, SubFunction}, ResponseData, Service, utils};

lazy_static!(
    pub static ref ECU_RESET_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::AuthenticationRequired,
    ]);
);

/// only sub-function is 0x04
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
            1 => Ok(Self(Some(data[0]))),
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
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        let data_len = data.len();
        match sub_func {
            Some(sub_func) => match sub_func {
                ECUResetType::EnableRapidPowerShutDown => utils::data_length_check(data_len, 1, true)?,
                ECUResetType::VehicleManufacturerSpecific(_) => {}
                ECUResetType::SystemSupplierSpecific(_) => {}
                ECUResetType::Reserved(_) => {},
                _ => utils::data_length_check(data_len, 0, true)?,
            },
            None => Err(Error::SubFunctionError(Service::ECUReset))?,
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn ecu_reset(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, Error> {
    if sub_func.is_none() {
        return Err(Error::SubFunctionError(service));
    }

    let sf = ECUResetType::try_from(sub_func.unwrap().0)?;
    let _ = PowerDownSeconds::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
