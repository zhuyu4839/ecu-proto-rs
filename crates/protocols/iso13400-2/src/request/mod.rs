use derive_getters::Getters;
use crate::{constant::*, DoIpError, LogicAddress, RoutingActiveType, utils};

/****** --- UDP --- ********/
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VehicleID;   // 0x0001

impl VehicleID {
    #[inline]
    const fn length() -> usize {
        0
    }
}

impl TryFrom<&[u8]> for VehicleID {
    type Error = DoIpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;

        Ok(Self)
    }
}

impl Into<Vec<u8>> for VehicleID {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_VEHICLE_IDENTIFIER.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Getters)]
pub struct VehicleIDWithEID {    // 0x0002
    pub(crate) eid: [u8; SIZE_OF_ID],
}

impl VehicleIDWithEID {
    pub fn new(eid: [u8; SIZE_OF_ID]) -> Self {
        Self { eid }
    }

    #[inline]
    const fn length() -> usize {
        SIZE_OF_ID
    }
}

impl TryFrom<&[u8]> for VehicleIDWithEID {
    type Error = DoIpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, offset) = utils::data_len_check(data, Self::length(), true)?;
        let eid: [u8; SIZE_OF_ID] = data[offset..offset+Self::length()].try_into().unwrap();

        Ok(Self { eid })
    }
}

impl Into<Vec<u8>> for VehicleIDWithEID {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_VEHICLE_ID_WITH_EID.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());
        result.extend(self.eid);

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Getters)]
pub struct VehicleIDWithVIN {     // 0x0003
    pub(crate) vin: String,
}

impl VehicleIDWithVIN {
    #[must_use]
    pub fn new(vin: String) -> Result<Self, DoIpError> {
        let vin_len = vin.as_bytes().len();
        if vin_len != Self::length() {
            return Err(DoIpError::InvalidLength { actual: vin_len, expected: Self::length() });
        }

        Ok(Self { vin })
    }

    #[inline]
    const fn length() -> usize {
        LENGTH_OF_VIN
    }
}

impl TryFrom<&[u8]> for VehicleIDWithVIN {
    type Error = DoIpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, offset) = utils::data_len_check(data, Self::length(), true)?;
        let vin = match String::from_utf8(data[offset..].to_vec()) {
            Ok(v) => v,
            Err(_) => {
                log::warn!("invalid UTF-8 string: {}", hex::encode(data));
                "-".repeat(Self::length())
            }
        };

        Ok(Self { vin })
    }
}

impl Into<Vec<u8>> for VehicleIDWithVIN {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_VEHICLE_ID_WITH_VIN.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());
        result.append(&mut self.vin.as_bytes().to_vec());

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntityStatus;   // 0x4001

impl EntityStatus {
    #[inline]
    const fn length() -> usize {
        0
    }
}

impl TryFrom<&[u8]> for EntityStatus {
    type Error = DoIpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;

        Ok(Self)
    }
}

impl Into<Vec<u8>> for EntityStatus {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_ENTITY_STATUS.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DiagnosticPowerMode;    // 0x4003

impl DiagnosticPowerMode {
    #[inline]
    const fn length() -> usize {
        0
    }
}

impl TryFrom<&[u8]> for DiagnosticPowerMode {
    type Error = DoIpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;

        Ok(Self)
    }
}

impl Into<Vec<u8>> for DiagnosticPowerMode {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_DIAGNOSTIC_POWER_MODE.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());

        result
    }
}

/****** --- end of UDP --- ********/

/****** --- TCP --- ********/
#[derive(Debug, Clone, Eq, PartialEq, Getters)]
pub struct RoutingActive {  // 0x0005
    pub(crate) src_addr: LogicAddress,
    pub(crate) active: RoutingActiveType,
    pub(crate) reserved: u32,
    pub(crate) user_def: Option<u32>,
}

impl RoutingActive {
    pub fn new(
        src_addr: LogicAddress,
        active: RoutingActiveType,
        user_def: Option<u32>,
    ) -> Self {
        Self { src_addr, active, reserved: Default::default(), user_def }
    }

    /// min length
    #[inline]
    const fn length() -> usize {
        SIZE_OF_ADDRESS + 1 + 4
    }
}

impl TryFrom<&[u8]> for RoutingActive {
    type Error = DoIpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (data_len, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let src_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let src_addr = LogicAddress::from(src_addr);
        let active = data[offset];
        offset += 1;
        let active = RoutingActiveType::from(active);
        let reserved = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());
        offset += 4;
        let user_def = match data_len - offset {
            0 => Ok(None),
            4 => Ok(Some(u32::from_be_bytes(data[offset..offset+4].try_into().unwrap()))),
            _ => Err(DoIpError::InvalidLength { actual: data_len, expected: Self::length()+4 }),
        }?;

        Ok(Self { src_addr, active, reserved, user_def } )
    }
}

impl Into<Vec<u8>> for RoutingActive {
    fn into(self) -> Vec<u8> {
        let mut result = TCP_REQ_ROUTING_ACTIVE.to_be_bytes().to_vec();
        let mut length = Self::length() as u32;
        if self.user_def.is_some() {
            length += 4;
        }
        result.extend(length.to_be_bytes());
        let src_addr: u16 = self.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        result.push(self.active.into());
        result.extend(self.reserved.to_be_bytes());
        if let Some(user_def) = self.user_def {
            result.extend(user_def.to_be_bytes());
        }

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AliveCheck;     // 0x0007

impl AliveCheck {
    #[inline]
    const fn length() -> usize {
        0
    }
}

impl TryFrom<&[u8]> for AliveCheck {
    type Error = DoIpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;

        Ok(Self)
    }
}

impl Into<Vec<u8>> for AliveCheck {
    fn into(self) -> Vec<u8> {
        let mut result = TCP_REQ_ALIVE_CHECK.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());

        result
    }
}

/// The first response is 0x8002 if diagnostic is positive,
/// that means diagnostic request was received,
/// then send 0x8001 response with UDS data.
/// Otherwise, send 0x8003 response with UDS NRC data.
#[derive(Debug, Clone, Eq, PartialEq, Getters)]
pub struct Diagnostic {     // 0x8001
    pub(crate) src_addr: LogicAddress,
    pub(crate) dst_addr: LogicAddress,
    pub(crate) data: Vec<u8>,
}

impl Diagnostic {
    pub fn new(
        src_addr: LogicAddress,
        dst_addr: LogicAddress,
        data: Vec<u8>,
    ) -> Self {
        Self { src_addr, dst_addr, data }
    }

    /// min length
    #[inline]
    const fn length() -> usize {
        SIZE_OF_ADDRESS + SIZE_OF_ADDRESS
    }
}

impl TryFrom<&[u8]> for Diagnostic {
    type Error = DoIpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let src_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let src_addr = LogicAddress::from(src_addr);
        let dst_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        let dst_addr = LogicAddress::from(dst_addr);
        offset += SIZE_OF_ADDRESS;
        let data = data[offset..].to_vec();

        Ok(Self { src_addr, dst_addr, data })
    }
}

impl Into<Vec<u8>> for Diagnostic {
    fn into(mut self) -> Vec<u8> {
        let mut result = TCP_REQ_DIAGNOSTIC.to_be_bytes().to_vec();
        let length = (Self::length() + self.data.len()) as u32;
        result.extend(length.to_be_bytes());
        let src_addr: u16 = self.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        let dst_addr: u16 = self.dst_addr.into();
        result.extend(dst_addr.to_be_bytes());
        result.append(&mut self.data);

        result
    }
}
/****** --- end of TCP --- ********/