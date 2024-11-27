use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::request::{ResponseFormat, TimeSyncBridge};

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct SlaveConfigure {
    pub(crate) response_fmt: ResponseFormat,
    pub(crate) daq_ts_relation: bool,
    pub(crate) time_sync_bridge: TimeSyncBridge,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EcuClock {
    ///  There is no ECU clock
    NoClock = 0x00,
    /// The XCP slave has access to the ECU clock and
    /// can read the clock randomly
    ReadRandomly = 0x01,
    /// The XCP slave has access to the ECU clock but
    /// cannot read the clock randomly. However, the
    /// XCP slave autonomously generates
    /// EV_TIME_SYNC events containing timestamps
    /// related to the XCP slave’s clock and the ECU
    /// clock. Thereby, these timestamps have been
    /// sampled simultaneously.
    NotReadRandomly = 0x02,
    /// The XCP slave reports ECU clock based
    /// timestamps whereas the XCP slave cannot read
    /// the ECU clock
    ClockNotReadable = 0x03,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GrandmasterClock {
    /// There is no dedicated clock in the XCP slave that is synchronized to a grandmaster clock
    NoClock = 0x00,
    /// The XCP slave offers a dedicated clock that might
    /// be synchronized to a grandmaster clock and can
    /// be read randomly
    ReadRandomly = 0x01,
    /// The XCP slave offers a dedicated clock that might
    /// be synchronized to a grandmaster clock. The clock
    /// cannot be read randomly. However, the XCP slave
    /// autonomously generates EV_TIME_SYNC events
    /// containing timestamps related to the XCP slave’s
    /// clock and the clock which is synchronized to a
    /// grandmaster clock. Thereby, these timestamps
    /// have been sampled simultaneously.
    NotReadRandomly = 0x02,
    Reserved = 0x03,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum XcpSlaveClock {
    /// Free running XCP slave clock that can be read randomly
    FreeRunning = 0x00,
    /// XCP slave clock might be syntonized or synchronized
    /// to a grandmaster clock and can be read randomly
    SyntonizedOrSynchronized = 0x01,
    /// There is no XCP slave clock. Nevertheless, DAQ timestamps
    /// might be related to a synchronized clock.
    NoXcpSlaveClock = 0x02,
    Reserved = 0x03,
}

#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct ObservableClock {
    #[bits(2)]
    __: u8,
    #[bits(2)]
    pub ecu_clock: u8,
    #[bits(2)]
    pub grandmaster_clock: u8,
    #[bits(2)]
    pub xcp_slave_clock: u8,
}

#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct ClockInfo {
    #[bits(3)]
    __: u8,
    pub ecu_grandmaster_clock_info: bool,
    pub ecu_clock_info: bool,
    pub clock_relation: bool,
    pub grandmaster_clock_info: bool,
    pub slave_clock_info: bool,
}
