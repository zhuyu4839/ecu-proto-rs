use std::net::SocketAddr;
use derive_getters::Getters;
use iso13400_2::{Eid, Gid, LogicAddress, LENGTH_OF_VIN, TCP_SERVER_PORT};

#[derive(Clone, Debug, Getters)]
pub struct Configuration {
    ip_address: String,
    vin: String,
    #[getter(copy)]
    address: LogicAddress,
    #[getter(copy)]
    eid: Eid,
    #[getter(copy)]
    gid: Gid,
}

impl Configuration {
    pub fn new(
        ip: &str,
        vin: &str,
        address: LogicAddress,
        eid: Eid,
        gid: Gid,
    ) -> Option<Self> {
        match vin.len() {
            LENGTH_OF_VIN => match format!("{}:{}", ip, TCP_SERVER_PORT).parse::<SocketAddr>() {
                Ok(_) => Some(Self {
                    ip_address: ip.to_owned(),
                    vin: vin.to_owned(),
                    address,
                    eid,
                    gid,
                }),
                Err(_) => None,
            },
            _ => None,
        }
    }
}

