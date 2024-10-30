pub(crate) mod constant;
pub mod request;
pub mod response;

use constant::*;

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
            _ => Self::Reserved(version),
        }
    }
}

impl TryFrom<&[u8]> for Version {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err(format!("invalid data length: {}", value.len()));
        }

        let version = value[0];
        let reverse = value[1];
        if reverse == !version {
            return Err(format!("invalid version: {} reverse: {}", version, reverse));
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
            _ => Self::Reserved(code),
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
    OEM(u16),           // 0x0001 ~ 0x0DFF | 0x1000 ~ 0x7FFF
    Client(u16),        // 0x0E00 ~ 0x0FFF
    OEMFunctional(u16), // 0xE400 ~ 0xEFFF (functional group logical addresses)
    Reserved(u16),      // 0x0000 | 0xE000 ~ 0xE3FF | 0xF000 ~ 0xFFFF
}

impl From<u16> for LogicAddress {
    fn from(val: u16) -> Self {
        match val {
            0x0001..=0x0DFF |
            0x1000..=0x7FFF => Self::OEM(val),
            0x0E00..=0x0FFF => Self::Client(val),
            0xE400..=0xEFFF => Self::OEMFunctional(val),
            _ => Self::Reserved(val),
        }
    }
}

impl Into<u16> for LogicAddress {
    fn into(self) -> u16 {
        match self {
            Self::OEM(v) => v,
            Self::Client(v) => v,
            Self::OEMFunctional(v) => v,
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
            _ => Self::Reserved(value),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FurtherAction {
    NoAction = 0x00,
    Reserved(u8),       // 0x01 ~ 0x0f
    CentralSecurity = 0x10,
    OEM(u8),            // 0x11 ~ 0xFF
}

impl Into<u8> for FurtherAction {
    fn into(self) -> u8 {
        match self {
            Self::NoAction => 0x00,
            Self::Reserved(v) => v,
            Self::CentralSecurity => 0x01,
            Self::OEM(v) => v,
        }
    }
}

impl From<u8> for FurtherAction {
    fn from(val: u8) -> Self {
        match val {
            0x00 => Self::NoAction,
            0x01..=0x0F => Self::Reserved(val),
            0x10 => Self::CentralSecurity,
            _ => Self::OEM(val),
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
    Success = 0x10,
    NeedConfirm = 0x11,
    OEM(u8),        // 0xE0 ~ 0xFE
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
            Self::Success => 0x10,
            Self::NeedConfirm => 0x11,
            Self::OEM(v) => v,
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
            0x10 => Self::Success,
            0x11 => Self::NeedConfirm,
            0xE0..=0xFE => Self::OEM(v),
            _ => Self::Reserved(v),
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
            _ => Self::Reserved(value),
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
    OEM(u8),        // 0xE1 ~ 0xFF
}

impl Into<u8> for RoutingActiveType {
    fn into(self) -> u8 {
        match self {
            Self::Default => 0x00,
            Self::WWHODB => 0x01,
            Self::Reserved(v) => v,
            Self::CentralSecurity => 0xE0,
            Self::OEM(v) => v,
        }
    }
}

impl From<u8> for RoutingActiveType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Default,
            0x01 => Self::WWHODB,
            0x02..=0xDF => Self::Reserved(value),
            0xE0 => Self::CentralSecurity,
            _ => Self::OEM(value),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DiagnosticCode {
    Positive = 0x00,
    SourceAddressUnknown = 0x02,
    TargetAddressUnknown = 0x03,
    OutOfLength = 0x04,
    OutOfMemory = 0x05,
    TargetNotAccessible = 0x06,
    NetworkUnknown = 0x07,
    ProtoError = 0x08,
    Reserved(u8),
}

impl Into<u8> for DiagnosticCode {
    fn into(self) -> u8 {
        match self {
            Self::Positive => 0x00,
            Self::SourceAddressUnknown => 0x02,
            Self::TargetAddressUnknown => 0x03,
            Self::OutOfLength => 0x04,
            Self::OutOfMemory => 0x05,
            Self::TargetNotAccessible => 0x06,
            Self::NetworkUnknown => 0x07,
            Self::ProtoError => 0x08,
            Self::Reserved(v) => v,
        }
    }
}

impl From<u8> for DiagnosticCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Positive,
            0x02 => Self::SourceAddressUnknown,
            0x03 => Self::TargetAddressUnknown,
            0x04 => Self::OutOfLength,
            0x05 => Self::OutOfMemory,
            0x06 => Self::TargetNotAccessible,
            0x07 => Self::NetworkUnknown,
            0x08 => Self::ProtoError,
            _ => Self::Reserved(value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Payload {
    HeaderNegativeAck(response::HeaderNegative),
    VehicleIdentificationRequest(request::VehicleID),
    VehicleIdentificationRequestWithEID(request::VehicleIDWithEID),
    VehicleIdentificationRequestWithVIN(request::VehicleIDWithVIN),
    VehicleIdentificationResponse(response::VehicleID),
    RoutingActivationRequest(request::RoutingActive),
    RoutingActivationResponse(response::RoutingActive),
    AliveCheckRequest(request::AliveCheck),
    AliveCheckResponse(response::AliveCheck),
    EntityStatusRequest(request::EntityStatus),
    EntityStatusResponse(response::EntityStatus),
    DiagnosticPowerModeRequest(request::DiagnosticPowerMode),
    DiagnosticPowerModeResponse(response::DiagnosticPowerMode),
    Diagnostic(request::Diagnostic),
    DiagnosticPositive(response::DiagnosticPositive),
    DiagnosticNegative(response::DiagnosticNegative),
}

#[derive(Debug, Clone)]
pub struct Message {
    pub(crate) version: Version,
    pub(crate) payload: Payload,
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
    type Error = String;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < 4 {   // 2 * version + data type
            return Err(format!("invalid message length: {}", data_len));
        }

        let mut offset = 0;
        let version = Version::try_from(&data[..2])?;
        offset += 2;
        let payload_type = u16::from_be_bytes([data[offset], data[offset + 1]]);
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
            _ => Err(format!("invalid payload type: {}", payload_type)),
        }?;

        Ok(Self { version, payload })
    }
}
