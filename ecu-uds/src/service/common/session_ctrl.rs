//! Commons of Service 10


use crate::error::Error;
use crate::utils;

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum SessionType {
    #[default]
    Default = 0x01,
    Programming = 0x02,
    Extended = 0x03,
    SafetySystemDiagnostic = 0x04,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl TryFrom<u8> for SessionType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Default),
            0x02 => Ok(Self::Programming),
            0x03 => Ok(Self::Extended),
            0x04 => Ok(Self::SafetySystemDiagnostic),
            0x05..=0x3F => Ok(Self::Reserved(value)),
            0x40..=0x5F => Ok(Self::VehicleManufacturerSpecific(value)),
            0x60..=0x7E => Ok(Self::SystemSupplierSpecific(value)),
            0x7F => Ok(Self::Reserved(value)),
            v => Err(Error::InvalidParam(utils::err_msg(v))),
        }
    }
}

impl Into<u8> for SessionType {
    fn into(self) -> u8 {
        match self {
            Self::Default => 0x01,
            Self::Programming => 0x02,
            Self::Extended => 0x03,
            Self::SafetySystemDiagnostic => 0x04,
            Self::VehicleManufacturerSpecific(v) => v,
            Self::SystemSupplierSpecific(v) => v,
            Self::Reserved(v) => v,
        }
    }
}
