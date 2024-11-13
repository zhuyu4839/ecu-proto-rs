// mod synchronous;
// pub use synchronous::*;

use std::{collections::HashMap, fmt::Display, hash::Hash};
use iso14229_1::{DataIdentifier, SessionType};
use rs_can::{isotp::{Address, P2Context}, ResultWrapper};
use crate::SecurityAlgo;

pub trait Server {
    type Channel: Display + Clone + Eq + PartialEq + Hash;
    type Device: Clone;
    type Error;

    fn new(
        device: Self::Device,
        address: Address,
        p2_ctx: P2Context,
        did_cfg: HashMap<DataIdentifier, usize>,
        security_algo: SecurityAlgo,
    ) -> ResultWrapper<Self, Self::Error>
    where
        Self: Sized;

    fn security_access_level(&self, channel: Self::Channel) -> Option<u8>;

    fn session_level(&self, channel: Self::Channel) -> ResultWrapper<SessionType, Self::Error>;

    // fn is_programming_session(&self) -> bool;

    fn session_ctrl(&self, channel: Self::Channel) ->  ResultWrapper<(), Self::Error>;

    fn ecu_reset(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn security_access(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn communication_control(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    #[cfg(feature = "std2020")]
    fn authentication(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn tester_present(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    fn access_timing_parameter(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn secured_data_transmit(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn control_dtc_setting(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn response_on_event(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn link_control(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn read_data_by_identifier(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn read_memory_by_address(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn read_scaling_data_by_identifier(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn read_data_by_period_identifier(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn dynamically_define_data_by_identifier(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn write_data_by_identifier(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn write_memory_by_address(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn clear_dtc_info(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn read_dtc_info(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn io_control(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn routine_control(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn request_download(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn request_upload(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn transfer_data(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn request_transfer_exit(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn request_file_transfer(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn service_forever(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;

    fn service_stop(&self, channel: Self::Channel) -> ResultWrapper<(), Self::Error>;
}
