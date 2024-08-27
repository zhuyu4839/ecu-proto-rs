#![allow(unused_imports)]

/* - Diagnostic and communication management functional unit - */
mod session_ctrl;           // 0x10
pub use session_ctrl::*;
mod ecu_reset;              // 0x11
pub use ecu_reset::*;
mod security_access;        // 0x27
pub use security_access::*;
mod communication_ctrl;     // 0x28
pub use communication_ctrl::*;
#[cfg(any(feature = "std2020"))]
mod authentication;         // 0x29
#[cfg(any(feature = "std2020"))]
pub use authentication::*;
mod tester_present;         // 0x3E
pub use tester_present::*;
#[cfg(any(feature = "std2006", feature = "std2013"))]
mod access_timing_param;    // 0x83
#[cfg(any(feature = "std2006", feature = "std2013"))]
pub use access_timing_param::*;
mod secured_data_trans;     // 0x84
pub use secured_data_trans::*;
mod ctrl_dtc_setting;       // 0x85
pub use ctrl_dtc_setting::*;
mod response_on_event;      // 0x86
pub use response_on_event::*;
mod link_ctrl;              // 0x87
pub use link_ctrl::*;

/* - Data transmission functional unit - */
mod read_did;               // 0x22
pub use read_did::*;
mod read_mem_by_addr;       // 0x23
pub use read_mem_by_addr::*;
mod read_scaling_did;       // 0x24
pub use read_scaling_did::*;
mod read_data_by_pid;       // 0x2A
pub use read_data_by_pid::*;
mod dynamically_define_did; // 0x2C
pub use dynamically_define_did::*;
mod write_did;              // 0x2E
pub use write_did::*;
mod write_mem_by_addr;      // 0x3D
pub use write_mem_by_addr::*;

/* - Stored data transmission functional unit - */
mod clear_diagnostic_info;  // 0x14
pub use clear_diagnostic_info::*;
mod read_dtc_info;          // 0x19
pub use read_dtc_info::*;

/* - InputOutput control functional unit - */
mod io_ctrl;                // 0x2F
pub use io_ctrl::*;

/* - Remote activation of routine functional unit - */
mod routine_ctrl;           // 0x31
pub use routine_ctrl::*;

/* - Upload download functional unit - */
mod request_load;           // 0x34 | 0x35
pub use request_load::*;
mod transfer_data;          // 0x36
pub use transfer_data::*;
mod request_transfer_exit;  // 0x37
pub use request_transfer_exit::*;
#[cfg(any(feature = "std2013", feature = "std2020"))]
mod request_file_transfer;  // 0x38
#[cfg(any(feature = "std2013", feature = "std2020"))]
pub use request_file_transfer::*;

mod code;
pub use code::Code;

// #[cfg(any(feature = "std2006", feature = "std2013"))]
// pub(crate) use crate::service::response::AccessTimingParam::ACCESS_TIMING_PARAM_NEGATIVES;
// #[cfg(any(feature = "std2020"))]
// pub(crate) use crate::service::response::Authentication::AUTH_NEGATIVES;
// pub(crate) use crate::service::response::ClearDiagnosticInfo::CLEAR_DIAGNOSTIC_INFO_NEGATIVES;
// pub(crate) use crate::service::response::CommunicationCtrl::COMMUNICATION_CTRL_NEGATIVES;
// pub(crate) use crate::service::response::CtrlDTCSetting::CTRL_DTC_SETTING_NEGATIVES;
// pub(crate) use crate::service::response::DynamicalDefineDID::DYNAMICAL_DID_NEGATIVES;
// pub(crate) use crate::service::response::ECUReset::ECU_RESET_NEGATIVES;
// pub(crate) use crate::service::response::IOCtrl::IO_CTRL_NEGATIVES;
// pub(crate) use crate::service::response::LinkCtrl::LINK_CTRL_NEGATIVES;
// pub(crate) use crate::service::response::ReadDataByPeriodId::READ_DATA_BY_PERIOD_ID_NEGATIVES;
// pub(crate) use crate::service::response::ReadDID::READ_DID_NEGATIVES;
// pub(crate) use crate::service::response::ReadDTCInfo::READ_DTC_INFO_NEGATIVES;
// pub(crate) use crate::service::response::ReadMemByAddr::READ_MEM_BY_ADDR_NEGATIVES;
// pub(crate) use crate::service::response::ReadScalingDID::READ_SCALING_DID_NEGATIVES;
// pub(crate) use crate::service::response::RequestDownload::REQUEST_DOWNLOAD_NEGATIVES;
// #[cfg(any(feature = "std2013", feature = "std2020"))]
// pub(crate) use crate::service::response::RequestFileTransfer::REQUEST_FILE_TRANSFER_NEGATIVES;
// pub(crate) use crate::service::response::RequestTransferExit::REQUEST_TRANSFER_EXIT_NEGATIVES;
// pub(crate) use crate::service::response::RequestUpload::REQUEST_UPLOAD_NEGATIVES;
// pub(crate) use crate::service::response::ResponseOnEvent::RESPONSE_ON_EVENT_NEGATIVES;
// pub(crate) use crate::service::response::RoutineCtrl::ROUTINE_CTRL_NEGATIVES;
// pub(crate) use crate::service::response::SecuredDataTrans::SECURED_DATA_TRANS_NEGATIVES;
// pub(crate) use crate::service::response::SecurityAccess::SECURITY_ACCESS_NEGATIVES;
// pub(crate) use crate::service::response::SessionCtrl::SESSION_CTRL_NEGATIVES;
// pub(crate) use crate::service::response::TesterPresent::TESTER_PRESENT_NEGATIVES;
// pub(crate) use crate::service::response::TransferData::TRANSFER_DATA_NEGATIVES;
// pub(crate) use crate::service::response::WriteDID::WRITE_DID_NEGATIVES;
// pub(crate) use crate::service::response::WriteMemByAddr::WRITE_MEM_BY_ADDR_NEGATIVES;

use crate::constant::POSITIVE_OFFSET;
use crate::utils;
use crate::error::Error;
use crate::service::{Configuration, ResponseData, Service};

// enum_to_vec! (
//     /// Defined by ISO-15764. Offset of 0x38 is defined within UDS standard (ISO-14229)
//     pub enum ISO15764 {
//         GeneralSecurityViolation = Code::SecureDataTransmissionRequired as u8 + 0,
//         SecuredModeRequested = Code::SecureDataTransmissionRequired as u8 + 1,
//         InsufficientProtection = Code::SecureDataTransmissionRequired as u8 + 2,
//         TerminationWithSignatureRequested = Code::SecureDataTransmissionRequired as u8 + 3,
//         AccessDenied = Code::SecureDataTransmissionRequired as u8 + 4,
//         VersionNotSupported = Code::SecureDataTransmissionRequired as u8 + 5,
//         SecuredLinkNotSupported = Code::SecureDataTransmissionRequired as u8 + 6,
//         CertificateNotAvailable = Code::SecureDataTransmissionRequired as u8 + 7,
//         AuditTrailInformationNotAvailable = Code::SecureDataTransmissionRequired as u8 + 8,
//     }, u8, Error, InvalidParam
// );

#[derive(Debug, Copy, Clone)]
pub struct SubFunction<T>(T);

impl<F: Copy> SubFunction<F> {
    pub fn new(
        function: F,
    ) -> Self {
        Self(function)
    }

    #[inline]
    pub fn function(&self) -> F {
        self.0
    }
}

impl<T: Into<u8>> Into<u8> for SubFunction<T> {
    fn into(self) -> u8 {
        self.0.into()
    }
}

#[derive(Debug, Clone)]
pub struct Response<F> {
    service: Service,
    negative: bool,
    sub_func: Option<SubFunction<F>>,
    data: Vec<u8>,  // the NRC code when negative is true
}

impl<F: Copy> Response<F> {
    pub fn new(
        service: Service,
        negative: bool,
        sub_func: Option<SubFunction<F>>,
        data: Vec<u8>,
    ) -> Self {
        Self { service, negative, sub_func, data, }
    }

    #[inline]
    pub fn service(&self) -> Service {
        self.service
    }

    #[inline]
    pub fn sub_function(&self) -> Option<SubFunction<F>> {
        self.sub_func
    }

    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    #[inline]
    pub fn nrc_code(&self) -> Result<Code, Error> {
        if !self.negative {
            return Err(Error::OtherError("get NRC from positive".into()));
        }

        if self.data.len() != 1 {
            return Err(Error::OtherError("invalid data length when getting NRC from negative".into()));
        }

        Code::try_from(self.data[0])
    }

    #[inline]
    pub fn raw_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    #[inline]
    pub fn data<T: ResponseData<SubFunc = F>>(&self, cfg: &Configuration) -> Result<T, Error> {
        T::try_parse(self.data.as_slice(), match self.sub_func {
            Some(v) => Some(v.0),
            None => None,
        }, cfg)
    }
}

impl<F: Into<u8>> Into<Vec<u8>> for Response<F> {
    fn into(mut self) -> Vec<u8> {
        let mut result = if self.negative {
            vec![Service::NRC.into(), ]
        }
        else {
            vec![]
        };

        let service: u8 = self.service.into();
        result.push(service | POSITIVE_OFFSET);

        if let Some(sub_func) = self.sub_func {
            result.push(sub_func.into());
        }

        result.append(&mut self.data);

        result
    }
}

impl<F: TryFrom<u8, Error = Error> + Copy> TryFrom<Vec<u8>> for Response<F> {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 1, false)?;

        let mut offset = 0;
        let service = data[offset];
        let service = if service == Service::NRC.into() {
            Ok(Service::NRC)
        }
        else {
            Service::try_from(service & !POSITIVE_OFFSET)
        }?;
        offset += 1;
        match service {
            Service::SessionCtrl |
            Service::ECUReset |
            Service::SecurityAccess |
            Service::CommunicationCtrl |
            Service::ReadDTCInfo |
            Service::RoutineCtrl |
            Service::CtrlDTCSetting |
            Service::TesterPresent |
            Service::LinkCtrl |
            Service::DynamicalDefineDID => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let sub_func = F::try_from(data[offset])?;
                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Self::new(service, false, Some(SubFunction::new(sub_func)), data))
            },
            Service::ClearDiagnosticInfo |
            Service::ReadDID |
            Service::ReadMemByAddr |
            Service::ReadScalingDID |
            Service::ReadDataByPeriodId |
            Service::WriteDID |
            Service::IOCtrl |
            Service::RequestDownload |
            Service::RequestUpload |
            Service::TransferData |
            Service::RequestTransferExit |
            Service::WriteMemByAddr |
            Service::SecuredDataTrans |
            Service::ResponseOnEvent => {
                Ok(Self::new(service, false, None, data[offset..].to_vec()))
            },
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let sub_func = F::try_from(data[offset])?;
                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Self::new(service, false, Some(SubFunction::new(sub_func)), data))
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let sub_func = F::try_from(data[offset])?;
                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Self::new(service, false, Some(SubFunction::new(sub_func)), data))
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::AccessTimingParam => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let sub_func = F::try_from(data[offset])?;
                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Self::new(service, false, Some(SubFunction::new(sub_func)), data))
            },
            Service::NRC => {
                utils::data_length_check(data_len, offset + 2, true)?;
                let nrc_service = Service::try_from(data[offset])?;
                offset += 1;

                let data = data[offset..].to_vec();

                Ok(Self::new(nrc_service, true, None, data))
            },
        }
    }
}
