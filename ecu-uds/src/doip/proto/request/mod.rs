use crate::doip::{LogicAddress, constant::*, RoutingActiveType};

/****** --- UDP --- ********/
#[derive(Debug, Clone)]
pub struct VehicleID;   // 0x0001

const LENGTH_OF_VEHICLE_ID: usize = 0;

impl TryFrom<&[u8]> for VehicleID {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len == LENGTH_OF_VEHICLE_ID {
            return Err(format!("invalid length: {}", data_len));
        }

        Ok(Self)
    }
}

impl Into<Vec<u8>> for VehicleID {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_VEHICLE_IDENTIFIER.to_be_bytes().to_vec();
        let length = LENGTH_OF_VEHICLE_ID as u16;
        result.extend(length.to_be_bytes());

        result
    }
}

#[derive(Debug, Clone)]
pub struct VehicleIDWithEID {    // 0x0002
    pub(crate) eid: [u8; LENGTH_OF_ID],
}

impl TryFrom<&[u8]> for VehicleIDWithEID {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len == LENGTH_OF_ID {
            return Err(format!("invalid length: {}", data_len));
        }

        let eid: [u8; LENGTH_OF_ID] = data[..LENGTH_OF_ID].try_into().unwrap();
        Ok(Self { eid })
    }
}

impl Into<Vec<u8>> for VehicleIDWithEID {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_VEHICLE_ID_WITH_EID.to_be_bytes().to_vec();
        let length = LENGTH_OF_ID as u16;
        result.extend(length.to_be_bytes());
        result.extend(self.eid);

        result
    }
}

#[derive(Debug, Clone)]
pub struct VehicleIDWithVIN {     // 0x0003
    pub(crate) vin: String, // TODO length equal 17
}

impl TryFrom<&[u8]> for VehicleIDWithVIN {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len != LENGTH_OF_VIN {
            return Err(format!("invalid length: {}", data_len));
        }

        let vin = match String::from_utf8(data.to_vec()) {
            Ok(v) => v,
            Err(_) => {
                log::warn!("invalid UTF-8 string: {}", hex::encode(data));
                "-".repeat(data_len)
            }
        };

        Ok(Self { vin })
    }
}

impl Into<Vec<u8>> for VehicleIDWithVIN {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_VEHICLE_ID_WITH_VIN.to_be_bytes().to_vec();
        let length = LENGTH_OF_VIN as u16;
        result.extend(length.to_be_bytes());
        result.append(&mut self.vin.as_bytes().to_vec());

        result
    }
}

#[derive(Debug, Clone)]
pub struct EntityStatus;   // 0x4001

const LENGTH_OF_ENTITY_STATUS: usize = 0x00;
impl TryFrom<&[u8]> for EntityStatus {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len != LENGTH_OF_ENTITY_STATUS {
            return Err(format!("invalid length: {}", data_len));
        }

        Ok(Self)
    }
}

impl Into<Vec<u8>> for EntityStatus {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_ENTITY_STATUS.to_be_bytes().to_vec();
        let length = LENGTH_OF_ENTITY_STATUS as u16;
        result.extend(length.to_be_bytes());

        result
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticPowerMode;    // 0x4003
const LENGTH_OF_DIAG_POWER_MODE: usize = 0x00;
impl TryFrom<&[u8]> for DiagnosticPowerMode {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len != LENGTH_OF_DIAG_POWER_MODE {
            return Err(format!("invalid length: {}", data_len));
        }
        Ok(Self)
    }
}

impl Into<Vec<u8>> for DiagnosticPowerMode {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_REQ_DIAGNOSTIC_POWER_MODE.to_be_bytes().to_vec();
        let length = LENGTH_OF_DIAG_POWER_MODE as u16;
        result.extend(length.to_be_bytes());

        result
    }
}

/****** --- end of UDP --- ********/

/****** --- TCP --- ********/
#[derive(Debug, Clone)]
pub struct RoutingActive {  // 0x0005
    pub(crate) src_addr: LogicAddress,
    pub(crate) active: RoutingActiveType,
    pub(crate) reserved: u32,
    pub(crate) user_def: Option<u32>,
}
const MIN_ROUTING_ACTIVE_LENGTH: usize = 2 + 1 + 4;
impl TryFrom<&[u8]> for RoutingActive {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < MIN_ROUTING_ACTIVE_LENGTH {
            return Err(format!("invalid length: {}", data_len));
        }

        let mut offset = 0;
        let src_addr = u16::from_be_bytes([data[0], data[1]]);
        offset += 2;
        let src_addr = LogicAddress::from(src_addr);
        let active = data[offset];
        offset += 1;
        let active = RoutingActiveType::from(active);
        let reserved = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());
        offset += 4;
        let user_def = match data_len - offset {
            0 => Ok(None),
            4 => Ok(Some(u32::from_be_bytes(data[offset..offset+4].try_into().unwrap()))),
            _ => Err(format!("invalid length: {}", data_len)),
        }?;

        Ok(Self { src_addr, active, reserved, user_def } )
    }
}

impl Into<Vec<u8>> for RoutingActive {
    fn into(self) -> Vec<u8> {
        let mut result = TCP_REQ_ROUTING_ACTIVE.to_be_bytes().to_vec();
        let mut length = MIN_ROUTING_ACTIVE_LENGTH as u16;
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

#[derive(Debug, Clone)]
pub struct AliveCheck;     // 0x0007
const LENGTH_OF_ALIVE_CHECK_LENGTH: usize = 0;

impl TryFrom<&[u8]> for AliveCheck {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len != LENGTH_OF_ALIVE_CHECK_LENGTH {
            return Err(format!("invalid length: {}", data_len));
        }

        Ok(Self)
    }
}

impl Into<Vec<u8>> for AliveCheck {
    fn into(self) -> Vec<u8> {
        let mut result = TCP_REQ_ALIVE_CHECK.to_be_bytes().to_vec();
        let length = LENGTH_OF_ALIVE_CHECK_LENGTH as u16;
        result.extend(length.to_be_bytes());

        result
    }
}

/// The first response is 0x8002 if diagnostic is positive,
/// that means diagnostic request was received,
/// then send 0x8001 response with UDS data.
/// Otherwise, send 0x8003 response with UDS NRC data.
#[derive(Debug, Clone)]
pub struct Diagnostic {     // 0x8001
    pub(crate) src_addr: LogicAddress,
    pub(crate) dst_addr: LogicAddress,
    pub(crate) data: Vec<u8>,
}
const MIN_DIAGNOSTIC_LENGTH: usize = 2 + 2;
impl TryFrom<&[u8]> for Diagnostic {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < MIN_DIAGNOSTIC_LENGTH {
            return Err(format!("invalid length: {}", data_len));
        }
        let mut offset = 0;
        let src_addr = u16::from_be_bytes(data[..offset+2].try_into().unwrap());
        offset += 2;
        let src_addr = LogicAddress::from(src_addr);
        let dst_addr = u16::from_be_bytes(data[offset..offset+2].try_into().unwrap());
        let dst_addr = LogicAddress::from(dst_addr);
        offset += 2;
        let data = data[offset..].to_vec();

        Ok(Self { src_addr, dst_addr, data })
    }
}

impl Into<Vec<u8>> for Diagnostic {
    fn into(mut self) -> Vec<u8> {
        let mut result = TCP_REQ_DIAGNOSTIC.to_be_bytes().to_vec();
        let length = (MIN_DIAGNOSTIC_LENGTH + self.data.len()) as u16;
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
