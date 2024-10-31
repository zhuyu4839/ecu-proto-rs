use crate::{request, response, Error};
use crate::constant::{HEADER_NEGATIVE, TCP_REQ_ALIVE_CHECK, TCP_REQ_DIAGNOSTIC, TCP_REQ_ROUTING_ACTIVE, TCP_RESP_ALIVE_CHECK, TCP_RESP_DIAGNOSTIC_NEGATIVE, TCP_RESP_DIAGNOSTIC_POSITIVE, TCP_RESP_ROUTING_ACTIVE, UDP_REQ_DIAGNOSTIC_POWER_MODE, UDP_REQ_ENTITY_STATUS, UDP_REQ_VEHICLE_IDENTIFIER, UDP_REQ_VEHICLE_ID_WITH_EID, UDP_REQ_VEHICLE_ID_WITH_VIN, UDP_RESP_DIAGNOSTIC_POWER_MODE, UDP_RESP_ENTITY_STATUS, UDP_RESP_VEHICLE_IDENTIFIER};

/// Table 16 — Generic DoIP header structure at line #48(ISO 13400-2-2019)
#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Version {
    ISO13400_2_2010 = 0x01,
    ISO13400_2_2012 = 0x02,
    ISO13400_2_2019 = 0x03,
    Reserved(u8),
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
    type Error = Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < 2 {
            return Err(Error::InvalidLength { actual: data_len, expected: 2 });
        }

        let version = data[0];
        let reverse = data[1];
        if !version != reverse {
            return Err(Error::InvalidVersion { version, reverse });
        }

        Ok(Self::from(version))
    }
}

/// Table 19 — Generic DoIP header NACK codes at line #52(ISO 13400-2-2019)
#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Entity {
    Gateway = 0x00,
    Node = 0x01,
    Reserved(u8),
}

impl Into<u8> for Entity {
    fn into(self) -> u8 {
        match self {
            Self::Gateway => 0x00,
            Self::Node => 0x01,
            Self::Reserved(v) => v,
        }
    }
}

impl From<u8> for Entity {
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

/// Table 6 — Definition of further action code values
#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ActiveCode {
    SourceAddressUnknown = 0x00,    // close TCP
    Activated = 0x01,       // close TCP
    SourceAddressInvalid = 0x02,    // close TCP
    SocketInvalid = 0x03,   // close TCP
    WithoutAuth = 0x04,
    VehicleRefused = 0x05,  // close TCP
    Unsupported = 0x06,     // close TCP
    /// ISO 14300-2 2019
    Denied = 0x07,
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
            Self::Denied => 0x07,
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
            0x07 => Self::Denied,
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

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[repr(u8)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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

#[repr(u8)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone)]
pub enum Payload {
    /// UDP/TCP 0x0000
    HeaderNegativeAck(response::HeaderNegative),
    /// UDP 0x0001
    VehicleIdentificationRequest(request::VehicleID),
    /// UDP 0x0002
    VehicleIdentificationRequestWithEID(request::VehicleIDWithEID),
    /// UDP 0x0003
    VehicleIdentificationRequestWithVIN(request::VehicleIDWithVIN),
    /// UDP 0x0004
    VehicleIdentificationResponse(response::VehicleID),
    /// TCP 0x0005
    RoutingActivationRequest(request::RoutingActive),
    /// TCP 0x0006
    RoutingActivationResponse(response::RoutingActive),
    /// TCP 0x0007
    AliveCheckRequest(request::AliveCheck),
    /// TCP 0x0008
    AliveCheckResponse(response::AliveCheck),
    /// UDP 0x4001
    EntityStatusRequest(request::EntityStatus),
    /// UDP 0x4002
    EntityStatusResponse(response::EntityStatus),
    /// UDP 0x4003
    DiagnosticPowerModeRequest(request::DiagnosticPowerMode),
    /// UDP 0x4004
    DiagnosticPowerModeResponse(response::DiagnosticPowerMode),
    /// TCP 0x8001
    Diagnostic(request::Diagnostic),
    /// TCP 0x8002
    DiagnosticPositive(response::DiagnosticPositive),
    /// TCP 0x8003
    DiagnosticNegative(response::DiagnosticNegative),
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
            Payload::HeaderNegativeAck(v) => result.append(&mut v.into()),
            Payload::VehicleIdentificationRequest(v) => result.append(&mut v.into()),
            Payload::VehicleIdentificationRequestWithEID(v) => result.append(&mut v.into()),
            Payload::VehicleIdentificationRequestWithVIN(v) => result.append(&mut v.into()),
            Payload::VehicleIdentificationResponse(v) => result.append(&mut v.into()),
            Payload::RoutingActivationRequest(v) => result.append(&mut v.into()),
            Payload::RoutingActivationResponse(v) => result.append(&mut v.into()),
            Payload::AliveCheckRequest(v) => result.append(&mut v.into()),
            Payload::AliveCheckResponse(v) => result.append(&mut v.into()),
            Payload::EntityStatusRequest(v) => result.append(&mut v.into()),
            Payload::EntityStatusResponse(v) => result.append(&mut v.into()),
            Payload::DiagnosticPowerModeRequest(v) => result.append(&mut v.into()),
            Payload::DiagnosticPowerModeResponse(v) => result.append(&mut v.into()),
            Payload::Diagnostic(v) => result.append(&mut v.into()),
            Payload::DiagnosticPositive(v) => result.append(&mut v.into()),
            Payload::DiagnosticNegative(v) => result.append(&mut v.into()),
        }

        result
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < 4 {   // 2 * version + data type
            return Err(Error::InvalidLength { actual: data_len, expected: 4 });
        }

        let mut offset = 0;
        let version = Version::try_from(&data[..offset+2])?;
        offset += 2;
        let payload_type = u16::from_be_bytes(data[offset..offset+2].try_into().unwrap());
        offset += 2;
        let payload = match payload_type {
            HEADER_NEGATIVE =>
                Ok(Payload::HeaderNegativeAck(response::HeaderNegative::try_from(&data[offset..])?)),
            UDP_REQ_VEHICLE_IDENTIFIER =>
                Ok(Payload::VehicleIdentificationRequest(request::VehicleID::try_from(&data[offset..])?)),
            UDP_REQ_VEHICLE_ID_WITH_EID =>
                Ok(Payload::VehicleIdentificationRequestWithEID(request::VehicleIDWithEID::try_from(&data[offset..])?)),
            UDP_REQ_VEHICLE_ID_WITH_VIN =>
                Ok(Payload::VehicleIdentificationRequestWithVIN(request::VehicleIDWithVIN::try_from(&data[offset..])?)),
            UDP_RESP_VEHICLE_IDENTIFIER =>
                Ok(Payload::VehicleIdentificationResponse(response::VehicleID::try_from(&data[offset..])?)),
            TCP_REQ_ROUTING_ACTIVE =>
                Ok(Payload::RoutingActivationRequest(request::RoutingActive::try_from(&data[offset..])?)),
            TCP_RESP_ROUTING_ACTIVE =>
                Ok(Payload::RoutingActivationResponse(response::RoutingActive::try_from(&data[offset..])?)),
            TCP_REQ_ALIVE_CHECK =>
                Ok(Payload::AliveCheckRequest(request::AliveCheck::try_from(&data[offset..])?)),
            TCP_RESP_ALIVE_CHECK =>
                Ok(Payload::AliveCheckResponse(response::AliveCheck::try_from(&data[offset..])?)),
            UDP_REQ_ENTITY_STATUS =>
                Ok(Payload::EntityStatusRequest(request::EntityStatus::try_from(&data[offset..])?)),
            UDP_RESP_ENTITY_STATUS =>
                Ok(Payload::EntityStatusResponse(response::EntityStatus::try_from(&data[offset..])?)),
            UDP_REQ_DIAGNOSTIC_POWER_MODE =>
                Ok(Payload::DiagnosticPowerModeRequest(request::DiagnosticPowerMode::try_from(&data[offset..])?)),
            UDP_RESP_DIAGNOSTIC_POWER_MODE =>
                Ok(Payload::DiagnosticPowerModeResponse(response::DiagnosticPowerMode::try_from(&data[offset..])?)),
            TCP_REQ_DIAGNOSTIC =>
                Ok(Payload::Diagnostic(request::Diagnostic::try_from(&data[offset..])?)),
            TCP_RESP_DIAGNOSTIC_POSITIVE =>
                Ok(Payload::DiagnosticPositive(response::DiagnosticPositive::try_from(&data[offset..])?)),
            TCP_RESP_DIAGNOSTIC_NEGATIVE =>
                Ok(Payload::DiagnosticNegative(response::DiagnosticNegative::try_from(&data[offset..])?)),
            _ => Err(Error::InvalidPayloadType(payload_type)),
        }?;

        Ok(Self { version, payload })
    }
}