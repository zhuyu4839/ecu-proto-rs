use std::{io::{Read, Write}, net::{TcpStream, UdpSocket, SocketAddr}};
use iso13400_2::{*, request};
use crate::DoIpError;
use super::{config::Configuration, context::Context};

type VerPayload = (Version, ResponsePayload);

#[derive(Debug)]
pub struct DoIpClient {
    config: Configuration,
    server_udp_addr: SocketAddr,
    udp_socket: UdpSocket,
    tcp_stream: TcpStream,
    context: Option<Context>,
}

impl DoIpClient {
    pub fn new(config: Configuration) -> Result<Self, DoIpError> {
        let udp_socket = UdpSocket::bind(format!("{}:0", config.server_ip()))
            .map_err(DoIpError::IoError)?;
        let tcp_stream = TcpStream::connect(format!("{}:{}", config.server_ip(), TCP_SERVER_PORT))
            .map_err(DoIpError::IoError)?;
        let server_udp_addr = format!("{}:{}", config.server_ip(), UDP_SERVER_PORT)
            .parse::<SocketAddr>()
            .unwrap();

        Ok(Self {
            config,
            server_udp_addr,
            udp_socket,
            tcp_stream,
            context: None,
        })
    }

    pub fn vehicle_identifier(&mut self) -> Result<bool, DoIpError> {
        let request = Message {
            version: self.version(),
            payload: Payload::Request(RequestPayload::VehicleId(request::VehicleID))
        };

        let response = self.udp_send_recv(request)?;
        self.vehicle_id_response(response)
    }

    pub fn vehicle_with_eid(&mut self, eid: Eid) -> Result<bool, DoIpError> {
        let request = Message {
            version: self.version(),
            payload: Payload::Request(
                RequestPayload::VehicleWithEid(request::VehicleIDWithEID::new(eid))
            )
        };

        let response = self.udp_send_recv(request)?;
        self.vehicle_id_response(response)
    }

    pub fn vehicle_with_vin(&mut self, vin: &str) -> Result<bool, DoIpError> {
        let payload = request::VehicleIDWithVIN::new(vin)
            .map_err(DoIpError::Iso13400Error)?;
        let request = Message {
            version: self.version(),
            payload: Payload::Request(RequestPayload::VehicleWithVIN(payload))
        };

        let response = self.udp_send_recv(request)?;
        self.vehicle_id_response(response)
    }

    pub fn routing_active(
        &mut self,
        r#type: RoutingActiveType,
        user_def: Option<u32>,
    ) -> Result<(bool, Option<u8>), DoIpError> {
        let src_addr = self.config.address();
        let request = Message {
            version: self.version(),
            payload: Payload::Request(RequestPayload::RoutingActive(
                request::RoutingActive::new(src_addr, r#type, user_def)
            ))
        };
        match self.tcp_write_read(request)? {
            Some((_, resp)) => match resp {
                ResponsePayload::HeaderNegative(v) => {
                    Err(DoIpError::HeaderNegativeError(v.code()))
                },
                ResponsePayload::RoutingActive(v) => {
                    let dst_addr = v.dst_addr();
                    if src_addr != dst_addr {
                        log::warn!("DoIPClient - routing active receive an message that target address: {:?}", dst_addr);
                    }

                    let active_code = v.active_code();
                    match active_code {
                        ActiveCode::Activated |
                        ActiveCode::Success => Ok((true, None)),
                        ActiveCode::NeedConfirm => Ok((true, None)),    // todo
                        ActiveCode::SourceAddressUnknown |
                        ActiveCode::SourceAddressInvalid |
                        ActiveCode::SocketInvalid |
                        ActiveCode::WithoutAuth |
                        ActiveCode::VehicleRefused |
                        ActiveCode::Unsupported |
                        ActiveCode::TLSRequired => Err(DoIpError::ActiveError(active_code)),
                        ActiveCode::VMSpecific(v) => {
                            log::info!("DoIPClient - routing active receive VM Specific value: {}", v);
                            Ok((true, Some(v)))
                        },
                        ActiveCode::Reserved(v) => {
                            log::warn!("DoIPClient - routing active receive Reserved value: {}", v);
                            Ok((true, Some(v)))
                        },
                    }
                },
                _ => Ok((false, None)),
            }
            None => Ok((false, None)),
        }
    }

    pub fn alive_check(&mut self) -> Result<bool, DoIpError> {
        let request = Message {
            version: self.version(),
            payload: Payload::Request(RequestPayload::AliveCheck(request::AliveCheck))
        };
        match self.tcp_write_read(request)? {
            Some((_, resp)) => match resp {
                ResponsePayload::HeaderNegative(v) => {
                    Err(DoIpError::HeaderNegativeError(v.code()))
                },
                ResponsePayload::AliveCheck(v) => {
                    log::info!("DoIPClient - alive check: {:?}", v.src_addr());
                    Ok(true)
                },
                _ => Ok(false),
            },
            None => Ok(true)
        }
    }

    pub fn entity_status(&mut self) -> Result<Option<response::EntityStatus>, DoIpError> {
        let request = Message {
            version: self.version(),
            payload: Payload::Request(RequestPayload::EntityStatue(request::EntityStatus))
        };
        match self.udp_send_recv(request)? {
            Some((_, resp)) => match resp {
                ResponsePayload::HeaderNegative(v) => {
                    Err(DoIpError::HeaderNegativeError(v.code()))
                },
                ResponsePayload::EntityStatue(v) => Ok(Some(v)),
                _ => Ok(None),
            },
            None => Ok(None)
        }
    }

    pub fn diag_power_mode(&mut self) -> Result<Option<PowerMode>, DoIpError> {
        let request = Message {
            version: self.version(),
            payload: Payload::Request(RequestPayload::DiagPowerMode(request::DiagnosticPowerMode))
        };
        match self.udp_send_recv(request)? {
            Some((_, resp)) => match resp {
                ResponsePayload::HeaderNegative(v) => {
                    Err(DoIpError::HeaderNegativeError(v.code()))
                },
                ResponsePayload::DiagPowerMode(v) => {
                    Ok(Some(v.mode()))
                },
                _ => Ok(None),
            },
            None => Ok(None),
        }
    }

    pub fn diagnostic(&mut self, data: Vec<u8>) -> Result<(), DoIpError> {
        match &self.context {
            Some(ctx) => {
                let request = Message {
                    version: self.version(),
                    payload: Payload::Request(RequestPayload::Diagnostic(
                        request::Diagnostic::new(self.config.address(), ctx.address(), data)
                    ))
                };
                match self.tcp_write_read(request)? {
                    Some((_, resp)) => match resp {
                        ResponsePayload::HeaderNegative(v) => {
                            Err(DoIpError::HeaderNegativeError(v.code()))
                        },
                        ResponsePayload::DiagNegative(v) => {
                            log::warn!("DoIPClient - {}", v);
                            Err(DoIpError::DiagnosticNegativeError {
                                code: v.code(),
                                data: hex::encode(v.previous_diagnostic_data())
                            })
                        },
                        ResponsePayload::DiagPositive(v) => {
                            log::debug!("DoIPClient - {}", v);
                            // TODO 特定时间内需要收到回复

                            Ok(())
                        },
                        _ => Ok(()),
                    },
                    None => Ok(()),
                }
            },
            None => Ok(())
        }
    }

    #[inline]
    fn vehicle_id_response(&mut self, response: Option<VerPayload>) -> Result<bool, DoIpError> {
        match response {
            Some((ver, resp)) => match resp {
                ResponsePayload::HeaderNegative(v) => {
                    Err(DoIpError::HeaderNegativeError(v.code()))
                },
                ResponsePayload::VehicleId(v) => {
                    self.context = Some(Context {
                        version: ver,
                        address: v.address(),
                        eid: v.eid(),
                        gid: v.gid(),
                        further_act: v.further_act(),
                        sync_status: v.sync_status(),
                    });

                    Ok(true)
                },
                _ => Ok(false),
            },
            None => Ok(false)
        }
    }

    fn udp_send_recv(&mut self, request: Message) -> Result<Option<VerPayload>, DoIpError> {
        let data: Vec<_> = request.into();
        log::trace!("DoIPClient - UDP writing data: {}", hex::encode(&data));
        let size = self.udp_socket.send_to(&data, &self.server_udp_addr)
            .map_err(DoIpError::IoError)?;
        let data_len = data.len();
        if size != data_len {
            log::warn!("DoIPClient - UDP wrote {} bytes, expect {}", size, data_len);
        }

        let mut buffer = [0; 1024];
        let size = self.udp_socket.recv(&mut buffer)
            .map_err(DoIpError::IoError)?;

        self.parse_response(&buffer[..size])
    }

    fn tcp_write_read(&mut self, request: Message) -> Result<Option<VerPayload>, DoIpError> {
        let data: Vec<_> = request.into();
        println!("DoIPClient - TCP writing data: {}", hex::encode(&data));
        let size = self.tcp_stream.write(&data)
            .map_err(DoIpError::IoError)?;
        let data_len = data.len();
        if size != data_len {
            log::warn!("DoIPClient - TCP wrote {} bytes, expect {}", size, data_len);
        }

        let mut buffer = [0; 1024];
        let size = self.tcp_stream.read(&mut buffer)
            .map_err(DoIpError::IoError)?;

        self.parse_response(&buffer[..size])
    }

    #[inline]
    fn parse_response(&mut self, data: &[u8]) -> Result<Option<VerPayload>, DoIpError> {
        let response = Message::try_from(data)
            .map_err(DoIpError::Iso13400Error)?;
        let version = response.version;
        self.version_check(version);

        match response.payload {
            Payload::Request(_) => {
                log::warn!("DoIPClient - unknown payload type from server!");
                Ok(None)
            },
            Payload::Response(response) => Ok(Some((version, response)))
        }
    }

    fn version_check(&self, version: Version) {
        match &self.context {
            Some(ctx) => if ctx.version() != version {
                log::warn!("DoIPClient - DoIP version mismatch!");
            },
            None => {},
        }
    }

    #[inline]
    fn version(&self) -> Version {
        match &self.context {
            Some(ctx) => ctx.version,
            None => Version::Default,
        }
    }
}

#[cfg(test)]
mod tests {
    use iso13400_2::{Eid, LogicAddress, PowerMode, RoutingActiveType};
    use iso14229_1::{request, Service, SessionType, Configuration as Iso14229Cfg};
    use crate::client::{Configuration, DoIpClient};
    use crate::DoIpError;

    fn init_client() -> Result<DoIpClient, DoIpError> {
        let cfg = Configuration::new("127.0.0.1", LogicAddress::from(0x0e00))
            .unwrap();
        DoIpClient::new(cfg)
    }

    #[test]
    fn vehicle_identifier() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let ret = client.vehicle_identifier()?;
        assert!(ret);

        Ok(())
    }

    #[test]
    fn vehicle_with_eid() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let ret = client.vehicle_with_eid(Eid::new(0xE1E2E3E4E5E6)?)?;
        assert!(ret);

        Ok(())
    }

    #[test]
    fn vehicle_with_vin() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let ret = client.vehicle_with_vin("12345678901234567")?;
        assert!(ret);

        // block
        // let ret = client.vehicle_with_vin("12345678901234560")?;
        // assert!(ret);

        Ok(())
    }

    #[test]
    fn entity_status() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let ret = client.entity_status()?;
        assert!(ret.is_some());

        Ok(())
    }

    #[test]
    fn diag_power_mode() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let power_mode = client.diag_power_mode()?.unwrap();
        assert_eq!(power_mode, PowerMode::NotReady);

        Ok(())
    }

    #[test]
    fn alive_check() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let ret = client.alive_check()?;
        assert!(ret);

        Ok(())
    }

    #[test]
    fn routing_active() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let (ret, value) = client.routing_active(RoutingActiveType::CentralSecurity, None)?;
        assert!(ret);

        Ok(())
    }

    #[test]
    fn diagnostics() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let cfg = Iso14229Cfg::default();
        let ret = client.vehicle_identifier()?;
        assert!(ret);
        let (ret, value) = client.routing_active(Default::default(), None)?;
        assert!(ret);
        assert!(value.is_none());
        let diag_req = request::Request::new(
            Service::SessionCtrl,
            Some(SessionType::Default.into()),
            vec![],
            &cfg
        )?;
        let ret = client.diagnostic(diag_req.into())?;
        // assert!(ret);

        Ok(())
    }
}
