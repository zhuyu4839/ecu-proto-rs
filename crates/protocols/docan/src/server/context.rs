mod session_manager;
use session_manager::SessionManager;
mod diag_info_manager;
use diag_info_manager::DiagInfoManager;

use std::{fmt::Display, time::Duration, thread};
use iso14229_1::{request::{self, Request}, response::{self, Response, Code}, *};
use iso15765_2::{IsoTpEvent, IsoTpEventListener};
use rs_can::{Frame, isotp::{AddressType, CanIsoTp}};
use crate::{buffer::IsoTpBuffer, DoCanError, SecurityAlgo, server::util};

#[derive(Debug, Default, Clone)]
pub struct IsoTpListener {
    pub(crate) buffer: IsoTpBuffer,
}

impl IsoTpEventListener for IsoTpListener {
    #[inline]
    fn buffer_data(&mut self) -> Option<IsoTpEvent> {
        self.buffer.get()
    }

    #[inline]
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    #[inline]
    fn on_iso_tp_event(&mut self, event: IsoTpEvent) {
        self.buffer.set(event);
    }
}

#[derive(Clone)]
pub struct Context<C, F> {
    flag: bool,
    iso_tp: CanIsoTp<C, F>,
    listener: IsoTpListener,
    config: Configuration,
    security_algo: Option<SecurityAlgo>,
    session_manager: SessionManager,
}

impl<C, F> Context<C, F>
where
    C: Display + Clone,
    F: Frame<Channel = C>
{
    pub fn new(iso_tp: CanIsoTp<C, F>, listener: IsoTpListener) -> Self {
        Self {
            flag: true,
            iso_tp,
            listener,
            config: Default::default(),
            security_algo: Default::default(),
            session_manager: Default::default(),
        }
    }

    pub fn server(&mut self, interval: u64) -> Result<(), DoCanError> {
        while self.flag {
            if let Some(event) = self.listener.buffer.get() {
                match event {
                    IsoTpEvent::Wait => self.session_manager.keep_session(),
                    IsoTpEvent::FirstFrameReceived => {},   // nothing to do
                    IsoTpEvent::DataReceived(data) => {
                        log::trace!("DoCANServer - data received: {}", hex::encode(&data));
                        if data.is_empty() {
                            continue;
                        }

                        let service = data[0];
                        let data = match self.session_manager.service_check(service) {
                            Some(data) => Some(data),
                            None => match Request::try_from_cfg(data, &self.config) {
                                Ok(request) => {
                                    let service = request.service();
                                    match service {
                                        Service::SessionCtrl => {
                                            match request.sub_function() {
                                                Some(sub_func) => match sub_func.function::<SessionType>() {
                                                    Ok(r#type) => {
                                                        self.session_manager.change_session(r#type);
                                                        if sub_func.is_suppress_positive() {
                                                            None
                                                        } else {
                                                            let session_timing = response::SessionTiming::default();
                                                            Some(self.positive_response(service, Some(sub_func.into()), session_timing.into()))
                                                        }
                                                    },
                                                    Err(_) => Some(util::sub_func_not_support(service.into())),
                                                },
                                                None => Some(util::sub_func_not_support(service.into())),
                                            }
                                        },
                                        Service::ECUReset => {
                                            match request.sub_function() {
                                                Some(sub_func) => {
                                                    if sub_func.is_suppress_positive() {
                                                        None
                                                    } else {
                                                        let sub_func: ECUResetType = sub_func.function().unwrap();
                                                        let data = match sub_func {
                                                            ECUResetType::EnableRapidPowerShutDown => vec![1, ],
                                                            _ => vec![],
                                                        };

                                                        Some(self.positive_response(service, Some(sub_func.into()), data))
                                                    }
                                                },
                                                None => Some(util::sub_func_not_support(service.into())),
                                            }
                                        }
                                        Service::ClearDiagnosticInfo => {
                                            self.clear_diag_info();
                                            Some(self.positive_response(service, None, vec![]))
                                        }
                                        // Service::ReadDTCInfo => {}
                                        // Service::ReadDID => {}
                                        // Service::ReadMemByAddr => {}
                                        // Service::ReadScalingDID => {}
                                        Service::SecurityAccess => {
                                            None    // TODO
                                        }
                                        // Service::CommunicationCtrl => {}
                                        // #[cfg(any(feature = "std2020"))]
                                        // Service::Authentication => {}
                                        // Service::ReadDataByPeriodId => {}
                                        // Service::DynamicalDefineDID => {}
                                        // Service::WriteDID => {}
                                        // Service::IOCtrl => {}
                                        // Service::RoutineCtrl => {}
                                        // Service::RequestDownload => {}
                                        // Service::RequestUpload => {}
                                        // Service::TransferData => {}
                                        Service::RequestTransferExit => {
                                            None    // TODO
                                        }
                                        // #[cfg(any(feature = "std2013", feature = "std2020"))]
                                        // Service::RequestFileTransfer => {}
                                        // Service::WriteMemByAddr => {}
                                        Service::TesterPresent => {
                                            match request.sub_function() {
                                                Some(sub_func) => {
                                                    if sub_func.is_suppress_positive() {
                                                        self.session_manager.keep_session();
                                                        None
                                                    } else {
                                                        Some(self.positive_response(service, Some(sub_func.into()), vec![]))
                                                    }
                                                },
                                                None => Some(util::sub_func_not_support(service.into())),
                                            }
                                        }
                                        // #[cfg(any(feature = "std2006", feature = "std2013"))]
                                        // Service::AccessTimingParam => {}
                                        // Service::SecuredDataTrans => {}
                                        // Service::CtrlDTCSetting => {}
                                        // Service::ResponseOnEvent => {}
                                        // Service::LinkCtrl => {}
                                        _ => Some(util::service_not_support(service.into())),
                                    }
                                },
                                Err(err) => {
                                    log::warn!("DoCANServer - error: {} when parsing response", err);
                                    Some(vec![Service::NRC.into(), service, Code::GeneralReject.into()])
                                }
                            }
                        };

                        if let Some(data) = data {
                            self.iso_tp.write(AddressType::Physical, data)
                                .map_err(DoCanError::IsoTpError)?;
                        }
                    },
                    IsoTpEvent::ErrorOccurred(e) => {
                        log::warn!("DoCANServer - iso-tp error: {}", e);
                        // TODO
                    },
                }
            }

            thread::sleep(Duration::from_micros(interval));
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), DoCanError> {
        self.flag = false;

        Ok(())
    }

    #[inline]
    fn clear_diag_info(&mut self) {
        // TODO
    }

    #[inline]
    fn positive_response(&self, service: Service, sub_func: Option<u8>, data: Vec<u8>) -> Vec<u8> {
        match Response::new(service, sub_func, data, &self.config) {
            Ok(v) => v.into(),
            Err(_) => vec![Service::NRC.into(), service.into(), Code::GeneralReject.into()]
        }
    }
}
