mod config;

use std::{collections::HashMap, net::{SocketAddr, TcpListener, TcpStream, UdpSocket}, sync::{Arc, Mutex}};
use iso13400_2::{response, *};
use crate::DoIpError;

#[derive(Debug)]
pub struct DoIpServer {
    version: Version,
    address: LogicAddress,
    udp_socket: UdpSocket,
    tcp_listener: TcpListener,
    tcp_streams: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>,
}

impl DoIpServer {
    pub fn new(addr: &str, version: Version, address: LogicAddress) -> Result<Self, DoIpError> {
        let udp_socket = UdpSocket::bind(format!("{addr}:{}", UDP_SERVER_PORT))
            .map_err(DoIpError::IoError)?;
        let tcp_listener = TcpListener::bind(format!("{addr}:{}", UDP_SERVER_PORT))
            .map_err(DoIpError::IoError)?;

        Ok(Self {
            version,
            address,
            udp_socket,
            tcp_listener,
            tcp_streams: Default::default(),
        })
    }

    fn udp_service(&self) -> Result<(), DoIpError> {
        loop {
            let mut buffer = [0; 1024];
            if let Ok(size) = self.udp_socket.recv(&mut buffer) {
                if size > 0 {
                    match Message::try_from(&buffer[..size]) {
                        Ok(message) => {
                            let version = self.version;
                            if version != self.version
                                && version != Version::Default {
                                let response = Message {
                                    version: self.version.clone(),
                                    payload: Payload::Response(ResponsePayload::HeaderNegative(
                                        response::HeaderNegative::new(HeaderNegativeCode::IncorrectPatternFormat)
                                    ))
                                };

                                self.udp_send(response)?;
                                continue;
                            }

                            match message.payload {
                                Payload::Request(req) => match req {
                                    RequestPayload::VehicleId(_) => {
                                        let response = Message {
                                            version: self.version.clone(),
                                            payload: Payload::Response(ResponsePayload::HeaderNegative(
                                                response::HeaderNegative::new(HeaderNegativeCode::IncorrectPatternFormat)
                                            ))
                                        };
                                    }
                                    RequestPayload::VehicleWithEid(v) => {}
                                    RequestPayload::VehicleWithVIN(v) => {}
                                    RequestPayload::EntityStatue(v) => {}
                                    RequestPayload::DiagPowerMode(v) => {}
                                    // RequestPayload::RoutingActive(_) => {}
                                    // RequestPayload::AliveCheck(_) => {}
                                    // RequestPayload::Diagnostic(_) => {}
                                    _ => {
                                        let response = Message {
                                            version: self.version.clone(),
                                            payload: Payload::Response(ResponsePayload::HeaderNegative(
                                                response::HeaderNegative::new(HeaderNegativeCode::UnknownPayloadTYpe)
                                            ))
                                        };
                                        self.udp_send(response)?;
                                    }
                                },
                                Payload::Response(_) => {
                                    let response = Message {
                                        version: self.version.clone(),
                                        payload: Payload::Response(ResponsePayload::HeaderNegative(
                                            response::HeaderNegative::new(HeaderNegativeCode::UnknownPayloadTYpe)
                                        ))
                                    };
                                    self.udp_send(response)?;
                                }
                            }
                        },
                        Err(error) => {
                            if let Some(code) = match error{
                                Iso13400Error::InvalidLength { .. } => Some(HeaderNegativeCode::IncorrectPatternFormat),
                                Iso13400Error::InvalidDataLen { .. } => Some(HeaderNegativeCode::InvalidPayloadLength),
                                Iso13400Error::InvalidVersion { .. } => Some(HeaderNegativeCode::IncorrectPatternFormat),
                                Iso13400Error::InvalidPayloadType(..) => Some(HeaderNegativeCode::UnknownPayloadTYpe),
                                _ => None
                            } {
                                match code {
                                    HeaderNegativeCode::IncorrectPatternFormat |
                                    HeaderNegativeCode::InvalidPayloadLength => {
                                        // self
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn tcp_service(&mut self) -> Result<(), DoIpError> {
        loop {
            if let Ok((tcp_stream, addr)) = self.tcp_listener.accept() {
                log::info!("Connection established with {}", addr);
                match self.tcp_streams.lock() {
                    Ok(mut streams) => {
                        streams.remove(&addr);
                        streams.insert(addr, tcp_stream);
                    },
                    Err(_) => {

                    },
                }
            }
        }
    }

    fn udp_send(&self, message: Message) -> Result<(), DoIpError> {
        let data: Vec<_> = message.into();
        let size = self.udp_socket.send(&data)
            .map_err(DoIpError::IoError)?;
        let data_len = data.len();
        if size < data_len {
            log::warn!("DoIPServer - udp send size: {}, expect: {}", size, data_len);
        }

        Ok(())
    }
}

