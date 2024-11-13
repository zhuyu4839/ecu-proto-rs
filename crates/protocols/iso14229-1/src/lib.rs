mod common;
pub use common::*;
pub mod request;
pub mod response;
pub mod utils;
mod constant;
pub use constant::*;
mod error;
pub use error::*;

use std::{collections::HashMap, fmt::{Display, Formatter}};

enum_extend! (
    /// the service marked with `✅` is completed.
    ///
    /// the service marked with `⭕` is partially completed.
    ///
    /// The service marked with `❌` is not implemented.
    pub enum Service {
        SessionCtrl = 0x10,         // ✅
        ECUReset = 0x11,            // ✅
        ClearDiagnosticInfo = 0x14, // ✅
        ReadDTCInfo = 0x19,         // ⭕
        ReadDID = 0x22,             // ✅
        ReadMemByAddr = 0x23,       // ✅
        ReadScalingDID = 0x24,      // ✅
        SecurityAccess = 0x27,      // ✅
        CommunicationCtrl = 0x28,   // ✅
        #[cfg(any(feature = "std2020"))]
        Authentication = 0x29,      // ✅
        ReadDataByPeriodId = 0x2A,  // ✅
        DynamicalDefineDID = 0x2C,  // ✅
        WriteDID = 0x2E,            // ✅
        IOCtrl = 0x2F,              // ✅
        RoutineCtrl = 0x31,         // ✅
        RequestDownload = 0x34,     // ✅
        RequestUpload = 0x35,       // ✅
        TransferData = 0x36,        // ✅
        RequestTransferExit = 0x37, // ✅
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        RequestFileTransfer = 0x38, // ✅
        WriteMemByAddr = 0x3D,      // ✅
        TesterPresent = 0x3E,       // ✅
        #[cfg(any(feature = "std2006", feature = "std2013"))]
        AccessTimingParam = 0x83,   // ✅
        SecuredDataTrans = 0x84,    // ✅
        CtrlDTCSetting = 0x85,      // ✅
        ResponseOnEvent = 0x86,     // ❌
        LinkCtrl = 0x87,            // ✅
        NRC = 0x7F,
    }, u8);

impl Display for Service {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SessionCtrl => write!(f, "DiagnosticSessionControl"),
            Self::ECUReset => write!(f, "ECUReset"),
            Self::ClearDiagnosticInfo => write!(f, "ClearDiagnosticInformation"),
            Self::ReadDTCInfo => write!(f, "ReadDTCInformation"),
            Self::ReadDID => write!(f, "ReadDataByIdentifier"),
            Self::ReadMemByAddr => write!(f, "ReadMemoryByAddress"),
            Self::ReadScalingDID => write!(f, "ReadScalingDataByIdentifier"),
            Self::SecurityAccess => write!(f, "SecurityAccess"),
            Self::CommunicationCtrl => write!(f, "CommunicationControl"),
            #[cfg(any(feature = "std2020"))]
            Self::Authentication => write!(f, "Authentication"),
            Self::ReadDataByPeriodId => write!(f, "ReadDataByPeriodicIdentifier"),
            Self::DynamicalDefineDID => write!(f, "DynamicalDefineDyIdentifier"),
            Self::WriteDID => write!(f, "WriteDataByIdentifier"),
            Self::IOCtrl => write!(f, "IOControl"),
            Self::RoutineCtrl => write!(f, "RoutineControl"),
            Self::RequestDownload => write!(f, "RequestDownload"),
            Self::RequestUpload => write!(f, "RequestUpload"),
            Self::TransferData => write!(f, "TransferData"),
            Self::RequestTransferExit => write!(f, "RequestTransferExit"),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::RequestFileTransfer => write!(f, "RequestFileTransfer"),
            Self::WriteMemByAddr => write!(f, "WriteMemoryByAddress"),
            Self::TesterPresent => write!(f, "TesterPresent"),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::AccessTimingParam => write!(f, "AccessTimingParam"),
            Self::SecuredDataTrans => write!(f, "SecuredDataTransmission"),
            Self::CtrlDTCSetting => write!(f, "ControlDTCSetting"),
            Self::ResponseOnEvent => write!(f, "ResponseOnEvent"),
            Self::LinkCtrl => write!(f, "LinkControl"),
            Self::NRC => write!(f, "Negative Response with Code"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ByteOrder {
    Big,
    #[default]
    Little,
    Native,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub did_cfg: HashMap<DataIdentifier, usize>,
    pub bo_addr: ByteOrder,
    pub bo_mem_size: ByteOrder,
}

impl Default for Configuration {
    /// ISO 14229-2 default using big-endian.
    fn default() -> Self {
        Self {
            did_cfg: Default::default(),
            bo_addr: ByteOrder::Big,
            bo_mem_size: ByteOrder::Big,
        }
    }
}

pub trait RequestData {
    fn request(data: &[u8], sub_func: Option<u8>, cfg: &Configuration) -> Result<request::Request, UdsError>;
    fn try_parse(request: &request::Request, cfg: &Configuration) -> Result<Self, UdsError>
    where
        Self: Sized;
    fn to_vec(self, cfg: &Configuration) -> Vec<u8>;
}

pub trait ResponseData {
    fn response(data: &[u8], sub_func: Option<u8>, cfg: &Configuration) -> Result<response::Response, UdsError>;
    fn try_parse(response: &response::Response, cfg: &Configuration) -> Result<Self, UdsError>
    where
        Self: Sized;
    fn to_vec(self, cfg: &Configuration) -> Vec<u8>;
}

pub trait TryFromWithCfg<T> {
    type Error;
    fn try_from_cfg(data: T, cfg: &Configuration) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
