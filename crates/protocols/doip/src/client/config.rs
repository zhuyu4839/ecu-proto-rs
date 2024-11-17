use std::net::SocketAddr;
use derive_getters::Getters;
use iso13400_2::{LogicAddress, TCP_SERVER_PORT};

#[derive(Clone, Debug, Getters)]
pub struct Configuration {
    server_ip: String,
    #[getter(copy)]
    address: LogicAddress,
}

impl Configuration {
    pub fn new(
        server_ip: &str,
        address: LogicAddress,
    ) -> Option<Self> {
        match format!("{}:{}", server_ip, TCP_SERVER_PORT).parse::<SocketAddr>() {
            Ok(_) => Some(Self {
                server_ip: server_ip.to_owned(),
                address
            }),
            Err(_) => None,
        }
    }
}
