//! Table 17 — Overview of DoIP payload types at line #49(ISO 13400-2-2019)
pub const TCP_SERVER_PORT: u16 = 13400;
pub const TLS_TCP_SERVER_PORT: u16 = 3496;
pub const UDP_SERVER_PORT: u16 = 13400;

pub(crate) const HEADER_NEGATIVE: u16 = 0x0000;
pub(crate) const UDP_REQ_VEHICLE_IDENTIFIER: u16 = 0x0001;
pub(crate) const UDP_REQ_VEHICLE_ID_WITH_EID: u16 = 0x0002;
pub(crate) const UDP_REQ_VEHICLE_ID_WITH_VIN: u16 = 0x0003;
pub(crate) const UDP_RESP_VEHICLE_IDENTIFIER: u16 = 0x0004;

pub(crate) const TCP_REQ_ROUTING_ACTIVE: u16 = 0x0005;
pub(crate) const TCP_RESP_ROUTING_ACTIVE: u16 = 0x0006;

pub(crate) const TCP_REQ_ALIVE_CHECK: u16 = 0x0007;
pub(crate) const TCP_RESP_ALIVE_CHECK: u16 = 0x0008;

pub(crate) const UDP_REQ_ENTITY_STATUS: u16 = 0x4001;
pub(crate) const UDP_RESP_ENTITY_STATUS: u16 = 0x4002;

pub(crate) const UDP_REQ_DIAGNOSTIC_POWER_MODE: u16 = 0x4003;
pub(crate) const UDP_RESP_DIAGNOSTIC_POWER_MODE: u16 = 0x4004;

pub(crate) const TCP_DIAGNOSTIC: u16 = 0x8001;
pub(crate) const TCP_RESP_DIAGNOSTIC_POSITIVE: u16 = 0x8002;
pub(crate) const TCP_RESP_DIAGNOSTIC_NEGATIVE: u16 = 0x8003;

/// length of EID and GID
pub(crate) const SIZE_OF_ID: usize = 6;
pub const LENGTH_OF_VIN: usize = 17;
pub(crate) const SIZE_OF_ADDRESS: usize = 2;
pub(crate) const SIZE_OF_VERSION: usize = 2;
pub(crate) const SIZE_OF_DATA_TYPE: usize = 2;
pub(crate) const SIZE_OF_LENGTH: usize = 4;
