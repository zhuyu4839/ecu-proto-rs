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

use crate::error::Error;
use crate::service::{Configuration, RequestData, Service};
use crate::utils;

#[derive(Debug, Copy, Clone)]
pub struct SubFunction<F> {
    function: F,
    suppress_positive: Option<bool>,
}

impl<F: Copy> SubFunction<F> {
    pub fn new(
        function: F,
        suppress_positive: Option<bool>,
    ) -> Self {
        Self {
            function,
            suppress_positive,
        }
    }

    #[inline]
    pub fn function(&self) -> F {
        self.function
    }

    #[inline]
    pub const fn is_suppress_positive(&self) -> Result<Option<bool>, Error> {
        Ok(self.suppress_positive)
    }
}

impl<F> Into<u8> for SubFunction<F>
where
    F: Into<u8> {
    fn into(self) -> u8 {
        if let Some(v) = self.suppress_positive {
            if v {
                return self.function.into() | 0x80
            }
        }

        self.function.into()
    }
}

#[derive(Debug, Clone)]
pub struct Request<F> {
    pub(crate) service: Service,
    pub(crate) sub_func: Option<SubFunction<F>>,
    pub(crate) data: Vec<u8>,
}

impl<F: Copy> Request<F> {
    pub fn new(
        service: Service,
        sub_func: Option<SubFunction<F>>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            service,
            sub_func,
            data,
        }
    }

    #[inline]
    pub fn service(&self) -> Service {
        self.service
    }

    #[inline]
    pub fn sub_function(&self) -> Option<SubFunction<F>> {
        self.sub_func.clone()
    }

    #[inline]
    pub fn raw_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    #[inline]
    pub fn data<T: RequestData<SubFunc = F>>(&self, cfg: &Configuration) -> Result<T, Error> {
        T::try_parse(self.data.as_slice(), match self.sub_func {
            Some(v) => Some(v.function),
            None => None,
        }, cfg)
    }
}

impl<F: Into<u8>> Into<Vec<u8>> for Request<F> {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.service.into(), ];
        if let Some(sub_func) = self.sub_func {
            result.push(sub_func.into());
        }

        result.append(&mut self.data);

        result
    }
}

impl<F: TryFrom<u8, Error = Error> + Copy> TryFrom<Vec<u8>> for Request<F> {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
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
                let sub_func = SubFunction::new(F::try_from(sub_func)?, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Request::new(service, Some(sub_func), data))
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
                Ok(Self::new(service, None, data[offset..].to_vec()))
            },
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => {
                utils::data_length_check(data_len, offset + 1, false)?;
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(data[offset]);
                let sub_func = SubFunction::new(F::try_from(sub_func)?, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Request::new(service, Some(sub_func), data))
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => {
                utils::data_length_check(data_len, offset + 1, false)?;
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(data[offset]);
                let sub_func = SubFunction::new(F::try_from(sub_func)?, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Request::new(service, Some(sub_func), data))
            },
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Service::AccessTimingParam => {
                utils::data_length_check(data_len, offset + 1, false)?;
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(data[offset]);
                let sub_func = SubFunction::new(F::try_from(sub_func)?, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Request::new(service, Some(sub_func), data))
            },
            Service::NRC => Err(Error::OtherError("got an NRC code from request data".into())),
        }
    }
}
