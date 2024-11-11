use std::{collections::HashMap, fmt::Display, hash::Hash, time::Duration};
use iso15765_2::{can::{Address, SyncCan, Frame, SyncCanIsoTp}, Driver, IsoTpError, IsoTpEventListener};
use iso14229_1::{response::{self, Response, Code}, request::{self, Request}, *};
use crate::{client::context::{Context, IsoTpListener}, Error, P2Context};

#[derive(Clone)]
pub struct SyncClient<D, C, F>
where
    C: Clone + Eq,
{
    driver: SyncCan<D, C, F>,
    context: HashMap<C, Context<C, F>>,
}

impl<D, C, F> SyncClient<D, C, F>
where
    D: Driver<C = C, F = F> + Clone + 'static,
    C: Display + Clone + Hash + Eq + 'static,
    F: Frame<Channel = C> + Clone + Send + Display + 'static
{
    pub fn new(driver: SyncCan<D, C, F>) -> Self {
        Self { driver, context: Default::default(), }
    }

    pub fn init_channel(&mut self,
                        channel: C,
                        address: Address,
                        p2_offset: Option<u16>,
    ) -> Result<(), Error> {
        let mut p2_ctx: P2Context = Default::default();
        if let Some(v) = p2_offset {
            p2_ctx.p2_offset = v;
        }

        let listener = IsoTpListener::new(p2_ctx);
        let iso_tp = SyncCanIsoTp::new(channel.clone(), address, self.driver.sender(), Box::new(listener.clone()));

        self.driver.register_listener(format!("UDS-{}", channel), Box::new(iso_tp.clone()));
        self.context.insert(channel, Context {
            iso_tp,
            listener,
            config: Default::default(),
        });

        Ok(())
    }
    #[inline]
    pub fn driver(&self) -> &SyncCan<D, C, F> {
        &self.driver
    }
    #[inline]
    pub fn update_address(&mut self, channel: C, address: Address) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            ctx.iso_tp.update_address(address);

            Ok(())
        })
    }
    #[inline]
    pub fn update_security_algo(&mut self, channel: C, algo: SecurityAlgo) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            ctx.config.security_algo = Some(algo);

            Ok(())
        })
    }
    #[inline]
    pub fn add_data_identifier(&mut self, channel: C, did: DataIdentifier, length: usize) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            ctx.config.did_cfg.insert(did, length);

            Ok(())
        })
    }
    #[inline]
    pub fn remove_data_identifier(&mut self, channel: C, did: DataIdentifier) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            ctx.config.did_cfg.remove(&did);

            Ok(())
        })
    }
    #[inline]
    pub fn set_address_of_byte_order(&mut self, channel: C, bo: ByteOrder) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            ctx.config.bo_addr = bo;

            Ok(())
        })
    }
    #[inline]
    pub fn set_memory_size_of_byte_order(&mut self, channel: C, bo: ByteOrder) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            ctx.config.bo_mem_size = bo;

            Ok(())
        })
    }

    /** - Diagnostic and communication management functional unit - **/
    pub fn session_ctrl(&mut self,
                        channel: C,
                        session_type: SessionType,
                        suppress_positive: bool,
                        functional: bool,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            let service = Service::SessionCtrl;
            let mut sub_func: u8 = session_type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), vec![], &ctx.config)
                .map_err(Error::ISO14229Error)?;

            if let Some(response) = Self::suppress_positive_sr(ctx, functional, request, suppress_positive)? {
                Self::sub_func_check(&response, session_type.into(), service)?;

                let timing = response.data::<response::SessionCtrl>(&ctx.config)
                    .map_err(Error::ISO14229Error)?
                    .0;
                ctx.listener.update_p2_ctx(timing.p2_ms(), timing.p2_star_ms());
            }

            Ok(())
        })
    }

    pub fn ecu_reset(&mut self,
                     channel: C,
                     reset_type: ECUResetType,
                     suppress_positive: bool,
                     functional: bool,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            let service = Service::ECUReset;
            let mut sub_func: u8 = reset_type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), vec![], &ctx.config)
                .map_err(Error::ISO14229Error)?;

            if let Some(response) = Self::suppress_positive_sr(ctx, functional, request, suppress_positive)? {
                Self::sub_func_check(&response, reset_type.into(), service)?;

                let resp = response.data::<response::ECUReset>(&ctx.config)
                    .map_err(Error::ISO14229Error)?;
                if let Some(seconds) = resp.second {
                    std::thread::sleep(Duration::from_secs(seconds as u64));
                }
            }

            Ok(())
        })
    }

    pub fn security_access(&mut self,
                           channel: C,
                           level: u8,
                           params: Vec<u8>,
    ) -> Result<Vec<u8>, Error> {
        self.context_util(channel, |ctx| {
            let service = Service::SecurityAccess;
            // let sub_func = request::SubFunction::new(level, None);
            // let data = SecurityAccessData(params);
            let request = Request::new(service, Some(level), params, &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            Self::sub_func_check(&response, level, service)?;

            Ok(response.raw_data().to_vec())
        })
    }

    pub fn unlock_security_access(&mut self,
                                  channel: C,
                                  level: u8,
                                  params: Vec<u8>,
                                  salt: Vec<u8>,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            if let Some(algo) = ctx.config.security_algo {
                let service = Service::SecurityAccess;
                // let sub_func = request::SubFunction::new(level, None);
                // let data = SecurityAccessData(params);
                let req = Request::new(service, Some(level), params.clone(), &ctx.config)
                    .map_err(Error::ISO14229Error)?;

                let resp = Self::send_and_response(ctx, false, req)?;
                Self::sub_func_check(&resp, level, service)?;

                let seed = resp.raw_data().to_vec();

                // let sub_func = request::SubFunction::new(level + 1, None);
                match algo(level, seed, salt)
                    .map_err(Error::ISO14229Error)? {
                    Some(data) => {
                        let request = Request::new(service, Some(level + 1), data, &ctx.config)
                            .map_err(Error::ISO14229Error)?;
                        let response = Self::send_and_response(ctx, false, request)?;

                        Self::sub_func_check(&response, level + 1, service)
                    },
                    None => Ok(())
                }
            }
            else {
                Err(Error::OtherError("security algorithm required".into()))
            }
        })
    }

    pub fn communication_control(&mut self,
                                 channel: C,
                                 ctrl_type: CommunicationCtrlType,
                                 comm_type: CommunicationType,
                                 node_id: Option<request::NodeId>,
                                 suppress_positive: bool,
                                 functional: bool,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            let service = Service::CommunicationCtrl;
            let mut sub_func = ctrl_type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let data = request::CommunicationCtrl::new(ctrl_type, comm_type, node_id)
                .map_err(Error::ISO14229Error)?;
            let req = Request::new(service, Some(sub_func), data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let resp = Self::suppress_positive_sr(ctx, functional, req, suppress_positive)?;

            if let Some(response) = resp {
                Self::sub_func_check(&response, ctrl_type.into(), service)?;
            }

            Ok(())
        })
    }

    #[cfg(any(feature = "std2020"))]
    pub fn authentication(&mut self,
                          channel: C,
                          auth_task: AuthenticationTask,
                          data: request::Authentication,
    ) -> Result<response::Authentication, Error> {
        self.context_util(channel, |ctx| {
            let service = Service::Authentication;
            let request = Request::new(service, Some(auth_task.into()), data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;
            Self::sub_func_check(&response, auth_task.into(), service)?;

            response.data::<response::Authentication>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    pub fn tester_present(&mut self,
                          channel: C,
                          test_type: TesterPresentType,
                          functional: bool,
                          suppress_positive: bool,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            let (service, request) =
                Self::tester_present_request(ctx, test_type, suppress_positive)?;

            let response = Self::suppress_positive_sr(ctx, functional, request, suppress_positive)?;

            if let Some(response) = response {
                Self::sub_func_check(&response, test_type.into(), service)?;
            }

            Ok(())
        })
    }

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    pub fn access_timing_parameter(&mut self,
                                   channel: C,
                                   access_type: TimingParameterAccessType,
                                   parameter: Vec<u8>,
                                   suppress_positive: bool,
    ) -> Result<Option<response::AccessTimingParameter>, Error> {
        self.context_util(channel, |ctx| {
            let service = Service::AccessTimingParam;
            let mut sub_func = access_type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), parameter, &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::suppress_positive_sr(ctx, false, request, suppress_positive)?;

            match response {
                Some(v) => {
                    Self::sub_func_check(&v, access_type.into(), service)?;
                    Ok(Some(v.data(&ctx.config).map_err(Error::ISO14229Error)?))
                },
                None => Ok(None)
            }
        })
    }

    pub fn secured_data_transmit(&mut self,
                                 channel: C,
                                 apar: AdministrativeParameter,
                                 signature: SignatureEncryptionCalculation,
                                 anti_replay_cnt: u16,
                                 service: u8,
                                 service_data: Vec<u8>,
                                 signature_data: Vec<u8>,
    ) -> Result<response::SecuredDataTrans, Error> {
        self.context_util(channel, |ctx| {
            let data = request::SecuredDataTrans::new(
                apar, signature, anti_replay_cnt, service, service_data, signature_data
            )
                .map_err(Error::ISO14229Error)?;
            let request = Request::new(Service::SecuredDataTrans, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            response.data::<response::SecuredDataTrans>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    pub fn control_dtc_setting(&mut self,
                               channel: C,
                               setting_type: DTCSettingType,
                               parameter: Vec<u8>,
                               suppress_positive: bool,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            let service = Service::CtrlDTCSetting;
            let mut sub_func = setting_type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), parameter, &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::suppress_positive_sr(ctx, false, request, suppress_positive)?;

            if let Some(response) = response {
                Self::sub_func_check(&response, setting_type.into(), service)?;
            }

            Ok(())
        })
    }

    pub fn response_on_event(&mut self,
                             channel: C,
    ) -> Result<(), Error> {
        self.context_util(channel, |_| {
            Err(Error::NotImplement(Service::ResponseOnEvent))
        })
    }

    pub fn link_control(&mut self,
                        channel: C,
                        ctrl_type: LinkCtrlType,
                        data: request::LinkCtrl,
                        suppress_positive: bool,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            let service = Service::LinkCtrl;
            let mut sub_func = ctrl_type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::suppress_positive_sr(ctx, false, request, suppress_positive)?;

            if let Some(response) = response {
                Self::sub_func_check(&response, ctrl_type.into(), service)?;
            }

            Ok(())
        })
    }

    /** - Data transmission functional unit - **/
    pub fn read_data_by_identifier(&mut self,
                                   channel: C,
                                   did: DataIdentifier,
                                   others: Vec<DataIdentifier>,
    ) -> Result<response::ReadDID, Error> {
        self.context_util(channel, |ctx| {
            let data = request::ReadDID::new(did, others);
            let request = Request::new(Service::ReadDID, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            response.data::<response::ReadDID>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    pub fn read_memory_by_address(&mut self,
                                  channel: C,
                                  mem_loc: MemoryLocation,
    ) -> Result<Vec<u8>, Error> {
        self.context_util(channel, |ctx| {
            let data = request::ReadMemByAddr(mem_loc);
            let request = Request::new(Service::ReadMemByAddr, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            Ok(response.raw_data().to_vec())
        })
    }

    pub fn read_scaling_data_by_identifier(&mut self,
                                           channel: C,
                                           did: DataIdentifier,
    ) -> Result<response::ReadScalingDID, Error> {
        self.context_util(channel, |ctx| {
            let data = request::ReadScalingDID(did);
            let request = Request::new(Service::ReadScalingDID, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            response.data::<response::ReadScalingDID>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    pub fn read_data_by_period_identifier(&mut self,
                                          channel: C,
                                          mode: request::TransmissionMode,
                                          did: Vec<u8>,
    ) -> Result<response::ReadDataByPeriodId, Error> {
        self.context_util(channel, |ctx| {
            let data = request::ReadDataByPeriodId::new(mode, did)
                .map_err(Error::ISO14229Error)?;
            let request = Request::new(Service::ReadDataByPeriodId, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            response.data::<response::ReadDataByPeriodId>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    pub fn dynamically_define_data_by_identifier(&mut self,
                                                 channel: C,
                                                 def_type: DefinitionType,
                                                 data: request::DynamicallyDefineDID,
                                                 suppress_positive: bool,
    ) -> Result<Option<response::DynamicallyDefineDID>, Error> {
        self.context_util(channel, |ctx| {
            let service = Service::DynamicalDefineDID;
            let mut sub_func = def_type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::suppress_positive_sr(ctx, false, request, suppress_positive)?;

            match response {
                Some(v) => {
                    Self::sub_func_check(&v, def_type.into(), service)?;
                    Ok(Some(v.data(&ctx.config)
                                .map_err(Error::ISO14229Error)?))
                },
                None => Ok(None)
            }
        })
    }

    pub fn write_data_by_identifier(&mut self,
                                    channel: C,
                                    did: DataIdentifier,
                                    data: Vec<u8>,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            let data = request::WriteDID(DIDData { did, data });
            let request = Request::new(Service::WriteDID, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let _ = Self::send_and_response(ctx, false, request)?;

            Ok(())
        })
    }

    pub fn write_memory_by_address(&mut self,
                                   channel: C,
                                   alfi: AddressAndLengthFormatIdentifier,
                                   mem_addr: u128,
                                   mem_size: u128,
                                   record: Vec<u8>,
    ) -> Result<response::WriteMemByAddr, Error> {
        self.context_util(channel, |ctx| {
            let data = request::WriteMemByAddr::new(alfi, mem_addr, mem_size, record)
                .map_err(Error::ISO14229Error)?;
            let request = Request::new(Service::WriteMemByAddr, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            response.data::<response::WriteMemByAddr>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    /** Stored data transmission functional unit - **/
    pub fn clear_dtc_info(&mut self,
                          channel: C,
                          group: utils::U24,
                          #[cfg(any(feature = "std2020"))]
                          mem_sel: Option<u8>,
                          functional: bool,
    ) -> Result<(), Error> {
        self.context_util(channel, |ctx| {
            #[cfg(any(feature = "std2020"))]
            let data = request::ClearDiagnosticInfo::new(group, mem_sel);
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            let data = request::ClearDiagnosticInfo::new(group);
            let request = Request::new(Service::ClearDiagnosticInfo, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let _ = Self::send_and_response(ctx, functional, request)?;

            Ok(())
        })
    }

    pub fn read_dtc_info(&mut self,
                         channel: C,
                         report_type: DTCReportType,
                         data: request::DTCInfo
    ) -> Result<response::DTCInfo, Error> {
        self.context_util(channel, |ctx| {
            let service = Service::ReadDTCInfo;
            let request = Request::new(service, Some(report_type.into()), data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;
            Self::sub_func_check(&response, report_type.into(), service)?;

            response.data::<response::DTCInfo>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    /** - InputOutput control functional unit - **/
    pub fn io_control(&mut self,
                      channel: C,
                      did: DataIdentifier,
                      param: IOCtrlParameter,
                      state: Vec<u8>,
                      mask: Vec<u8>,
    ) -> Result<response::IOCtrl, Error> {
        self.context_util(channel, |ctx| {
            let data = request::IOCtrl::new(did, param, state, mask, &ctx.config)
                .map_err(Error::ISO14229Error)?;
            let request = Request::new(Service::IOCtrl, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            response.data::<response::IOCtrl>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    /** - Remote activation of routine functional unit - **/
    pub fn routine_control(&mut self,
                           channel: C,
                           ctrl_type: RoutineCtrlType,
                           routine_id: u16,
                           option_record: Vec<u8>,
    ) -> Result<response::RoutineCtrl, Error> {
        self.context_util(channel, |ctx| {
            let service = Service::RoutineCtrl;
            let data = request::RoutineCtrl { routine_id: RoutineId(routine_id), option_record };
            let request = Request::new(service, Some(ctrl_type.into()), data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;
            Self::sub_func_check(&response, ctrl_type.into(), service)?;

            response.data::<response::RoutineCtrl>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    /** - Upload download functional unit - **/
    pub fn request_download(&mut self,
                            channel: C,
                            alfi: AddressAndLengthFormatIdentifier,
                            mem_addr: u128,
                            mem_size: u128,
                            dfi: Option<DataFormatIdentifier>,
    ) -> Result<response::RequestDownload, Error> {
        self.context_util(channel, |ctx| {
            let data = request::RequestDownload {
                dfi: dfi.unwrap_or_default(),
                mem_loc: MemoryLocation::new(alfi, mem_addr, mem_size)
                    .map_err(Error::ISO14229Error)?
            };
            let request = Request::new(Service::RequestDownload, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            response.data::<response::RequestDownload>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    pub fn request_upload(&mut self,
                          channel: C,
                          alfi: AddressAndLengthFormatIdentifier,
                          mem_addr: u128,
                          mem_size: u128,
                          dfi: Option<DataFormatIdentifier>,
    ) -> Result<response::RequestUpload, Error> {
        self.context_util(channel, |ctx| {
            let data = request::RequestUpload {
                dfi: dfi.unwrap_or_default(),
                mem_loc: MemoryLocation::new(alfi, mem_addr, mem_size)
                    .map_err(Error::ISO14229Error)?
            };
            let request = Request::new(Service::RequestDownload, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            response.data::<response::RequestUpload>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    pub fn transfer_data(&mut self,
                         channel: C,
                         sequence: u8,
                         data: Vec<u8>,
    ) -> Result<response::TransferData, Error> {
        self.context_util(channel, |ctx| {
            let data = response::TransferData { sequence, data };
            let request = Request::new(Service::TransferData, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            let data = response.data::<response::TransferData>(&ctx.config)
                .map_err(Error::ISO14229Error)?;

            if data.sequence != sequence {
                return Err(Error::UnexpectedTransferSequence { expect: sequence, actual: data.sequence })
            }

            Ok(data)
        })
    }

    pub fn request_transfer_exit(&mut self,
                                 channel: C,
                                 parameter: Vec<u8>,
    ) -> Result<Vec<u8>, Error> {
        self.context_util(channel, |ctx| {
            let request = Request::new(Service::RequestTransferExit, None, parameter, &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;

            Ok(response.raw_data().to_vec())
        })
    }

    // #[cfg(any(feature = "std2013", feature = "std2020"))]
    pub fn request_file_transfer(&mut self,
                                 channel: C,
                                 operation: ModeOfOperation,
                                 data: request::RequestFileTransfer,
    ) -> Result<response::RequestFileTransfer, Error> {
        self.context_util(channel, |ctx| {
            let service = Service::RequestFileTransfer;
            let sub_func = operation.into();
            let request = Request::new(service, Some(sub_func), data.to_vec(&ctx.config), &ctx.config)
                .map_err(Error::ISO14229Error)?;

            let response = Self::send_and_response(ctx, false, request)?;
            Self::sub_func_check(&response, operation.into(), service)?;

            response.data::<response::RequestFileTransfer>(&ctx.config)
                .map_err(Error::ISO14229Error)
        })
    }

    #[inline]
    fn context_util<R>(&mut self,
                       channel: C,
                       callback: impl FnOnce(&mut Context<C, F>) -> Result<R, Error>
    ) -> Result<R, Error> {
        match self.context.get_mut(&channel) {
            Some(ctx) => callback(ctx),
            None => Err(Error::OtherError(format!("channel: {} is not initialized", channel))),
        }
    }

    fn response_service_check(response: &Response, target: Service) -> Result<bool, Error> {
        let service = response.service();
        if response.is_negative() {
            let nrc_code = response.nrc_code()
                .map_err(Error::ISO14229Error)?;
            match nrc_code {
                Code::RequestCorrectlyReceivedResponsePending => Ok(true),
                _ => Err(Error::NRCError { service, code: nrc_code }),
            }
        } else if service != target {
            Err(Error::UnexpectedResponse { expect: target, actual: service })
        }
        else {
            Ok(false)
        }
    }

    fn suppress_positive_sr(ctx: &mut Context<C, F>,
                               functional: bool,
                               request: Request,
                               suppress_positive: bool,
    ) -> Result<Option<Response>, Error> {
        match Self::send_and_response(ctx, functional, request) {
            Ok(r) => Ok(Some(r)),
            Err(e) => match e {
                Error::IsoTpError(e) => match e {
                    IsoTpError::Timeout {..} => if suppress_positive {
                        Ok(None)
                    } else {
                        Err(Error::IsoTpError(e))
                    },
                    _ => Err(Error::IsoTpError(e)),
                }
                _ => Err(e),
            }
        }
    }

    fn send_and_response(ctx: &mut Context<C, F>,
                         functional: bool,
                         request: Request,
    ) -> Result<Response, Error>  {
        ctx.listener.clear_buffer();
        let service = request.service();
        ctx.iso_tp.write(functional, request.into())
            .map_err(Error::IsoTpError)?;

        let data = ctx.listener.sync_timer(false)
            .map_err(Error::IsoTpError)?;
        let mut response = Response::try_from_cfg(data, &ctx.config)
            .map_err(Error::ISO14229Error)?;
        while Self::response_service_check(&response, service)? {
            log::debug!("UDS - tester present when {:?}", Code::RequestCorrectlyReceivedResponsePending);
            let (_, request) =
                Self::tester_present_request(ctx, TesterPresentType::Zero, true)?;
            ctx.iso_tp.write(functional, request.into())
                .map_err(Error::IsoTpError)?;

            let data = ctx.listener.sync_timer(true)
                .map_err(Error::IsoTpError)?;

            response = Response::try_from_cfg(data, &ctx.config)
                .map_err(Error::ISO14229Error)?;
        }

        Ok(response)
    }

    fn sub_func_check(response: &Response, source: u8, service: Service) -> Result<(), Error> {
        match response.sub_function() {
            Some(v) => {
                // let source: u8 = session_type.into();
                let target = v.origin();
                if target != source {
                    Err(Error::UnexpectedSubFunction { service, expect: source, actual: target })
                }
                else {
                    Ok(())
                }
            },
            None => Err(Error::OtherError(format!("response of service `{}` got an empty sub-function", service))),
        }
    }

    #[inline]
    fn tester_present_request(
        ctx: &mut Context<C, F>,
        test_type: TesterPresentType,
        suppress_positive: bool,
    ) -> Result<(Service, Request), Error> {
        let service = Service::TesterPresent;
        let mut sub_func = test_type.into();
        if suppress_positive {
            sub_func |= SUPPRESS_POSITIVE;
        }
        let request = Request::new(service, Some(sub_func), vec![], &ctx.config)
            .map_err(Error::ISO14229Error)?;

        Ok((service, request))
    }
}
