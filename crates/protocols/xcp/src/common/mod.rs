mod c0;
pub use c0::*;
mod daq;
pub use daq::*;
mod page;
pub use page::*;
mod pgm;
pub use pgm::*;
mod standard;
pub use standard::*;

use bitfield_struct::bitfield;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AddressGranularity {
    Byte = 0x00,    // byte length 1
    Word = 0x01,    // byte length 2
    DWord = 0x02,   // byte length 4
    // DLong = 0x03,   // byte length 8
}

impl AddressGranularity {
    pub fn bytes(&self) -> usize {
        match self {
            Self::Byte => 0x01,
            Self::Word => 0x02,
            Self::DWord => 0x04,
            // Self::DLong => 0x08,
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

impl Into<u8> for AddressGranularity {
    #[inline]
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for AddressGranularity {
    #[inline]
    fn from(bits: u8) -> Self {
        match bits {
            0x00 => Self::Byte,
            0x01 => Self::Word,
            0x02 => Self::DWord,
            // 0x03 => Self::DLong,
            _ => Self::DWord,
        }
    }
}

/// | Service Request | Code | Description |
/// | ---- | ---- | ----------- |
/// | SERV_RESET | 0x01 | Slave requesting to be reset |
/// | SERV_TEXT | 0x02 | Slave transferring a byte stream of plain ASCII text. The line separator is LF or CR/LF. The text can be transferred in consecutive packets. The end of the overall text is indicated by the last packet containing a Null terminated string. |
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ServiceCode {
    Reset = 0x00,
    Text = 0x01,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DAQPackedMode {
    NotPacked = 0x00,
    ElementGroup = 0x01,
    EventGroup = 0x02,
    /// 3..127 = future extensions
    /// 128..255 = reserved
    Undefined(u8),
}

impl Into<u8> for DAQPackedMode {
    fn into(self) -> u8 {
        match self {
            Self::NotPacked => 0x00,
            Self::ElementGroup => 0x01,
            Self::EventGroup => 0x02,
            Self::Undefined(x) => x,
        }
    }
}

impl From<u8> for DAQPackedMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotPacked,
            0x01 => Self::ElementGroup,
            0x02 => Self::EventGroup,
            _ => Self::Undefined(value),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TimestampUnit {
    Unit1ns = 0x00,
    Unit10ns = 0x01,
    Unit100ns = 0x02,
    Unit1us = 0x03,
    Unit10us = 0x04,
    Unit100us = 0x05,
    Unit1ms = 0x06,
    Unit10ms = 0x07,
    Unit100ms = 0x08,
    Unit1s = 0x09,
    Unit1ps = 0x0A,
    Unit10ps = 0x0B,
    Unit100ps = 0x0D,
}

impl Into<u8> for TimestampUnit {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for TimestampUnit {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Unit1ns,
            0x01 => Self::Unit10ns,
            0x02 => Self::Unit100ns,
            0x03 => Self::Unit1us,
            0x04 => Self::Unit10us,
            0x05 => Self::Unit100us,
            0x06 => Self::Unit1ms,
            0x07 => Self::Unit10ms,
            0x08 => Self::Unit100ms,
            0x09 => Self::Unit1s,
            0x0A => Self::Unit1ps,
            0x0B => Self::Unit10ps,
            0x0D => Self::Unit100ps,
            _ => Self::Unit1us,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ClockFormat {
    DWord = 0x01,
    DLong = 0x02,
    /// include
    /// 0x00 = not part of event payload
    /// 0x03 = reserved
    Undefined(u8),
}

impl From<u8> for ClockFormat {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::DWord,
            0x02 => Self::DLong,
            _ => Self::Undefined(value),
        }
    }
}

impl Into<u8> for ClockFormat {
    fn into(self) -> u8 {
        match self {
            Self::DWord => 0x01,
            Self::DLong => 0x02,
            Self::Undefined(val) => val,
        }
    }
}

#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct PayloadFormat {
    __: bool,
    pub cluster_id: bool,
    /// ClockFormat
    #[bits(2)]
    pub fmt_ecu: u8,
    /// ClockFormat
    #[bits(2)]
    pub fmt_grandmaster: u8,
    /// ClockFormat
    #[bits(2)]
    pub fmt_xcp_slave: u8,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EcuClockSyncState {
    NotSynchronized = 0x00,
    Synchronized = 0x01,
    Unknown = 0x02,
    Reserved = 0x03,
}

impl From<u8> for EcuClockSyncState {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotSynchronized,
            0x01 => Self::Synchronized,
            0x02 => Self::Unknown,
            _ => Self::Reserved,
        }
    }
}

impl Into<u8> for EcuClockSyncState {
    fn into(self) -> u8 {
        self as _
    }
}

/// Table 217 SYNC_STATE parameter bit mask structure
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct SyncState {
    #[bits(2)]
    __: u8,
    /// EcuClockSyncState
    #[bits(2)]
    ecu_clock_sync_state: u8,
    grandmaster_clock_sync_state: bool,
    /// SlaveClockSyncState
    #[bits(3)]
    slave_clock_sync_state: u8,
}
