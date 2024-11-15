use std::{collections::HashMap, fmt::Display, hash::Hash, time::Duration};
use iso14229_1::{response::{self, Response, Code}, request::{self, Request}, *};
use iso14229_1::utils::U24;
use iso15765_2::{Iso15765Error, IsoTpEventListener};
use rs_can::{CanDriver, isotp::{Address, AddressType, CanIsoTp, IsoTpAdapter}, Frame, ResultWrapper};
use crate::{client::context::{Context, IsoTpListener}, Client, DoCanError, SecurityAlgo};

#[derive(Clone)]
pub struct DoCanClient<D, C, F>
where
    C: Clone + Eq,
{
    adapter: IsoTpAdapter<D, C, F>,
    context: HashMap<C, Context<C, F>>,
    p2_offset: u64,
}

impl<D, C, F> DoCanClient<D, C, F>
where
    D: CanDriver<Channel = C, Frame = F> + Clone + Send + 'static,
    C: Display + Clone + Hash + Eq + 'static,
    F: Frame<Channel = C> + Clone + Send + Display + 'static
{
    pub fn new(adapter: IsoTpAdapter<D, C, F>, p2_offset: Option<u16>) -> Self {
        Self {
            adapter,
            context: Default::default(),
            p2_offset: p2_offset.unwrap_or_default() as u64,
        }
    }

    pub fn init_channel(&mut self, channel: C, address: Address,) -> Result<(), DoCanError> {
        let listener = IsoTpListener::new(Default::default(), self.p2_offset);
        let iso_tp = CanIsoTp::new(
            channel.clone(),
            address,
            self.adapter.sender(),
            Box::new(listener.clone())
        );

        self.adapter.register_listener(
            format!("DoCANClient-{}", channel),
            Box::new(iso_tp.clone())
        );
        self.context.insert(channel, Context {
            iso_tp,
            listener,
            config: Default::default(),
            security_algo: Default::default(),
        });

        Ok(())
    }
    #[inline]
    pub fn adapter(&self) -> &IsoTpAdapter<D, C, F> {
        &self.adapter
    }

    #[inline]
    fn context_util<R>(&mut self,
                       channel: C,
                       callback: impl FnOnce(&mut Context<C, F>) -> Result<R, DoCanError>
    ) -> Result<R, DoCanError> {
        match self.context.get_mut(&channel) {
            Some(ctx) => callback(ctx),
            None => Err(DoCanError::OtherError(format!("channel: {} is not initialized", channel))),
        }
    }

    fn response_service_check(response: &Response, target: Service) -> Result<bool, DoCanError> {
        let service = response.service();
        if response.is_negative() {
            let nrc_code = response.nrc_code()
                .map_err(DoCanError::ISO14229Error)?;
            match nrc_code {
                Code::RequestCorrectlyReceivedResponsePending => Ok(true),
                _ => Err(DoCanError::NRCError { service, code: nrc_code }),
            }
        } else if service != target {
            Err(DoCanError::UnexpectedResponse { expect: target, actual: service })
        }
        else {
            Ok(false)
        }
    }

    fn suppress_positive_sr(ctx: &mut Context<C, F>,
                            addr_type: AddressType,
                            request: Request,
                            suppress_positive: bool,
    ) -> Result<Option<Response>, DoCanError> {
        match Self::send_and_response(ctx, addr_type, request) {
            Ok(r) => Ok(Some(r)),
            Err(e) => match e {
                DoCanError::IsoTpError(e) => match e {
                    Iso15765Error::Timeout {..} => if suppress_positive {
                        Ok(None)
                    } else {
                        Err(DoCanError::IsoTpError(e))
                    },
                    _ => Err(DoCanError::IsoTpError(e)),
                }
                _ => Err(e),
            }
        }
    }

    fn send_and_response(ctx: &mut Context<C, F>,
                         addr_type: AddressType,
                         request: Request,
    ) -> Result<Response, DoCanError>  {
        ctx.listener.clear_buffer();
        let service = request.service();
        ctx.iso_tp.write(addr_type, request.into())
            .map_err(DoCanError::IsoTpError)?;

        let data = ctx.listener.sync_timer(false)
            .map_err(DoCanError::IsoTpError)?;
        let mut response = Response::try_from_cfg(data, &ctx.config)
            .map_err(DoCanError::ISO14229Error)?;
        while Self::response_service_check(&response, service)? {
            log::debug!("DoCANClient - tester present when {:?}", Code::RequestCorrectlyReceivedResponsePending);
            let (_, request) =
                Self::tester_present_request(ctx, TesterPresentType::Zero, true)?;
            ctx.iso_tp.write(addr_type, request.into())
                .map_err(DoCanError::IsoTpError)?;

            let data = ctx.listener.sync_timer(true)
                .map_err(DoCanError::IsoTpError)?;

            response = Response::try_from_cfg(data, &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;
        }

        Ok(response)
    }

    fn sub_func_check(response: &Response, source: u8, service: Service) -> Result<(), DoCanError> {
        match response.sub_function() {
            Some(v) => {
                // let source: u8 = session_type.into();
                let target = v.origin();
                if target != source {
                    Err(DoCanError::UnexpectedSubFunction { service, expect: source, actual: target })
                }
                else {
                    Ok(())
                }
            },
            None => Err(DoCanError::OtherError(format!("response of service `{}` got an empty sub-function", service))),
        }
    }

    #[inline]
    fn tester_present_request(
        ctx: &Context<C, F>,
        test_type: TesterPresentType,
        suppress_positive: bool,
    ) -> Result<(Service, Request), DoCanError> {
        let service = Service::TesterPresent;
        let mut sub_func = test_type.into();
        if suppress_positive {
            sub_func |= SUPPRESS_POSITIVE;
        }
        let request = Request::new(service, Some(sub_func), vec![], &ctx.config)
            .map_err(DoCanError::ISO14229Error)?;

        Ok((service, request))
    }
}

impl<D, C, F> Client for DoCanClient<D, C, F>
where
    D: CanDriver<Channel = C, Frame = F> + Clone + Send + 'static,
    C: Display + Clone + Hash + Eq + 'static,
    F: Frame<Channel = C> + Clone + Send + Display + 'static
{
    type Channel = C;
    type Error = DoCanError;

    fn update_address(&mut self, channel: Self::Channel, address: Address) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            ctx.iso_tp.update_address(address);

            Ok(())
        })
    }

    fn update_security_algo(&mut self, channel: Self::Channel, algo: SecurityAlgo) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            ctx.security_algo = Some(algo);

            Ok(())
        })
    }

    fn add_data_identifier(&mut self, channel: Self::Channel, did: DataIdentifier, length: usize) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            ctx.config.did_cfg.insert(did, length);

            Ok(())
        })
    }

    fn remove_data_identifier(&mut self, channel: Self::Channel, did: DataIdentifier) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            ctx.config.did_cfg.remove(&did);

            Ok(())
        })
    }

    fn set_address_of_byte_order(&mut self, channel: Self::Channel, bo: ByteOrder) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            ctx.config.bo_addr = bo;

            Ok(())
        })
    }

    fn set_memory_size_of_byte_order(&mut self, channel: Self::Channel, bo: ByteOrder) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            ctx.config.bo_mem_size = bo;

            Ok(())
        })
    }

    fn session_ctrl(&mut self, channel: Self::Channel, r#type: SessionType, suppress_positive: bool, addr_type: AddressType) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::SessionCtrl;
            let mut sub_func: u8 = r#type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), vec![], &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            if let Some(response) = Self::suppress_positive_sr(ctx, addr_type, request, suppress_positive)? {
                Self::sub_func_check(&response, r#type.into(), service)?;

                let timing = response.data::<response::SessionCtrl>(&ctx.config)
                    .map_err(DoCanError::ISO14229Error)?
                    .0;
                ctx.listener.update_p2_ctx(timing.p2, timing.p2_star);
            }

            Ok(())
        })
    }

    fn ecu_reset(&mut self, channel: Self::Channel, r#type: ECUResetType, suppress_positive: bool, addr_type: AddressType) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::ECUReset;
            let mut sub_func: u8 = r#type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), vec![], &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            if let Some(response) = Self::suppress_positive_sr(ctx, addr_type, request, suppress_positive)? {
                Self::sub_func_check(&response, r#type.into(), service)?;

                let resp = response.data::<response::ECUReset>(&ctx.config)
                    .map_err(DoCanError::ISO14229Error)?;
                if let Some(seconds) = resp.second {
                    std::thread::sleep(Duration::from_secs(seconds as u64));
                }
            }

            Ok(())
        })
    }

    fn security_access(&mut self, channel: Self::Channel, level: u8, params: Vec<u8>) -> ResultWrapper<Vec<u8>, Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::SecurityAccess;
            let request = Request::new(service, Some(level), params, &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            Self::sub_func_check(&response, level, service)?;

            Ok(response.raw_data().to_vec())
        })
    }

    fn unlock_security_access(&mut self, channel: Self::Channel, level: u8, params: Vec<u8>, salt: Vec<u8>) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            if let Some(algo) = ctx.security_algo {
                let service = Service::SecurityAccess;
                let req = Request::new(service, Some(level), params, &ctx.config)
                    .map_err(DoCanError::ISO14229Error)?;

                let resp = Self::send_and_response(ctx, AddressType::Physical, req)?;
                Self::sub_func_check(&resp, level, service)?;

                let seed = resp.raw_data().to_vec();
                match algo(level, seed, salt)? {
                    Some(data) => {
                        let request = Request::new(service, Some(level + 1), data, &ctx.config)
                            .map_err(DoCanError::ISO14229Error)?;
                        let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

                        Self::sub_func_check(&response, level + 1, service)
                    },
                    None => Ok(())
                }
            }
            else {
                Err(DoCanError::OtherError("security algorithm required".into()))
            }
        })
    }

    fn communication_control(&mut self, channel: Self::Channel, ctrl_type: CommunicationCtrlType, comm_type: CommunicationType, node_id: Option<request::NodeId>, suppress_positive: bool, addr_type: AddressType) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::CommunicationCtrl;
            let mut sub_func = ctrl_type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let data = request::CommunicationCtrl::new(ctrl_type, comm_type, node_id)
                .map_err(DoCanError::ISO14229Error)?;
            let req = Request::new(service, Some(sub_func), data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let resp = Self::suppress_positive_sr(ctx, addr_type, req, suppress_positive)?;

            if let Some(response) = resp {
                Self::sub_func_check(&response, ctrl_type.into(), service)?;
            }

            Ok(())
        })
    }

    #[cfg(feature = "std2020")]
    fn authentication(&mut self, channel: Self::Channel, auth_task: AuthenticationTask, data: request::Authentication) -> ResultWrapper<response::Authentication, Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::Authentication;
            let request = Request::new(service, Some(auth_task.into()), data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;
            Self::sub_func_check(&response, auth_task.into(), service)?;

            response.data::<response::Authentication>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn tester_present(&mut self, channel: Self::Channel, r#type: TesterPresentType, suppress_positive: bool, addr_type: AddressType) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            let (service, request) =
                Self::tester_present_request(ctx, r#type, suppress_positive)?;

            let response = Self::suppress_positive_sr(ctx, addr_type, request, suppress_positive)?;

            if let Some(response) = response {
                Self::sub_func_check(&response, r#type.into(), service)?;
            }

            Ok(())
        })
    }

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    fn access_timing_parameter(&mut self, channel: Self::Channel, r#type: TimingParameterAccessType, parameter: Vec<u8>, suppress_positive: bool) -> ResultWrapper<Option<response::AccessTimingParameter>, Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::AccessTimingParam;
            let mut sub_func = r#type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), parameter, &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::suppress_positive_sr(ctx, AddressType::Physical, request, suppress_positive)?;

            match response {
                Some(v) => {
                    Self::sub_func_check(&v, r#type.into(), service)?;
                    Ok(Some(v.data(&ctx.config).map_err(DoCanError::ISO14229Error)?))
                },
                None => Ok(None)
            }
        })
    }

    fn secured_data_transmit(&mut self, channel: Self::Channel, apar: AdministrativeParameter, signature: SignatureEncryptionCalculation, anti_replay_cnt: u16, service: u8, service_data: Vec<u8>, signature_data: Vec<u8>) -> ResultWrapper<response::SecuredDataTrans, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::SecuredDataTrans::new(
                apar, signature, anti_replay_cnt, service, service_data, signature_data
            )
                .map_err(DoCanError::ISO14229Error)?;
            let request = Request::new(Service::SecuredDataTrans, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            response.data::<response::SecuredDataTrans>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn control_dtc_setting(&mut self, channel: Self::Channel, r#type: DTCSettingType, parameter: Vec<u8>, suppress_positive: bool) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::CtrlDTCSetting;
            let mut sub_func = r#type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), parameter, &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::suppress_positive_sr(ctx, AddressType::Physical, request, suppress_positive)?;

            if let Some(response) = response {
                Self::sub_func_check(&response, r#type.into(), service)?;
            }

            Ok(())
        })
    }

    fn response_on_event(&mut self, channel: Self::Channel) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |_| {
            Err(DoCanError::NotImplement(Service::ResponseOnEvent))
        })
    }

    fn link_control(&mut self, channel: Self::Channel, r#type: LinkCtrlType, data: request::LinkCtrl, suppress_positive: bool) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::LinkCtrl;
            let mut sub_func = r#type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::suppress_positive_sr(ctx, AddressType::Physical, request, suppress_positive)?;

            if let Some(response) = response {
                Self::sub_func_check(&response, r#type.into(), service)?;
            }

            Ok(())
        })
    }

    fn read_data_by_identifier(&mut self, channel: Self::Channel, did: DataIdentifier, others: Vec<DataIdentifier>) -> ResultWrapper<response::ReadDID, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::ReadDID::new(did, others);
            let request = Request::new(Service::ReadDID, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            response.data::<response::ReadDID>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn read_memory_by_address(&mut self, channel: Self::Channel, mem_loc: MemoryLocation) -> ResultWrapper<Vec<u8>, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::ReadMemByAddr(mem_loc);
            let request = Request::new(Service::ReadMemByAddr, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            Ok(response.raw_data().to_vec())
        })
    }

    fn read_scaling_data_by_identifier(&mut self, channel: Self::Channel, did: DataIdentifier) -> ResultWrapper<response::ReadScalingDID, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::ReadScalingDID(did);
            let request = Request::new(Service::ReadScalingDID, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            response.data::<response::ReadScalingDID>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn read_data_by_period_identifier(&mut self, channel: Self::Channel, mode: request::TransmissionMode, did: Vec<u8>) -> ResultWrapper<response::ReadDataByPeriodId, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::ReadDataByPeriodId::new(mode, did)
                .map_err(DoCanError::ISO14229Error)?;
            let request = Request::new(Service::ReadDataByPeriodId, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            response.data::<response::ReadDataByPeriodId>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn dynamically_define_data_by_identifier(&mut self, channel: Self::Channel, r#type: DefinitionType, data: request::DynamicallyDefineDID, suppress_positive: bool) -> ResultWrapper<Option<response::DynamicallyDefineDID>, Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::DynamicalDefineDID;
            let mut sub_func = r#type.into();
            if suppress_positive {
                sub_func |= SUPPRESS_POSITIVE;
            }
            let request = Request::new(service, Some(sub_func), data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::suppress_positive_sr(ctx, AddressType::Physical, request, suppress_positive)?;

            match response {
                Some(v) => {
                    Self::sub_func_check(&v, r#type.into(), service)?;
                    Ok(Some(v.data(&ctx.config)
                        .map_err(DoCanError::ISO14229Error)?))
                },
                None => Ok(None)
            }
        })
    }

    fn write_data_by_identifier(&mut self, channel: Self::Channel, did: DataIdentifier, data: Vec<u8>) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::WriteDID(DIDData { did, data });
            let request = Request::new(Service::WriteDID, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let _ = Self::send_and_response(ctx, AddressType::Physical, request)?;

            Ok(())
        })
    }

    fn write_memory_by_address(&mut self, channel: Self::Channel, alfi: AddressAndLengthFormatIdentifier, mem_addr: u128, mem_size: u128, record: Vec<u8>) -> ResultWrapper<response::WriteMemByAddr, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::WriteMemByAddr::new(alfi, mem_addr, mem_size, record)
                .map_err(DoCanError::ISO14229Error)?;
            let request = Request::new(Service::WriteMemByAddr, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            response.data::<response::WriteMemByAddr>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn clear_dtc_info(&mut self, channel: Self::Channel, group: U24, mem_sel: Option<u8>, addr_type: AddressType) -> ResultWrapper<(), Self::Error> {
        self.context_util(channel, |ctx| {
            #[cfg(any(feature = "std2020"))]
            let data = request::ClearDiagnosticInfo::new(group, mem_sel);
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            let data = request::ClearDiagnosticInfo::new(group);
            let request = Request::new(Service::ClearDiagnosticInfo, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let _ = Self::send_and_response(ctx, addr_type, request)?;

            Ok(())
        })
    }

    fn read_dtc_info(&mut self, channel: Self::Channel, r#type: DTCReportType, data: request::DTCInfo) -> ResultWrapper<response::DTCInfo, Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::ReadDTCInfo;
            let request = Request::new(service, Some(r#type.into()), data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;
            Self::sub_func_check(&response, r#type.into(), service)?;

            response.data::<response::DTCInfo>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn io_control(&mut self, channel: Self::Channel, did: DataIdentifier, param: IOCtrlParameter, state: Vec<u8>, mask: Vec<u8>) -> ResultWrapper<response::IOCtrl, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::IOCtrl::new(did, param, state, mask, &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;
            let request = Request::new(Service::IOCtrl, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            response.data::<response::IOCtrl>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn routine_control(&mut self, channel: Self::Channel, r#type: RoutineCtrlType, routine_id: u16, option_record: Vec<u8>) -> ResultWrapper<response::RoutineCtrl, Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::RoutineCtrl;
            let data = request::RoutineCtrl { routine_id: RoutineId(routine_id), option_record };
            let request = Request::new(service, Some(r#type.into()), data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;
            Self::sub_func_check(&response, r#type.into(), service)?;

            response.data::<response::RoutineCtrl>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn request_download(&mut self, channel: Self::Channel, alfi: AddressAndLengthFormatIdentifier, mem_addr: u128, mem_size: u128, dfi: Option<DataFormatIdentifier>) -> ResultWrapper<response::RequestDownload, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::RequestDownload {
                dfi: dfi.unwrap_or_default(),
                mem_loc: MemoryLocation::new(alfi, mem_addr, mem_size)
                    .map_err(DoCanError::ISO14229Error)?
            };
            let request = Request::new(Service::RequestDownload, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            response.data::<response::RequestDownload>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn request_upload(&mut self, channel: Self::Channel, alfi: AddressAndLengthFormatIdentifier, mem_addr: u128, mem_size: u128, dfi: Option<DataFormatIdentifier>) -> ResultWrapper<response::RequestUpload, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = request::RequestUpload {
                dfi: dfi.unwrap_or_default(),
                mem_loc: MemoryLocation::new(alfi, mem_addr, mem_size)
                    .map_err(DoCanError::ISO14229Error)?
            };
            let request = Request::new(Service::RequestDownload, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            response.data::<response::RequestUpload>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }

    fn transfer_data(&mut self, channel: Self::Channel, sequence: u8, data: Vec<u8>) -> ResultWrapper<response::TransferData, Self::Error> {
        self.context_util(channel, |ctx| {
            let data = response::TransferData { sequence, data };
            let request = Request::new(Service::TransferData, None, data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            let data = response.data::<response::TransferData>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            if data.sequence != sequence {
                return Err(DoCanError::UnexpectedTransferSequence { expect: sequence, actual: data.sequence })
            }

            Ok(data)
        })
    }

    fn request_transfer_exit(&mut self, channel: Self::Channel, parameter: Vec<u8>) -> ResultWrapper<Vec<u8>, Self::Error> {
        self.context_util(channel, |ctx| {
            let request = Request::new(Service::RequestTransferExit, None, parameter, &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;

            Ok(response.raw_data().to_vec())
        })
    }

    fn request_file_transfer(&mut self, channel: Self::Channel, operation: ModeOfOperation, data: request::RequestFileTransfer) -> ResultWrapper<response::RequestFileTransfer, Self::Error> {
        self.context_util(channel, |ctx| {
            let service = Service::RequestFileTransfer;
            let sub_func = operation.into();
            let request = Request::new(service, Some(sub_func), data.to_vec(&ctx.config), &ctx.config)
                .map_err(DoCanError::ISO14229Error)?;

            let response = Self::send_and_response(ctx, AddressType::Physical, request)?;
            Self::sub_func_check(&response, operation.into(), service)?;

            response.data::<response::RequestFileTransfer>(&ctx.config)
                .map_err(DoCanError::ISO14229Error)
        })
    }
}
