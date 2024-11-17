use std::fmt::{Display, Formatter};
use crate::{constants::*, request, response, Iso13400Error};

/// Table 16 — Generic DoIP header structure at line #48(ISO 13400-2-2019)
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Version {
    ISO13400_2_2010 = 0x01,
    ISO13400_2_2012 = 0x02,
    ISO13400_2_2019 = 0x03,
    Reserved(u8),
    #[default]
    Default = 0xFF,
}

impl Into<u8> for Version {
    fn into(self) -> u8 {
        match self {
            Self::ISO13400_2_2010 => 0x01,
            Self::ISO13400_2_2012 => 0x02,
            Self::ISO13400_2_2019 => 0x03,
            Self::Default => 0xFF,
            Self::Reserved(v) => v,
        }
    }
}

impl Into<Vec<u8>> for Version {
    fn into(self) -> Vec<u8> {
        let version: u8 = self.into();
        vec![version, !version]
    }
}

impl From<u8> for Version {
    fn from(version: u8) -> Self {
        match version {
            0x01 => Self::ISO13400_2_2010,
            0x02 => Self::ISO13400_2_2012,
            0x03 => Self::ISO13400_2_2019,
            0xFF => Self::Default,
            _ => {
                log::warn!("ISO 13400-2 - used reserved version: {}", version);
                Self::Reserved(version)
            },
        }
    }
}

impl TryFrom<&[u8]> for Version {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < SIZE_OF_VERSION {
            return Err(Iso13400Error::InvalidLength { actual: data_len, expected: SIZE_OF_VERSION });
        }

        let version = data[0];
        let reverse = data[1];
        if !version != reverse {
            return Err(Iso13400Error::InvalidVersion { version, reverse });
        }

        Ok(Self::from(version))
    }
}

/// Table 19 — Generic DoIP header NACK codes at line #52(ISO 13400-2-2019)
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HeaderNegativeCode {
    IncorrectPatternFormat = 0x00,     // close socket
    UnknownPayloadTYpe = 0x01,
    MessageTooLarge = 0x02,
    OutOfMemory = 0x03,
    InvalidPayloadLength = 0x04,    // close socket
    Reserved(u8),
}

impl Into<u8> for HeaderNegativeCode {
    fn into(self) -> u8 {
        match self {
            Self::IncorrectPatternFormat => 0x00,
            Self::UnknownPayloadTYpe => 0x01,
            Self::MessageTooLarge => 0x02,
            Self::OutOfMemory => 0x03,
            Self::InvalidPayloadLength => 0x04,
            Self::Reserved(code) => code,
        }
    }
}

impl From<u8> for HeaderNegativeCode {
    fn from(code: u8) -> Self {
        match code {
            0x00 => Self::IncorrectPatternFormat,
            0x01 => Self::UnknownPayloadTYpe,
            0x02 => Self::MessageTooLarge,
            0x03 => Self::OutOfMemory,
            0x04 => Self::InvalidPayloadLength,
            _ => {
                log::warn!("ISO 13400-2 - used reserved header negative code: {}", code);
                Self::Reserved(code)
            },
        }
    }
}

/// Table 13 — Logical address overview at line #37(ISO 13400-2-2019)
///
/// 0000         ISO/SAE reserved
///
/// 0001 to 0DFF VM specific
/// 0E00 to 0FFF reserved for addresses of client
///
/// 0E00 to 0E7F external legislated diagnostics test equipment (e.g. for emissions external test equipment)
///              When using these addresses in the routing activation request other ongoing diagnostic communication in the vehicle may be interrupted and other normal functionality may be impaired (e.g. return to a failsafe behaviour).
///
/// 0E80 to 0EFF external vehicle-manufacturer-/aftermarket-enhanced diagnostics test equipment
///              When using these addresses in the routing activation request and diagnostic messages the routing activation may be delayed initially due to other ongoing diagnostic communication, which may then be interrupted and other normal functionality may also be impaired (e.g. return to a failsafe behaviour).
///
/// 0F00 to 0F7F internal data collection/on-board diagnostic equipment (for vehicle-manufacturer use only)
///              These addresses should not be used by client DoIP entity that is not designed as an integral part of the vehicle. This includes any plug-in equipment that performs diagnostic communication through the diagnostic connector.
///
/// 0F80 to 0FFF external prolonged data collection equipment (vehicle data recorders and loggers, e.g. used by insurance companies or to collect vehicle fleet data)
///              These addresses should be used by equipment that is installed in the vehicle and remains in the vehicle for periodic data retrieval by means of diagnostic communication. The DoIP entities may deny/delay accepting a routing activation request from this type of equipment in order to complete ongoing vehicle internal communication to avoid that normal operation of the vehicle may be impaired.
///
/// 1000 to 7FFF VM specific
///
/// 8000 to CFFF ISO/SAE reserved
///
/// D000 to DFFF Reserved for SAE Truck & Bus Control and Communication Committee
///
/// E000 to E3FF Definition of logical address is specified in use case-specific standard(e.g. ISO 27145-1, ISO 20730-1).
///
/// E400 to EFFF vehicle-manufacturer-defined functional group logical addresses
///
/// F000 to FFFF ISO/SAE reserved
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LogicAddress {
    VMSpecific(u16),           // 0x0001 ~ 0x0DFF | 0x1000 ~ 0x7FFF
    Client(u16),        // 0x0E00 ~ 0x0FFF
    VMSpecificFunctional(u16), // 0xE400 ~ 0xEFFF (functional group logical addresses)
    Reserved(u16),      // 0x0000 | 0xE000 ~ 0xE3FF | 0xF000 ~ 0xFFFF
}

impl From<u16> for LogicAddress {
    fn from(value: u16) -> Self {
        match value {
            0x0001..=0x0DFF |
            0x1000..=0x7FFF => Self::VMSpecific(value),
            0x0E00..=0x0FFF => Self::Client(value),
            0xE400..=0xEFFF => Self::VMSpecificFunctional(value),
            _ => {
                log::warn!("ISO 13400-2 - used reserved logic address: {}", value);
                Self::Reserved(value)
            },
        }
    }
}

impl Into<u16> for LogicAddress {
    fn into(self) -> u16 {
        match self {
            Self::VMSpecific(v) => v,
            Self::Client(v) => v,
            Self::VMSpecificFunctional(v) => v,
            Self::Reserved(v) => v,
        }
    }
}

impl Display for LogicAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value: u16 = (*self).into();
        write!(f, "{:#X}", value)
    }
}

/// Table 11 — DoIP entity status response
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeType {
    Gateway = 0x00,
    Node = 0x01,
    Reserved(u8),
}

impl Into<u8> for NodeType {
    fn into(self) -> u8 {
        match self {
            Self::Gateway => 0x00,
            Self::Node => 0x01,
            Self::Reserved(v) => v,
        }
    }
}

impl From<u8> for NodeType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Gateway,
            0x01 => Self::Node,
            _ => {
                log::warn!("ISO 13400-2 - used reserved entity: {}", value);
                Self::Reserved(value)
            },
        }
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Gateway => write!(f, "Gateway"),
            NodeType::Node => write!(f, "Node"),
            NodeType::Reserved(v) => write!(f, "{}", format!("Unknown({:#X})", *v)),
        }
    }
}

/// Table 6 — Definition of further action code values
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FurtherAction {
    NoAction = 0x00,
    Reserved(u8),       // 0x01 ~ 0x0f
    CentralSecurity = 0x10,
    VMSpecific(u8),            // 0x11 ~ 0xFF
}

impl Into<u8> for FurtherAction {
    fn into(self) -> u8 {
        match self {
            Self::NoAction => 0x00,
            Self::Reserved(v) => v,
            Self::CentralSecurity => 0x10,
            Self::VMSpecific(v) => v,
        }
    }
}

impl From<u8> for FurtherAction {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NoAction,
            0x01..=0x0F => {
                log::warn!("ISO 13400-2 - used reserved further action: {}", value);
                Self::Reserved(value)
            },
            0x10 => Self::CentralSecurity,
            _ => Self::VMSpecific(value),
        }
    }
}

/// Table 7 — Definition of VIN/GID synchronization status code values
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SyncStatus {
    VINorGIDSync = 0x00,
    VINorGIDNotSync = 0x10,
    Reserved(u8),
}

impl Into<u8> for SyncStatus {
    fn into(self) -> u8 {
        match self {
            Self::VINorGIDSync => 0x00,
            Self::VINorGIDNotSync => 0x10,
            Self::Reserved(v) => v,
        }
    }
}

impl From<u8> for SyncStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::VINorGIDSync,
            0x10 => Self::VINorGIDNotSync,
            _ => {
                log::warn!("ISO 13400-2 - used reserved VIN/GID sync. status: {}", value);
                Self::Reserved(value)
            },
        }
    }
}

/// Table 49 — Routing activation response code values
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActiveCode {
    SourceAddressUnknown = 0x00,    // close TCP
    Activated = 0x01,       // close TCP
    SourceAddressInvalid = 0x02,    // close TCP
    SocketInvalid = 0x03,   // close TCP
    WithoutAuth = 0x04,
    VehicleRefused = 0x05,  // close TCP
    Unsupported = 0x06,     // close TCP
    /// ISO 14300-2:2019
    TLSRequired = 0x07,
    Success = 0x10,
    NeedConfirm = 0x11,
    VMSpecific(u8),        // 0xE0 ~ 0xFE
    Reserved(u8),   // 0x07 ~ 0x0F | 0x12 ~ 0xDF | 0xFF
}

impl Into<u8> for ActiveCode {
    fn into(self) -> u8 {
        match self {
            Self::SourceAddressUnknown => 0x00,
            Self::Activated => 0x01,
            Self::SourceAddressInvalid => 0x02,
            Self::SocketInvalid => 0x03,
            Self::WithoutAuth => 0x04,
            Self::VehicleRefused => 0x05,
            Self::Unsupported => 0x06,
            Self::TLSRequired => 0x07,
            Self::Success => 0x10,
            Self::NeedConfirm => 0x11,
            Self::VMSpecific(v) => v,
            Self::Reserved(v) => v,
        }
    }
}

impl From<u8> for ActiveCode {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::SourceAddressUnknown,
            0x01 => Self::Activated,
            0x02 => Self::SourceAddressInvalid,
            0x03 => Self::SocketInvalid,
            0x04 => Self::WithoutAuth,
            0x05 => Self::VehicleRefused,
            0x06 => Self::Unsupported,
            0x07 => Self::TLSRequired,
            0x10 => Self::Success,
            0x11 => Self::NeedConfirm,
            0xE0..=0xFE => Self::VMSpecific(v),
            _ => {
                log::warn!("ISO 13400-2 - used reserved active code: {}", v);
                Self::Reserved(v)
            },
        }
    }
}

/// Table 9 — Diagnostic power mode information response
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PowerMode {
    NotReady = 0x00,
    Ready = 0x01,
    NotSupported = 0x02,
    Reserved(u8),
}

impl Into<u8> for PowerMode {
    fn into(self) -> u8 {
        match self {
            Self::NotReady => 0x00,
            Self::Ready => 0x01,
            Self::NotSupported => 0x02,
            Self::Reserved(v) => v,
        }
    }
}

impl From<u8> for PowerMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotReady,
            0x01 => Self::Ready,
            0x02 => Self::NotSupported,
            _ => {
                log::warn!("ISO 13400-2 - used reserved power mode: {}", value);
                Self::Reserved(value)
            },
        }
    }
}

/// Table 47 — Routing activation request activation types
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum RoutingActiveType {
    #[default]
    Default = 0x00,
    WWHODB = 0x01,
    Reserved(u8),   // 0x02 ~ 0xDF
    CentralSecurity = 0xE0,
    VMSpecific(u8),        // 0xE1 ~ 0xFF
}

impl Into<u8> for RoutingActiveType {
    fn into(self) -> u8 {
        match self {
            Self::Default => 0x00,
            Self::WWHODB => 0x01,
            Self::Reserved(v) => v,
            Self::CentralSecurity => 0xE0,
            Self::VMSpecific(v) => v,
        }
    }
}

impl From<u8> for RoutingActiveType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Default,
            0x01 => Self::WWHODB,
            0x02..=0xDF => {
                log::warn!("ISO 13400-2 - used reserved routing active type: {}", value);
                Self::Reserved(value)
            },
            0xE0 => Self::CentralSecurity,
            _ => Self::VMSpecific(value),
        }
    }
}

/// Table 24 — Diagnostic message positive acknowledge codes
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum DiagnosticPositiveCode {
    #[default]
    Confirm = 0x00,
    Reserved(u8),
}

impl Into<u8> for DiagnosticPositiveCode {
    fn into(self) -> u8 {
        match self {
            Self::Confirm => 0x00,
            Self::Reserved(v) => v,
        }
    }
}

impl From<u8> for DiagnosticPositiveCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Confirm,
            _ => {
                log::warn!("ISO 13400-2 - used reserved diagnostic positive code: {}", value);
                Self::Reserved(value)
            },
        }
    }
}

impl Display for DiagnosticPositiveCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Confirm => write!(f, "Diagnostic Positive Confirm"),
            Self::Reserved(v) => write!(f, "Diagnostic Positive Reserved({})", v),
        }
    }
}

/// Table 26 — Diagnostic message negative acknowledge codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DiagnosticNegativeCode {
    InvalidSourceAddress = 0x02,
    UnknownTargetAddress = 0x03,
    DiagnosticMessageTooLarge = 0x04,
    OutOfMemory = 0x05,
    TargetUnreachable = 0x06,
    UnknownNetwork = 0x07,
    TransportProtocolError = 0x08,
    Reserved(u8),
}

impl Into<u8> for DiagnosticNegativeCode {
    fn into(self) -> u8 {
        match self {
            Self::InvalidSourceAddress => 0x02,
            Self::UnknownTargetAddress => 0x03,
            Self::DiagnosticMessageTooLarge => 0x04,
            Self::OutOfMemory => 0x05,
            Self::TargetUnreachable => 0x06,
            Self::UnknownNetwork => 0x07,
            Self::TransportProtocolError => 0x08,
            Self::Reserved(v) => v,
        }
    }
}

impl From<u8> for DiagnosticNegativeCode {
    fn from(value: u8) -> Self {
        match value {
            0x02 => Self::InvalidSourceAddress,
            0x03 => Self::UnknownTargetAddress,
            0x04 => Self::DiagnosticMessageTooLarge,
            0x05 => Self::OutOfMemory,
            0x06 => Self::TargetUnreachable,
            0x07 => Self::UnknownNetwork,
            0x08 => Self::TransportProtocolError,
            _ => {
                log::warn!("ISO 13400-2 - used reserved diagnostic negative code: {}", value);
                Self::Reserved(value)
            },
        }
    }
}

impl Display for DiagnosticNegativeCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSourceAddress => write!(f, "Diagnostic Negative Source Address"),
            Self::UnknownTargetAddress => write!(f, "Diagnostic Negative Target Address"),
            Self::DiagnosticMessageTooLarge => write!(f, "Diagnostic Negative Diagnostic Message Too Large"),
            Self::OutOfMemory => write!(f, "Diagnostic Negative Target Address"),
            Self::TargetUnreachable => write!(f, "Diagnostic Negative Target Address"),
            Self::UnknownNetwork => write!(f, "Diagnostic Negative Target Address"),
            Self::TransportProtocolError => write!(f, "Diagnostic Negative Transport Protocol Error"),
            Self::Reserved(v) => write!(f, "Diagnostic Negative Reserved({})", v),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RequestPayload {
    VehicleId(request::VehicleID),              // UDP 0x0001
    VehicleWithEid(request::VehicleIDWithEID),  // UDP 0x0002
    VehicleWithVIN(request::VehicleIDWithVIN),  // UDP 0x0003
    RoutingActive(request::RoutingActive),      // TCP 0x0005
    AliveCheck(request::AliveCheck),            // TCP 0x0007
    EntityStatue(request::EntityStatus),        // UDP 0x4001
    DiagPowerMode(request::DiagnosticPowerMode),// UDP 0x4003
    Diagnostic(request::Diagnostic),            // TCP 0x8001
}

#[derive(Debug, Clone)]
pub enum ResponsePayload {
    HeaderNegative(response::HeaderNegative),       // UDP/TCP 0x0000
    VehicleId(response::VehicleID),                 // UDP 0x0004
    RoutingActive(response::RoutingActive),         // TCP 0x0006
    AliveCheck(response::AliveCheck),               // TCP 0x0008
    EntityStatue(response::EntityStatus),           // UDP 0x4002
    DiagPowerMode(response::DiagnosticPowerMode),   // UDP 0x4004
    DiagPositive(response::DiagnosticPositive),     // TCP 0x8002
    DiagNegative(response::DiagnosticNegative),     // TCP 0x8003
}

/// Table 17 — Overview of DoIP payload types at line #49(ISO 13400-2-2019)
#[derive(Debug, Clone)]
pub enum Payload {
    Request(RequestPayload),
    Response(ResponsePayload),
}

#[derive(Debug, Clone)]
pub struct Message {
    pub version: Version,
    pub payload: Payload,
}

impl Into<Vec<u8>> for Message {
    fn into(self) -> Vec<u8> {
        let mut result: Vec<_> = self.version.into();
        match self.payload {
            Payload::Request(v) => {
                match v {
                    RequestPayload::VehicleId(v) => result.append(&mut v.into()),
                    RequestPayload::VehicleWithEid(v) => result.append(&mut v.into()),
                    RequestPayload::VehicleWithVIN(v) => result.append(&mut v.into()),
                    RequestPayload::RoutingActive(v) => result.append(&mut v.into()),
                    RequestPayload::AliveCheck(v) => result.append(&mut v.into()),
                    RequestPayload::EntityStatue(v) => result.append(&mut v.into()),
                    RequestPayload::DiagPowerMode(v) => result.append(&mut v.into()),
                    RequestPayload::Diagnostic(v) => result.append(&mut v.into()),
                }
            }
            Payload::Response(v) => match v {
                ResponsePayload::HeaderNegative(v) => result.append(&mut v.into()),
                ResponsePayload::VehicleId(v) => result.append(&mut v.into()),
                ResponsePayload::RoutingActive(v) => result.append(&mut v.into()),
                ResponsePayload::AliveCheck(v) => result.append(&mut v.into()),
                ResponsePayload::EntityStatue(v) => result.append(&mut v.into()),
                ResponsePayload::DiagPowerMode(v) => result.append(&mut v.into()),
                ResponsePayload::DiagPositive(v) => result.append(&mut v.into()),
                ResponsePayload::DiagNegative(v) => result.append(&mut v.into()),
            }
        }

        result
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        log::debug!("ISO 13400-2 - parsing data: {}", hex::encode(data));
        let data_len = data.len();
        let expected = SIZE_OF_VERSION + SIZE_OF_DATA_TYPE + SIZE_OF_LENGTH;
        if data_len < expected {
            return Err(Iso13400Error::InvalidLength { actual: data_len, expected });
        }

        let mut offset = 0;
        let version = Version::try_from(&data[..])?;
        offset += SIZE_OF_VERSION;
        let payload_type = u16::from_be_bytes(data[offset..offset+SIZE_OF_DATA_TYPE].try_into().unwrap());
        offset += SIZE_OF_DATA_TYPE;
        let payload_len = u32::from_be_bytes(data[offset..offset+SIZE_OF_LENGTH].try_into().unwrap());
        offset += SIZE_OF_LENGTH;
        let expected = data_len - offset;
        if (payload_len as usize) != expected {
            return Err(Iso13400Error::InvalidPayloadLength { actual: payload_len as usize, expected });
        }
        let payload = match payload_type {
            HEADER_NEGATIVE => Ok(Payload::Response(ResponsePayload::HeaderNegative(
                response::HeaderNegative::try_from(&data[offset..])?
            ))),
            UDP_REQ_VEHICLE_IDENTIFIER => Ok(Payload::Request(RequestPayload::VehicleId(
                request::VehicleID::try_from(&data[offset..])?
            ))),
            UDP_REQ_VEHICLE_ID_WITH_EID => Ok(Payload::Request(RequestPayload::VehicleWithEid(
                request::VehicleIDWithEID::try_from(&data[offset..])?
            ))),
            UDP_REQ_VEHICLE_ID_WITH_VIN => Ok(Payload::Request(RequestPayload::VehicleWithVIN(
                request::VehicleIDWithVIN::try_from(&data[offset..])?
            ))),
            UDP_RESP_VEHICLE_IDENTIFIER => Ok(Payload::Response(ResponsePayload::VehicleId(
                response::VehicleID::try_from(&data[offset..])?
            ))),
            TCP_REQ_ROUTING_ACTIVE => Ok(Payload::Request(RequestPayload::RoutingActive(
                request::RoutingActive::try_from(&data[offset..])?
            ))),
            TCP_RESP_ROUTING_ACTIVE => Ok(Payload::Response(ResponsePayload::RoutingActive(
                response::RoutingActive::try_from(&data[offset..])?
            ))),
            TCP_REQ_ALIVE_CHECK => Ok(Payload::Request(RequestPayload::AliveCheck(
                request::AliveCheck::try_from(&data[offset..])?
            ))),
            TCP_RESP_ALIVE_CHECK => Ok(Payload::Response(ResponsePayload::AliveCheck(
                response::AliveCheck::try_from(&data[offset..])?
            ))),
            UDP_REQ_ENTITY_STATUS => Ok(Payload::Request(RequestPayload::EntityStatue(
                request::EntityStatus::try_from(&data[offset..])?
            ))),
            UDP_RESP_ENTITY_STATUS => Ok(Payload::Response(ResponsePayload::EntityStatue(
                response::EntityStatus::try_from(&data[offset..])?
            ))),
            UDP_REQ_DIAGNOSTIC_POWER_MODE => Ok(Payload::Request(RequestPayload::DiagPowerMode(
                request::DiagnosticPowerMode::try_from(&data[offset..])?
            ))),
            UDP_RESP_DIAGNOSTIC_POWER_MODE => Ok(Payload::Response(ResponsePayload::DiagPowerMode(
                response::DiagnosticPowerMode::try_from(&data[offset..])?
            ))),
            TCP_REQ_DIAGNOSTIC => Ok(Payload::Request(RequestPayload::Diagnostic(
                request::Diagnostic::try_from(&data[offset..])?
            ))),
            TCP_RESP_DIAGNOSTIC_POSITIVE => Ok(Payload::Response(ResponsePayload::DiagPositive(
                response::DiagnosticPositive::try_from(&data[offset..])?
            ))),
            TCP_RESP_DIAGNOSTIC_NEGATIVE => Ok(Payload::Response(ResponsePayload::DiagNegative(
                response::DiagnosticNegative::try_from(&data[offset..])?
            ))),
            _ => Err(Iso13400Error::InvalidPayloadType(payload_type)),
        }?;

        Ok(Self { version, payload })
    }
}
