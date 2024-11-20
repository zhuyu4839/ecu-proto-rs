use iso14229_1::{Service, SessionType};
use crate::server::util;

/// Session manager.
#[derive(Debug, Default, Clone)]
pub struct SessionManager {
    pub(crate) session_type: SessionType,
    // TODO timer to set session to default
}

impl SessionManager {
    #[inline]
    pub fn change_session(&mut self, r#type: SessionType) {
        self.session_type = r#type;
    }

    #[inline]
    pub fn keep_session(&mut self) {
        // TODO
    }

    /// Check whether the current session supports the service type currently requested
    pub fn service_check(&self, service: u8) -> Option<Vec<u8>> {
        match Service::try_from(service) {
            Ok(service) => match service {
                Service::SessionCtrl => None,
                Service::ECUReset => None,
                Service::ClearDiagnosticInfo => todo!(),
                Service::ReadDTCInfo => None,
                Service::ReadDID => None,
                Service::ReadMemByAddr => todo!(),
                Service::ReadScalingDID => todo!(),
                Service::SecurityAccess => todo!(),
                Service::CommunicationCtrl => todo!(),
                Service::Authentication => todo!(),
                Service::ReadDataByPeriodId => todo!(),
                Service::DynamicalDefineDID => todo!(),
                Service::WriteDID => {
                    if self.session_type == SessionType::Default {
                        Some(util::service_not_support_in_session(service))
                    }
                    else {
                        None
                    }
                },
                Service::IOCtrl => todo!(),
                Service::RoutineCtrl => todo!(),
                Service::RequestDownload => todo!(),
                Service::RequestUpload => todo!(),
                Service::TransferData => todo!(),
                Service::RequestTransferExit => todo!(),
                Service::RequestFileTransfer => todo!(),
                Service::WriteMemByAddr => todo!(),
                Service::TesterPresent => todo!(),
                #[cfg(any(feature = "std2006", feature = "std2013"))]
                Service::AccessTimingParam => todo!(),
                Service::SecuredDataTrans => todo!(),
                Service::CtrlDTCSetting => todo!(),
                Service::ResponseOnEvent => todo!(),
                Service::LinkCtrl => todo!(),
                Service::NRC => Some(util::service_not_support(service.into())),
            },
            Err(_) => Some(util::service_not_support(service))
        }
    }
}
