mod common;

use std::collections::HashMap;
pub use common::*;
pub mod request;
pub mod response;

use std::fmt::{Display, Formatter};
use isotp_rs::ByteOrder;
use crate::{enum_to_vec, SecurityAlgo};
use crate::error::Error;

enum_to_vec! (
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
        ReadScalingDID = 0x24,      // ⭕
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
    }, u8, Error, InvalidParam
);

impl Display for Service {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Service::SessionCtrl => write!(f, "DiagnosticSessionControl"),
            Service::ECUReset => write!(f, "ECUReset"),
            Service::ClearDiagnosticInfo => write!(f, "ClearDiagnosticInformation"),
            Service::ReadDTCInfo => write!(f, "ReadDTCInformation"),
            Service::ReadDID => write!(f, "ReadDataByIdentifier"),
            Service::ReadMemByAddr => write!(f, "ReadMemoryByAddress"),
            Service::ReadScalingDID => write!(f, "ReadScalingDataByIdentifier"),
            Service::SecurityAccess => write!(f, "SecurityAccess"),
            Service::CommunicationCtrl => write!(f, "CommunicationControl"),
            Service::Authentication => write!(f, "Authentication"),
            Service::ReadDataByPeriodId => write!(f, "ReadDataByPeriodicIdentifier"),
            Service::DynamicalDefineDID => write!(f, "DynamicalDefineDyIdentifier"),
            Service::WriteDID => write!(f, "WriteDataByIdentifier"),
            Service::IOCtrl => write!(f, "IOControl"),
            Service::RoutineCtrl => write!(f, "RoutineControl"),
            Service::RequestDownload => write!(f, "RequestDownload"),
            Service::RequestUpload => write!(f, "RequestUpload"),
            Service::TransferData => write!(f, "TransferData"),
            Service::RequestTransferExit => write!(f, "RequestTransferExit"),
            Service::RequestFileTransfer => write!(f, "RequestFileTransfer"),
            Service::WriteMemByAddr => write!(f, "WriteMemoryByAddress"),
            Service::TesterPresent => write!(f, "TesterPresent"),
            Service::AccessTimingParam => write!(f, "AccessTimingParam"),
            Service::SecuredDataTrans => write!(f, "SecuredDataTransmission"),
            Service::CtrlDTCSetting => write!(f, "ControlDTCSetting"),
            Service::ResponseOnEvent => write!(f, "ResponseOnEvent"),
            Service::LinkCtrl => write!(f, "LinkControl"),
            Service::NRC => write!(f, "Negative Response with Code"),
        }
    }
}

/// The sub-function placeholder
#[derive(Debug, Copy, Clone)]
pub struct Placeholder;

impl TryFrom<u8> for Placeholder{
    type Error = Error;
    #[inline]
    fn try_from(_: u8) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

impl Into<u8> for Placeholder {
    fn into(self) -> u8 {
        panic!("The placeholder sub-function is `None`. Should not call this!");
    }
}

#[derive(Default, Clone)]
pub struct Configuration {
    pub(crate) security_algo: Option<SecurityAlgo>,
    pub(crate) did_cfg: HashMap<DataIdentifier, usize>,
    pub(crate) bo_addr: ByteOrder,
    pub(crate) bo_mem_size: ByteOrder,
}

pub trait RequestData {
    type SubFunc;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait ResponseData {
    type SubFunc;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error>
    where
        Self: Sized;
}

