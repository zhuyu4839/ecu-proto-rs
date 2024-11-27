//! STIM - synchronous data STIMulation packet
//! DAQ - Data Acquisition
//! ODT - Object Descriptor Table
//!
//! Table 229 Standard commands error handling
//! Table 230 Calibration commands error handling
//! Table 231 Page switching commands error handling
//! Table 232 Data acquisition and stimulation commands error handling
//! Table 233 Non-volatile memory programming commands error handling
//! Table 234 Time Synchronization commands error handling
//!

mod constants;
use constants::*;

pub mod event;
pub mod request;
pub mod response;
mod common;
pub use common::*;
mod error;
pub use error::*;
mod packet_id;
pub use packet_id::*;

/// TODO
mod config;

use getset::CopyGetters;

pub trait IntoWith<T, V> {
    fn into_with(self, val: V) -> T;
}

pub trait TryFromWith<T, V>: Sized {
    type Error;
    fn try_from_with(data: T, val: V) -> Result<Self, Self::Error>;
}


#[derive(Debug, Copy, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct XcpClockUuid {
    pub(crate) uuid: u64,  // 8 bit
}

/// Table 226 Parameter encoding of clock information characteristics
///
/// <table>
///   <tbody>
///     <tr>
///       <td><br>Epoch of grandmaster’s clock
///         EndFragment</td>
///       <td><br>0 = Atomic Time (TAI)<br>1 = Universal Coordinated Time (UTC)<br>2 = arbitrary (unknown)</td>
///     </tr>
///     <tr>
///       <td>Clock quality categorized by Stratum Level</td>
///       <td><br>Stratum level as described in ANSI Synchronization&nbsp;<br>Interface Standard T1.101, ITU standard G.810,&nbsp;<br>Telecordia/Bellcore standards GR-253 and GR-1244,<br>255 if unknown</td>
///     </tr>
///     <tr>
///       <td><br>Native timestamp size</td>
///       <td><br>Size of the counter from which the timestamps are&nbsp;<br>taken and are sent as part of an EV_TIME_SYNC<br>event. The timestamps have always to be&nbsp;<br>interpreted as unsigned, independent of the size.<br>4 = 4 bytes (DWORD)<br>8 = 8 bytes (DLONG)<br>others = not allowed<br>Remark for SLV_CLK_INFO &amp; ECU_CLK_INFO:<br>When the size of the DAQ timestamps (see Table&nbsp;<br>146 &amp; Table 147) is less than the native timestamp&nbsp;<br>size of the data acquisition clock, the DAQ&nbsp;<br>timestamps correlate to the least significant bytes of&nbsp;<br>the timestamp of the data acquisition clock.</td>
///     </tr>
///   </tbody>
///   <colgroup>
///     <col>
///     <col>
///   </colgroup>
/// </table>

/// Table 221 in page 236
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct XcpSlaveClockInfo {
    pub(crate) uuid: XcpClockUuid,
    /// Timestamp Ticks of XCP slave’s clock(see Table 145)
    pub(crate) timestamp: u32,
    /// Timestamp Unit of XCP slave’s clock (Table 148)
    pub(crate) timestamp_unit: TimestampUnit,
    /// Clock quality categorized by Stratum Level (see Table 226)
    pub(crate) clock_quality: u8,
    /// Native timestamp size (see Table 226)
    pub(crate) native_ts_size: u8,
    #[getset(skip)]
    reserved0: u8,
    #[getset(skip)]
    reserved1: u16,
    /// The last valid timestamp value before the counter wraps
    /// to 0
    /// MAX_TIMESTAMP_VALUE_BEFORE_WRAP_AROUND
    pub(crate) last_valid_timestamp: u64,
}

/// Table 222 in page 236
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct SlaveGrandmasterClockInfo {
    pub(crate) uuid: XcpClockUuid,
    /// Timestamp Ticks of grandmaster’s clock
    pub(crate) timestamp_tick: u16,
    /// Timestamp Unit of grandmaster’s clock
    pub(crate) timestamp_unit: TimestampUnit,
    /// Clock quality categorized by Stratum Level
    pub(crate) clock_quality: u8,
    /// Native timestamp size
    pub(crate) native_ts_size: u8,
    /// Epoch of grandmaster’s clock (see Table 226)
    epoch: u8,
    #[getset(skip)]
    reserved: u16,
    /// The last valid timestamp value before the counter wraps
    /// to 0
    /// MAX_TIMESTAMP_VALUE_BEFORE_WRAP_AROUND
    pub(crate) last_valid_timestamp: u64,
}

/// Table 223 in page 237
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ClockRelationInfo {
    /// Origin (timestamp) of XCP slave’s clock in
    /// grandmaster’s clock’s time domain
    pub(crate) origin: u64,
    /// XCP slave’s timestamp
    pub(crate) slave_timestamp: u64,
}

/// Table 224 in page 237
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ECUClockInfo {
    pub(crate) uuid: XcpClockUuid,
    /// Timestamp Ticks of ECU clock
    pub(crate) timestamp_tick: u16,
    /// Timestamp Unit of ECU clock
    pub(crate) timestamp_unit: TimestampUnit,
    /// Clock quality categorized by Stratum Level
    pub(crate) clock_quality: u8,
    /// Native timestamp size
    pub(crate) native_ts_size: u8,
    #[getset(skip)]
    reserved0: u8,
    #[getset(skip)]
    reserved1: u16,
    /// The last valid timestamp value before the counter wraps
    /// to 0
    /// MAX_TIMESTAMP_VALUE_BEFORE_WRAP_AROUND
    pub(crate) last_valid_timestamp: u64,
}
