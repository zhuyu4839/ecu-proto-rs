use std::fmt::{Display, Formatter};
use getset::{CopyGetters, Getters};
use crate::{*, utils};

#[derive(Debug, Clone, Eq, PartialEq, CopyGetters)]
#[get_copy = "pub"]
pub struct HeaderNegative {
    pub(crate) code: HeaderNegativeCode,
}

impl HeaderNegative {
    pub fn new(code: HeaderNegativeCode) -> Self {
        Self { code }
    }

    #[inline]
    const fn length() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for HeaderNegative {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, offset) = utils::data_len_check(data, Self::length(), true)?;
        let code = data[offset];
        let code = HeaderNegativeCode::from(code);

        Ok(Self { code })
    }
}

impl Into<Vec<u8>> for HeaderNegative {
    fn into(self) -> Vec<u8> {
        let mut result = HEADER_NEGATIVE.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());
        result.push(self.code.into());

        result
    }
}

/****** --- UDP --- ********/

/// response with delay
/// send response 3 times with interval 500ms
/// the RoutingActive from client must be 0xE0 when further_act = 0x10.
#[derive(Debug, Clone, Eq, PartialEq, Getters, CopyGetters)]
#[get = "pub"]
pub struct VehicleID {  // 0x0004
    pub(crate) vin: String,
    #[get_copy = "pub"]
    pub(crate) address: LogicAddress,
    #[get_copy = "pub"]
    pub(crate) eid: Eid,
    #[get_copy = "pub"]
    pub(crate) gid: Gid,
    #[get_copy = "pub"]
    pub(crate) further_act: FurtherAction,
    #[get_copy = "pub"]
    pub(crate) sync_status: Option<SyncStatus>,
}

impl VehicleID {
    #[must_use]
    pub fn new(
        vin: String,
        address: LogicAddress,
        eid: Eid,
        gid: Eid,
        further_act: FurtherAction,
        sync_status: Option<SyncStatus>,
    ) -> Result<Self, Iso13400Error> {
        let vin_len = vin.as_bytes().len();
        if vin_len != LENGTH_OF_VIN {
            return Err(Iso13400Error::InputError(
                format!("length of vin must equal {}", LENGTH_OF_VIN)
            ));
        }

        Ok(Self { vin, address, eid, gid, further_act, sync_status })
    }

    /// min length
    #[inline]
    const fn length() -> usize {
        LENGTH_OF_VIN + SIZE_OF_ADDRESS + Eid::length() + Gid::length() + 1
    }
}

impl TryFrom<&[u8]> for VehicleID {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (data_len, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let vin = match String::from_utf8(data[offset..offset+LENGTH_OF_VIN].to_vec()) {
            Ok(v) => v,
            Err(_) => {
                log::warn!("invalid UTF-8 string: {}", hex::encode(data));
                "-".repeat(data_len)
            }
        };
        offset += LENGTH_OF_VIN;
        let address = u16::from_be_bytes(data[offset..offset+SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let address = LogicAddress::from(address);
        let eid = Eid::try_from(&data[offset..])?;
        offset += Eid::length();
        let gid = Gid::try_from(&data[offset..])?;
        offset += Gid::length();
        let further_act = FurtherAction::from(data[offset]);
        offset += 1;
        let sync_status = match data_len - offset {
            0 => Ok(None),
            1 => Ok(Some(SyncStatus::from(data[offset]))),
            _ => Err(Iso13400Error::InvalidLength { actual: data_len, expected: Self::length()+1 })
        }?;

        Ok(Self { vin, address, eid, gid, further_act, sync_status })
    }
}

impl Into<Vec<u8>> for VehicleID {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_RESP_VEHICLE_IDENTIFIER.to_be_bytes().to_vec();
        let mut length = Self::length() as u32;
        if self.sync_status.is_some() {
            length += 1;
        }
        result.extend(length.to_be_bytes());
        result.extend(self.vin.as_bytes());
        let address: u16 = self.address.into();
        result.extend(address.to_be_bytes());
        result.append(&mut self.eid.into());
        result.append(&mut self.gid.into());
        result.push(self.further_act.into());
        if let Some(status) = self.sync_status {
            result.push(status.into());
        }

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq, CopyGetters)]
#[get_copy = "pub"]
pub struct EntityStatus {   // 0x4002
    pub(crate) node_type: NodeType,
    /// 1 ~ 255
    pub(crate) mcts: u8,    // Max. concurrent TCP_DATA sockets
    /// 0 ~ 255
    pub(crate) ncts: u8,    // Current opened TCP_DATA sockets
    /// 0 ~ 4GB
    pub(crate) max_data_size: Option<u32>,
}

impl EntityStatus {
    pub fn new(
        node_type: NodeType,
        mcts: u8,
        ncts: u8,
        max_data_size: Option<u32>,
    ) -> Self {
        Self { node_type, mcts, ncts, max_data_size }
    }

    /// min length
    #[inline]
    const fn length() -> usize {
        1 + 1 + 1
    }
}

impl TryFrom<&[u8]> for EntityStatus {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (data_len, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let node_type = data[offset];
        offset += 1;
        let node_type = NodeType::from(node_type);
        let mcts = data[offset];
        offset += 1;
        let ncts = data[offset];
        offset += 1;
        let max_data_size = match data_len - offset {
            0 => Ok(None),
            4 => Ok(Some(u32::from_be_bytes(data[offset..offset+4].try_into().unwrap()))),
            _ => Err(Iso13400Error::InvalidLength { actual: data_len, expected: Self::length()+4 }),
        }?;

        Ok(Self { node_type, mcts, ncts, max_data_size })
    }
}

impl Into<Vec<u8>> for EntityStatus {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_RESP_ENTITY_STATUS.to_be_bytes().to_vec();
        let mut length = Self::length() as u32;
        if self.max_data_size.is_some() {
            length += 4;
        }
        result.extend(length.to_be_bytes());
        result.push(self.node_type.into());
        result.push(self.mcts);
        result.push(self.ncts);
        if let Some(size) = self.max_data_size {
            result.extend(size.to_be_bytes());
        }

        result
    }
}

impl Display for EntityStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DoIP Entity Status")
            .field("\n              Node Type", &self.node_type)
            .field("\n    Max. TCP Connectors", &self.mcts)
            .field("\n Current TCP Connectors", &self.ncts)
            .field("\n       Max. Data Length", &self.max_data_size)
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, CopyGetters)]
#[get_copy = "pub"]
pub struct DiagnosticPowerMode {    // 0x4004
    pub(crate) mode: PowerMode,
}

impl DiagnosticPowerMode {
    pub fn new(mode: PowerMode) -> Self {
        Self { mode }
    }

    #[inline]
    const fn length() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for DiagnosticPowerMode {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, offset) = utils::data_len_check(data, Self::length(), true)?;
        let mode = data[offset];
        let mode = PowerMode::from(mode);
        Ok(Self { mode })
    }
}

impl Into<Vec<u8>> for DiagnosticPowerMode {
    fn into(self) -> Vec<u8> {
        let mut result = UDP_RESP_DIAGNOSTIC_POWER_MODE.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());
        result.push(self.mode.into());
        result
    }
}
/****** --- end of UDP --- ********/

/****** --- TCP --- ********/
#[derive(Debug, Clone, Eq, PartialEq, CopyGetters)]
#[get_copy = "pub"]
pub struct RoutingActive {  // 0x0006
    pub(crate) dst_addr: LogicAddress,
    pub(crate) src_addr: LogicAddress,
    pub(crate) active_code: ActiveCode,
    pub(crate) reserved: u32,
    // #[getter(name = "user_define")]
    pub(crate) user_def: Option<u32>,
}

impl RoutingActive {
    pub fn new(
        dst_addr: LogicAddress,
        src_addr: LogicAddress,
        active_code: ActiveCode,
        user_def: Option<u32>
    ) -> Self {
        Self { dst_addr, src_addr, active_code, reserved: Default::default(), user_def }
    }

    /// min length
    #[inline]
    const fn length() -> usize {
        SIZE_OF_ADDRESS + SIZE_OF_ADDRESS + 1 + 4
    }
}

impl TryFrom<&[u8]> for RoutingActive {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (data_len, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let dst_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let dst_addr = LogicAddress::from(dst_addr);
        let src_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let src_addr = LogicAddress::from(src_addr);
        let active = data[offset];
        offset += 1;
        let active_code = ActiveCode::from(active);
        let reserved = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());
        offset += 4;
        let user_def = match data_len - offset {
            0 => Ok(None),
            4 => Ok(Some(u32::from_be_bytes(data[offset..offset+4].try_into().unwrap()))),
            _ => Err(Iso13400Error::InvalidLength { actual: data_len, expected: Self::length()+4 }),
        }?;

        Ok(Self { dst_addr, src_addr, active_code, reserved, user_def })
    }
}

impl Into<Vec<u8>> for RoutingActive {
    fn into(self) -> Vec<u8> {
        let mut result = TCP_RESP_ROUTING_ACTIVE.to_be_bytes().to_vec();
        let mut length = Self::length() as u32;
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

#[derive(Debug, Clone, Eq, PartialEq, CopyGetters)]
#[get_copy = "pub"]
pub struct AliveCheck {     // 0x0008
    pub(crate) src_addr: LogicAddress,
}

impl AliveCheck {
    pub fn new(addr: LogicAddress) -> Self {
        Self { src_addr: addr }
    }

    #[inline]
    const fn length() -> usize {
        SIZE_OF_ADDRESS
    }
}

impl TryFrom<&[u8]> for AliveCheck {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, offset) = utils::data_len_check(data, Self::length(), true)?;
        let src_addr = u16::from_be_bytes(data[offset..].try_into().unwrap());
        let src_addr = LogicAddress::from(src_addr);

        Ok(Self { src_addr })
    }
}

impl Into<Vec<u8>> for AliveCheck {
    fn into(self) -> Vec<u8> {
        let mut result = TCP_RESP_ALIVE_CHECK.to_be_bytes().to_vec();
        let length = Self::length() as u32;
        result.extend(length.to_be_bytes());
        let src_addr: u16 = self.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Getters)]
#[get = "pub"]
pub struct DiagnosticPositive {     // 0x8002
    #[getset(get_copy = "pub")]
    pub(crate) src_addr: LogicAddress,
    #[getset(get_copy = "pub")]
    pub(crate) dst_addr: LogicAddress,
    #[getset(get_copy = "pub")]
    pub(crate) code: DiagnosticPositiveCode,
    // #[getter(name = "previous_diagnostic_data")]
    pub(crate) pre_diag_msg: Vec<u8>,
}

impl DiagnosticPositive {
    pub fn new(
        src_addr: LogicAddress,
        dst_addr: LogicAddress,
        code: DiagnosticPositiveCode,
        pre_diag_msg: Vec<u8>,
    ) -> Self {
        if code != DiagnosticPositiveCode::Confirm {
            log::warn!("Diagnostic Positive code: {:?}", code);
        }
        Self { src_addr, dst_addr, code, pre_diag_msg }
    }
    /// min length
    #[inline]
    const fn length() -> usize {
        SIZE_OF_ADDRESS + SIZE_OF_ADDRESS + 1
    }
}

impl TryFrom<&[u8]> for DiagnosticPositive {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let src_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let src_addr = LogicAddress::from(src_addr);
        let dst_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let dst_addr = LogicAddress::from(dst_addr);
        let code = DiagnosticPositiveCode::from(data[offset]);
        offset += 1;
        let pre_diag_msg = data[offset..].to_vec();

        Ok(Self::new(src_addr, dst_addr, code, pre_diag_msg))
    }
}

impl Into<Vec<u8>> for DiagnosticPositive {
    fn into(mut self) -> Vec<u8> {
        let mut result = TCP_RESP_DIAGNOSTIC_POSITIVE.to_be_bytes().to_vec();
        let length = (Self::length() + self.pre_diag_msg.len()) as u32;
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

impl Display for DiagnosticPositive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Diagnostic Positive")
            .field("\n       Source Address", &self.src_addr)
            .field("\n       Target Address", &self.dst_addr)
            .field("\n                 Code", &self.code)
            .field("\n        Previous Data", &format!("{}", hex::encode(&self.pre_diag_msg)))
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Getters, CopyGetters)]
pub struct DiagnosticNegative {     // 0x8003
    #[getset(get_copy = "pub")]
    pub(crate) src_addr: LogicAddress,
    #[getset(get_copy = "pub")]
    pub(crate) dst_addr: LogicAddress,
    #[getset(get_copy = "pub")]
    pub(crate) code: DiagnosticNegativeCode,
    // #[getter(name = "previous_diagnostic_data")]
    #[getset(get = "pub")]
    pub(crate) previous_diagnostic_data: Vec<u8>,
}

impl DiagnosticNegative {
    pub fn new(
        src_addr: LogicAddress,
        dst_addr: LogicAddress,
        code: DiagnosticNegativeCode,
        previous_diagnostic_data: Vec<u8>,
    ) -> Self {
        Self { src_addr, dst_addr, code, previous_diagnostic_data }
    }

    /// min length
    #[inline]
    const fn length() -> usize {
        SIZE_OF_ADDRESS + SIZE_OF_ADDRESS + 1
    }
}

impl TryFrom<&[u8]> for DiagnosticNegative {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let src_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let src_addr = LogicAddress::from(src_addr);
        let dst_addr = u16::from_be_bytes(data[offset..offset+ SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let dst_addr = LogicAddress::from(dst_addr);
        let code = DiagnosticNegativeCode::from(data[offset]);
        offset += 1;
        let previous_diagnostic_data = data[offset..].to_vec();

        Ok(Self { src_addr, dst_addr, code, previous_diagnostic_data })
    }
}

impl Into<Vec<u8>> for DiagnosticNegative {
    fn into(mut self) -> Vec<u8> {
        let mut result = TCP_RESP_DIAGNOSTIC_NEGATIVE.to_be_bytes().to_vec();
        let length = (Self::length() + self.previous_diagnostic_data.len()) as u32;
        result.extend(length.to_be_bytes());
        let src_addr: u16 = self.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        let dst_addr: u16 = self.dst_addr.into();
        result.extend(dst_addr.to_be_bytes());
        result.push(self.code.into());
        result.append(&mut self.previous_diagnostic_data);

        result
    }
}

impl Display for DiagnosticNegative {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Diagnostic Negative")
            .field("\n       Source Address", &self.src_addr)
            .field("\n       Target Address", &self.dst_addr)
            .field("\n                 Code", &self.code)
            .field("\n        Previous Data", &format!("{}", hex::encode(&self.previous_diagnostic_data)))
            .finish()
    }
}
/****** --- end of UDP --- ********/
