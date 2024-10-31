//! Commons of Service 85

use crate::{Error, utils};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DTCSettingType {
    On = 0x01,
    Off = 0x02,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl TryFrom<u8> for DTCSettingType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::On),
            0x02 => Ok(Self::Off),
            0x03..=0x3F => Ok(Self::Reserved(value)),                               // ISOSAEReserved
            0x40..=0x5F => Ok(Self::VehicleManufacturerSpecific(value)),            // vehicleManufacturerSpecific
            0x60..=0x7E => Ok(Self::SystemSupplierSpecific(value)),                 // systemSupplierSpecific
            0x7F => Ok(Self::Reserved(value)),
            v => Err(Error::InvalidParam(utils::err_msg(v))),
        }
    }
}

impl Into<u8> for DTCSettingType {
    fn into(self) -> u8 {
        match self {
            Self::On => 0x01,
            Self::Off => 0x02,
            Self::VehicleManufacturerSpecific(v) => v,
            Self::SystemSupplierSpecific(v) => v,
            Self::Reserved(v) => v,
        }
    }
}
