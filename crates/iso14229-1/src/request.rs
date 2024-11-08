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
mod request_download;       // 0x34
pub use request_download::*;
mod request_upload;         // 0x35
pub use request_upload::*;
mod transfer_data;          // 0x36
pub use transfer_data::*;
mod request_transfer_exit;  // 0x37
pub use request_transfer_exit::*;
#[cfg(any(feature = "std2013", feature = "std2020"))]
mod request_file_transfer;  // 0x38
#[cfg(any(feature = "std2013", feature = "std2020"))]
pub use request_file_transfer::*;

use crate::{Configuration, error::UdsError, RequestData, Service, utils, request, TryFromWithCfg};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SubFunction {
    function: u8,
    suppress_positive: Option<bool>,
}

impl SubFunction {
    pub fn new(
        function: u8,
        suppress_positive: Option<bool>,
    ) -> Self {
        Self {
            function,
            suppress_positive,
        }
    }

    #[inline]
    pub fn function<T: TryFrom<u8, Error =UdsError>>(&self) -> Result<T, UdsError> {
        T::try_from(self.function)
    }

    #[inline]
    pub const fn is_suppress_positive(&self) -> Option<bool> {
        self.suppress_positive
    }
}

impl Into<u8> for SubFunction {
    fn into(self) -> u8 {
        let mut result = self.function;
        if let Some(v) = self.suppress_positive {
            if v {
                result |= 0x80;
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub(crate) service: Service,
    pub(crate) sub_func: Option<SubFunction>,
    pub(crate) data: Vec<u8>,
}

impl Request {
    pub fn new(
        service: Service,
        sub_func: Option<SubFunction>,
        data: Vec<u8>,
        cfg: &Configuration,
    ) -> Result<Self, UdsError> {
        match service {
            Service::SessionCtrl => request::session_ctrl(service, sub_func, data, cfg),
            Service::ECUReset => request::ecu_reset(service, sub_func, data, cfg),
            Service::ClearDiagnosticInfo => request::clear_diag_info(service, sub_func, data, cfg),
            Service::ReadDTCInfo => request::read_dtc_info(service, sub_func, data, cfg),
            Service::ReadDID => request::read_did(service, sub_func, data, cfg),
            Service::ReadMemByAddr => request::read_mem_by_addr(service, sub_func, data, cfg),
            Service::ReadScalingDID => request::read_scaling_did(service, sub_func, data, cfg),
            Service::SecurityAccess => request::security_access(service, sub_func, data, cfg),
            Service::CommunicationCtrl => request::communication_ctrl(service, sub_func, data, cfg),
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => request::authentication(service, sub_func, data, cfg),
            Service::ReadDataByPeriodId => request::read_data_by_pid(service, sub_func, data, cfg),
            Service::DynamicalDefineDID => request::dyn_define_did(service, sub_func, data, cfg),
            Service::WriteDID => request::write_did(service, sub_func, data, cfg),
            Service::IOCtrl => request::io_ctrl(service, sub_func, data, cfg),
            Service::RoutineCtrl => request::routine_ctrl(service, sub_func, data, cfg),
            Service::RequestDownload => request::request_download(service, sub_func, data, cfg),
            Service::RequestUpload => request::request_upload(service, sub_func, data, cfg),
            Service::TransferData => request::transfer_data(service, sub_func, data, cfg),
            Service::RequestTransferExit => request::request_transfer_exit(service, sub_func, data, cfg),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => request::request_file_transfer(service, sub_func, data, cfg),
            Service::WriteMemByAddr => request::write_mem_by_addr(service, sub_func, data, cfg),
            Service::TesterPresent => request::tester_present(service, sub_func, data, cfg),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Service::AccessTimingParam => request::access_timing_param(service, sub_func, data, cfg),
            Service::SecuredDataTrans => request::secured_data_trans(service, sub_func, data, cfg),
            Service::CtrlDTCSetting => request::ctrl_dtc_setting(service, sub_func, data, cfg),
            Service::ResponseOnEvent => request::response_on_event(service, sub_func, data, cfg),
            Service::LinkCtrl => request::link_ctrl(service, sub_func, data, cfg),
            Service::NRC => Err(UdsError::OtherError("got an NRC service from request".into())),
        }
    }

    #[inline]
    pub fn service(&self) -> Service {
        self.service
    }

    #[inline]
    pub fn sub_function(&self) -> Option<SubFunction> {
        self.sub_func
    }

    #[inline]
    pub fn raw_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    #[inline]
    pub fn data<F, T>(&self, cfg: &Configuration) -> Result<T, UdsError>
    where
        F: TryFrom<u8, Error =UdsError>,
        T: RequestData<SubFunc = F>,
    {
        T::try_parse(self.data.as_slice(), match self.sub_func {
            Some(v) => Some(F::try_from(v.function)?),
            None => None,
        }, cfg)
    }
}

impl Into<Vec<u8>> for Request {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.service.into(), ];
        if let Some(sub_func) = self.sub_func {
            result.push(sub_func.into());
        }

        result.append(&mut self.data);

        result
    }
}

impl TryFromWithCfg<Vec<u8>> for Request {
    type Error = UdsError;
    fn try_from_cfg(data: Vec<u8>, cfg: &Configuration) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 1, false)?;

        let mut offset = 0;
        let service = Service::try_from(data[offset])?;
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
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(data[offset]);
                let sub_func = SubFunction::new(sub_func, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Request::new(service, Some(sub_func), data, cfg)
            },
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Service::AccessTimingParam => {
                utils::data_length_check(data_len, offset + 1, false)?;
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(data[offset]);
                let sub_func = SubFunction::new(sub_func, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Request::new(service, Some(sub_func), data, cfg)
            },
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => {
                utils::data_length_check(data_len, offset + 1, false)?;
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(data[offset]);
                let sub_func = SubFunction::new(sub_func, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Request::new(service, Some(sub_func), data, cfg)
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => {
                utils::data_length_check(data_len, offset + 1, false)?;
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(data[offset]);
                let sub_func = SubFunction::new(sub_func, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Request::new(service, Some(sub_func), data, cfg)
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
            Service::ResponseOnEvent => Self::new(service, None, data[offset..].to_vec(), cfg),
            Service::NRC => Err(UdsError::OtherError("got an NRC service from request".into())),
        }
    }
}
