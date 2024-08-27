/// Commons of Service 11

use crate::error::Error;
use crate::utils;

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ECUResetType {
    #[default]
    HardReset = 1,
    KeyOffOnReset = 2,
    SoftReset = 3,
    EnableRapidPowerShutDown = 4,
    DisableRapidPowerShutDown = 5,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl TryFrom<u8> for ECUResetType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::HardReset),
            0x02 => Ok(Self::KeyOffOnReset),
            0x03 => Ok(Self::SoftReset),
            0x04 => Ok(Self::EnableRapidPowerShutDown),
            0x05 => Ok(Self::DisableRapidPowerShutDown),
            0x06..=0x3F => Ok(Self::Reserved(value)),
            0x40..=0x5F => Ok(Self::VehicleManufacturerSpecific(value)),
            0x60..=0x7E => Ok(Self::SystemSupplierSpecific(value)),
            0x7F => Ok(Self::Reserved(value)),
            v => Err(Error::InvalidParam(utils::err_msg(v))),
        }
    }
}

impl Into<u8> for ECUResetType {
    fn into(self) -> u8 {
        match self {
            Self::HardReset => 0x01,
            Self::KeyOffOnReset => 0x02,
            Self::SoftReset => 0x03,
            Self::EnableRapidPowerShutDown => 0x04,
            Self::DisableRapidPowerShutDown => 0x05,
            Self::VehicleManufacturerSpecific(v) => v,
            Self::SystemSupplierSpecific(v) => v,
            Self::Reserved(v) => v,
        }
    }
}

