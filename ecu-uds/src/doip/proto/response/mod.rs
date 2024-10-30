use crate::doip::{{ActiveCode, DiagnosticCode, Entity, FurtherAction, HeaderNegativeCode, LogicAddress, PowerMode}, constant::*};

#[derive(Debug, Clone)]
pub struct HeaderNegative {
    pub(crate) code: HeaderNegativeCode,
}

const LENGTH_OF_HEADER_NEGATIVE: usize = 1;

impl TryFrom<&[u8]> for HeaderNegative {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len != LENGTH_OF_HEADER_NEGATIVE {
            return Err(format!("invalid length: {}", data_len));
        }

        let code = data[0];
        let code = HeaderNegativeCode::from(code);

        Ok(Self { code })
    }
}

impl Into<Vec<u8>> for HeaderNegative {
    fn into(self) -> Vec<u8> {
        let mut result = HEADER_NEGATIVE.to_be_bytes().to_vec();
        let length = LENGTH_OF_HEADER_NEGATIVE as u16;
        result.extend(length.to_be_bytes());
        result.push(self.code.into());

        result
    }
}

/****** --- UDP --- ********/

/// response with delay
/// send response 3 times with interval 500ms
/// the RoutingActive from client must be 0xE0 when further_act = 0x10.
#[derive(Debug, Clone)]
pub struct VehicleID {  // 0x0004
    pub(crate) vin: String, // TODO length equal 17
    pub(crate) address: LogicAddress,
    pub(crate) eid: [u8; LENGTH_OF_ID],
    pub(crate) gid: [u8; LENGTH_OF_ID],
    pub(crate) further_act: FurtherAction,
    pub(crate) sync_status: Option<u8>, // TODO
}

const MIN_VEHICLE_ID_LENGTH: usize = LENGTH_OF_VIN + 2 + 2 * LENGTH_OF_ID + 1;

impl TryFrom<&[u8]> for VehicleID {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < MIN_VEHICLE_ID_LENGTH {
            return Err(format!("invalid length: {}", data_len));
        }

        let mut offset = 0;
        let vin = match String::from_utf8(data[..LENGTH_OF_VIN].to_vec()) {
            Ok(v) => v,
            Err(_) => {
                log::warn!("invalid UTF-8 string: {}", hex::encode(data));
                "-".repeat(data_len)
            }
        };
        offset += LENGTH_OF_VIN;
        let address = u16::from_be_bytes(data[offset..offset + 2].try_into().unwrap());
        offset += 2;
        let address = LogicAddress::from(address);
        let eid: [u8; LENGTH_OF_ID] = data[offset..offset + LENGTH_OF_ID].try_into().unwrap();
        offset += LENGTH_OF_ID;
        let gid: [u8; LENGTH_OF_ID] = data[offset..offset + LENGTH_OF_ID].try_into().unwrap();
        offset += LENGTH_OF_ID;
        let further_act = FurtherAction::from(data[offset]);
        offset += 1;
        let sync_status = match data_len - offset {
            0 => Ok(None),
            1 => Ok(Some(data[offset])),
            _ => Err(format!("invalid length: {}", data_len)),
        }?;

        Ok(Self { vin, address, eid, gid, further_act, sync_status })
    }
}

impl Into<Vec<u8>> for VehicleID {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_RESP_VEHICLE_IDENTIFIER.to_be_bytes().to_vec();
        let mut length = MIN_VEHICLE_ID_LENGTH as u16;
        if self.sync_status.is_some() {
            length += 1;
        }
        result.extend(length.to_be_bytes());
        result.extend(self.vin.as_bytes());
        let address: u16 = self.address.into();
        result.extend(address.to_be_bytes());
        result.extend(self.eid);
        result.extend(self.gid);
        result.push(self.further_act.into());
        if let Some(status) = self.sync_status {
            result.push(status);
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct EntityStatus {   // 0x4002
    pub(crate) entity: Entity,
    /// 1 ~ 255
    pub(crate) mcts: u8,    // Max. concurrent TCP_DATA sockets
    /// 0 ~ 255
    pub(crate) ncts: u8,    // Current opened TCP_DATA sockets
    pub(crate) max_data_size: Option<u32>,
}
const MIN_OF_ENTITY_STATUS: usize = 1 + 1 + 1;
impl TryFrom<&[u8]> for EntityStatus {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < MIN_OF_ENTITY_STATUS {
            return Err(format!("invalid length: {}", data_len));
        }
        let mut offset = 0;
        let entity = data[offset];
        offset += 1;
        let entity = Entity::from(entity);
        let mcts = data[offset];
        offset += 1;
        let ncts = data[offset];
        offset += 1;
        let max_data_size = match data_len - offset {
            0 => Ok(None),
            4 => Ok(Some(u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap()))),
            _ => Err(format!("invalid length: {}", data_len)),
        }?;

        Ok(Self { entity, mcts, ncts, max_data_size })
    }
}

impl Into<Vec<u8>> for EntityStatus {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_RESP_ENTITY_STATUS.to_be_bytes().to_vec();
        let mut length = MIN_OF_ENTITY_STATUS as u16;
        if self.max_data_size.is_some() {
            length += 4;
        }
        result.extend(length.to_be_bytes());
        result.push(self.entity.into());
        result.push(self.mcts);
        result.push(self.ncts);
        if let Some(size) = self.max_data_size {
            result.extend(size.to_be_bytes());
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticPowerMode {    // 0x4004
    pub(crate) mode: PowerMode,
}
const LENGTH_OF_DIAG_POWER_MODE: usize = 1;
impl TryFrom<&[u8]> for DiagnosticPowerMode {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len != LENGTH_OF_DIAG_POWER_MODE {
            return Err(format!("invalid length: {}", data_len));
        }

        let mode = data[0];
        let mode = PowerMode::from(mode);
        Ok(Self { mode })
    }
}

impl Into<Vec<u8>> for DiagnosticPowerMode {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_RESP_DIAGNOSTIC_POWER_MODE.to_be_bytes().to_vec();
        let length = LENGTH_OF_DIAG_POWER_MODE as u16;
        result.extend(length.to_be_bytes());
        result.push(self.mode.into());
        result
    }
}
/****** --- end of UDP --- ********/

/****** --- TCP --- ********/
#[derive(Debug, Clone)]
pub struct RoutingActive {  // 0x0006
    pub(crate) dst_addr: LogicAddress,
    pub(crate) src_addr: LogicAddress,
    pub(crate) active_code: ActiveCode,
    pub(crate) reserved: u32,
    pub(crate) user_def: Option<u32>,
}
const MIN_ROUTING_ACTIVE_LENGTH: usize = 2 + 2 + 1 + 4;
impl TryFrom<&[u8]> for RoutingActive {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < MIN_ROUTING_ACTIVE_LENGTH {
            return Err(format!("invalid length: {}", data_len));
        }
        let mut offset = 0;
        let dst_addr = u16::from_be_bytes(data[offset..offset + 2].try_into().unwrap());
        offset += 2;
        let dst_addr = LogicAddress::from(dst_addr);
        let src_addr = u16::from_be_bytes(data[offset..offset + 2].try_into().unwrap());
        offset += 2;
        let src_addr = LogicAddress::from(src_addr);
        let active = data[offset];
        offset += 1;
        let active_code = ActiveCode::from(active);
        let reserved = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let user_def = match data_len - offset {
            0 => Ok(None),
            4 => Ok(Some(u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap()))),
            _ => Err(format!("invalid length: {}", data_len)),
        }?;

        Ok(Self { dst_addr, src_addr, active_code, reserved, user_def })
    }
}

impl Into<Vec<u8>> for RoutingActive {
    fn into(self) -> Vec<u8> {
        let mut result = TCP_RESP_ROUTING_ACTIVE.to_be_bytes().to_vec();
        let mut length = MIN_ROUTING_ACTIVE_LENGTH as u16;
        if self.user_def.is_some() {
            length += 4;
        }
        result.extend(length.to_be_bytes());
        let dst_addr: u16 = self.dst_addr.into();
        result.extend(dst_addr.to_be_bytes());
        let src_addr: u16 = self.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        result.push(self.active_code.into());
        result.extend(self.reserved.to_be_bytes());
        if let Some(user_def) = self.user_def {
            result.extend(user_def.to_be_bytes());
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct AliveCheck {     // 0x0008
    pub(crate) src_addr: LogicAddress,
}
const LENGTH_OF_ALIVE_CHECK: usize = 2;
impl TryFrom<&[u8]> for AliveCheck {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len != LENGTH_OF_ALIVE_CHECK {
            return Err(format!("invalid length: {}", data_len));
        }
        let src_addr = u16::from_be_bytes(data.try_into().unwrap());
        let src_addr = LogicAddress::from(src_addr);

        Ok(Self { src_addr })
    }
}

impl Into<Vec<u8>> for AliveCheck {
    fn into(self) -> Vec<u8> {
        let mut result = TCP_RESP_ALIVE_CHECK.to_be_bytes().to_vec();
        let length = LENGTH_OF_ALIVE_CHECK as u16;
        result.extend(length.to_be_bytes());
        let src_addr: u16 = self.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        result
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticPositive {     // 0x8002
    pub(crate) src_addr: LogicAddress,
    pub(crate) dst_addr: LogicAddress,
    pub(crate) code: DiagnosticCode,
    pub(crate) pre_diag_msg: Vec<u8>,
}
const MIN_DIAG_POSITIVE_LENGTH: usize = 2 + 2 + 1;
impl TryFrom<&[u8]> for DiagnosticPositive {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < MIN_DIAG_POSITIVE_LENGTH {
            return Err(format!("invalid length: {}", data_len));
        }
        let mut offset = 0;
        let src_addr = u16::from_be_bytes(data[..offset + 2].try_into().unwrap());
        offset += 2;
        let src_addr = LogicAddress::from(src_addr);
        let dst_addr = u16::from_be_bytes(data[..offset + 2].try_into().unwrap());
        offset += 2;
        let dst_addr = LogicAddress::from(dst_addr);
        let code = DiagnosticCode::from(data[offset]);
        offset += 1;
        let pre_diag_msg = data[offset..].to_vec();

        Ok(Self { src_addr, dst_addr, code, pre_diag_msg })
    }
}

impl Into<Vec<u8>> for DiagnosticPositive {
    fn into(mut self) -> Vec<u8> {
        let mut result = TCP_RESP_DIAGNOSTIC_POSITIVE.to_be_bytes().to_vec();
        let length = (MIN_DIAG_POSITIVE_LENGTH + self.pre_diag_msg.len()) as u16;
        result.extend(length.to_be_bytes());
        let src_addr: u16 = self.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        let dst_addr: u16 = self.dst_addr.into();
        result.extend(dst_addr.to_be_bytes());
        result.push(self.code.into());
        result.append(&mut self.pre_diag_msg);

        result
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticNegative {     // 0x8003
    pub(crate) src_addr: LogicAddress,
    pub(crate) dst_addr: LogicAddress,
    pub(crate) code: DiagnosticCode,
    pub(crate) pre_diag_msg: Vec<u8>,
}
const MIN_DIAG_NEGATIVE_LENGTH: usize = 2 + 2 + 1;
impl TryFrom<&[u8]> for DiagnosticNegative {
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < MIN_DIAG_NEGATIVE_LENGTH {
            return Err(format!("invalid length: {}", data_len));
        }
        let mut offset = 0;
        let src_addr = u16::from_be_bytes(data[..offset + 2].try_into().unwrap());
        offset += 2;
        let src_addr = LogicAddress::from(src_addr);
        let dst_addr = u16::from_be_bytes(data[..offset + 2].try_into().unwrap());
        offset += 2;
        let dst_addr = LogicAddress::from(dst_addr);
        let code = DiagnosticCode::from(data[offset]);
        offset += 1;
        let pre_diag_msg = data[offset..].to_vec();

        Ok(Self { src_addr, dst_addr, code, pre_diag_msg })
    }
}

impl Into<Vec<u8>> for DiagnosticNegative {
    fn into(mut self) -> Vec<u8> {
        let mut result = TCP_RESP_DIAGNOSTIC_NEGATIVE.to_be_bytes().to_vec();
        let length = (MIN_DIAG_NEGATIVE_LENGTH + self.pre_diag_msg.len()) as u16;
        result.extend(length.to_be_bytes());
        let src_addr: u16 = self.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        let dst_addr: u16 = self.dst_addr.into();
        result.extend(dst_addr.to_be_bytes());
        result.push(self.code.into());
        result.append(&mut self.pre_diag_msg);

        result
    }
}
/****** --- end of UDP --- ********/
