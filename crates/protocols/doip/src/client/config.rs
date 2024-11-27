use std::net::SocketAddr;
use getset::{CopyGetters, Getters};
use iso13400_2::{LogicAddress, TCP_SERVER_PORT};

#[derive(Clone, Debug, Getters, CopyGetters)]
#[get = "pub"]
pub struct Configuration {
    server_ip: String,
    #[get_copy = "pub"]
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
