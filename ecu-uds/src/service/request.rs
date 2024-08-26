#![allow(non_snake_case, unused_imports)]

/* - Diagnostic and communication management functional unit - */
mod SessionCtrl;

use std::collections::HashMap;
use std::sync::Arc;
use isotp_rs::ByteOrder;
// 0x10
pub use SessionCtrl::*;
mod ECUReset;           // 0x11
pub use ECUReset::*;
mod SecurityAccess;     // 0x27
pub use SecurityAccess::*;
mod CommunicationCtrl;  // 0x28
pub use CommunicationCtrl::*;
#[cfg(any(feature = "std2020"))]
mod Authentication;     // 0x29
#[cfg(any(feature = "std2020"))]
pub use Authentication::*;
mod TesterPresent;      // 0x3E
pub use TesterPresent::*;
#[cfg(any(feature = "std2006", feature = "std2013"))]
mod AccessTimingParam;  // 0x83
#[cfg(any(feature = "std2006", feature = "std2013"))]
pub use AccessTimingParam::*;
mod SecuredDataTrans;   // 0x84
pub use SecuredDataTrans::*;
mod CtrlDTCSetting;     // 0x85
pub use CtrlDTCSetting::*;
mod ResponseOnEvent;    // 0x86
pub use ResponseOnEvent::*;
mod LinkCtrl;           // 0x87
pub use LinkCtrl::*;

/* - Data transmission functional unit - */
mod ReadDID;            // 0x22
pub use ReadDID::*;
mod ReadMemByAddr;      // 0x23
pub use ReadMemByAddr::*;
mod ReadScalingDID;     // 0x24
pub use ReadScalingDID::*;
mod ReadDataByPeriodId; // 0x2A
pub use ReadDataByPeriodId::*;
mod DynamicalDefineDID; // 0x2C
pub use DynamicalDefineDID::*;
mod WriteDID;           // 0x2E
pub use WriteDID::*;
mod WriteMemByAddr;     // 0x3D
pub use WriteMemByAddr::*;

/* - Stored data transmission functional unit - */
mod ClearDiagnosticInfo;// 0x14
pub use ClearDiagnosticInfo::*;
mod ReadDTCInfo;        // 0x19
pub use ReadDTCInfo::*;

/* - InputOutput control functional unit - */
mod IOCtrl;             // 0x2F
pub use IOCtrl::*;

/* - Remote activation of routine functional unit - */
mod RoutineCtrl;        // 0x31
pub use RoutineCtrl::*;

/* - Upload download functional unit - */
mod RequestLoad;        // 0x34 | 0x35
pub use RequestLoad::*;
// mod RequestDownload;    // 0x34
// pub use RequestDownload::*;
// mod RequestUpload;      // 0x35
// pub use RequestUpload::*;
mod TransferData;       // 0x36
pub use TransferData::*;
mod RequestTransferExit;// 0x37
pub use RequestTransferExit::*;
#[cfg(any(feature = "std2013", feature = "std2020"))]
mod RequestFileTransfer;// 0x38
#[cfg(any(feature = "std2013", feature = "std2020"))]
pub use RequestFileTransfer::*;

use crate::error::Error;
use crate::service::{Configuration, DataIdentifier, RequestData, Service};
use crate::{utils, SecurityAlgo};

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
    pub fn new<T: Into<Vec<u8>>>(
        service: Service,
        sub_func: Option<SubFunction<F>>,
        request: T,
    ) -> Self {
        Self {
            service,
            sub_func,
            data: request.into(),
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
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::AccessTimingParam => {
                utils::data_length_check(data_len, offset + 1, false)?;
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(data[offset]);
                let sub_func = SubFunction::new(F::try_from(sub_func)?, Some(suppress_positive));

                offset += 1;
                let data = data[offset..].to_vec();

                Ok(Request::new(service, Some(sub_func), data))
            },
            Service::NRC => {
                todo!()
            },
        }
    }
}
