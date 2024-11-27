use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::{TimestampUnit, XcpError};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Consistency {
    ODTLevel = 0x00,
    DAQListLevel = 0x01,
    EventChannelLevel = 0x02,
    NotAvailable = 0x03,
}

impl Consistency {
    #[inline]
    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

impl Into<u8> for Consistency {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for Consistency {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::ODTLevel,
            0x01 => Self::DAQListLevel,
            0x02 => Self::EventChannelLevel,
            0x03 => Self::NotAvailable,
            _ => Self::NotAvailable,
        }
    }
}

/// Bitfield representation of 4-bit `DAQ_EVENT_PROPERTIES parameter in GET_DAQ_EVENT_INFO`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | CONSISTENCY_EVENT       | 1           |
/// | CONSISTENCY_DAQ         | 1           |
/// | Reserved                | 2           |
/// | STIM                    | 1           |
/// | DAQ                     | 1           |
/// | Reserved                | 2           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DAQEventProperty {
    /// Consistency
    #[bits(2)]
    pub consistency: u8,    // Consistency
    __: bool,
    pub packed: bool,
    pub stim: bool,     // at least one of stim and daq must be true
    pub daq: bool,
    #[bits(2)]
    __: u8,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDAQEventInfo {
    /// Specific properties for this event channel
    pub(crate) property: DAQEventProperty,
    /// MAX_DAQ_LIST [0,1,2,..255]
    /// maximum number of DAQ lists in this event channel
    pub(crate) max_daq_list: u8,
    /// EVENT_CHANNEL_NAME_LENGTH in bytes
    /// 0 – If not available
    pub(crate) channel_name_length: u8,
    /// EVENT_CHANNEL_TIME_CYCLE
    /// 0 – Not cyclic
    pub(crate) channel_time_cycle: u8,
    /// EVENT_CHANNEL_TIME_UNIT
    /// do not care if Event channel time cycle = 0
    pub(crate) channel_time_unit: TimestampUnit,
    /// EVENT_CHANNEL_PRIORITY (FF highest)
    pub(crate) channel_priority: u8,
}

impl GetDAQEventInfo {
    pub fn new(
        property: DAQEventProperty,
        max_daq_list: u8,
        channel_name_length: u8,
        channel_time_cycle: u8,
        channel_time_unit: TimestampUnit,
        channel_priority: u8,
    ) -> Self {
        Self {
            property,
            max_daq_list,
            channel_name_length,
            channel_time_cycle,
            channel_time_unit,
            channel_priority,
        }
    }

    pub const fn length() -> usize {
        6
    }
}

impl Into<Vec<u8>> for GetDAQEventInfo {
    fn into(self) -> Vec<u8> {
        vec![
            self.property.into(),
            self.max_daq_list,
            self.channel_name_length,
            self.channel_time_cycle,
            self.channel_time_unit.into(),
            self.channel_priority,
        ]
    }
}

impl TryFrom<&[u8]> for GetDAQEventInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let property = DAQEventProperty::from(data[offset]);
        offset += 1;
        let max_daq_list = data[offset];
        offset += 1;
        let channel_name_length = data[offset];
        offset += 1;
        let channel_time_cycle = data[offset];
        offset += 1;
        let channel_time_unit = TimestampUnit::from(data[offset]);
        offset += 1;
        let channel_priority = data[offset];

        Ok(Self::new(
            property,
            max_daq_list,
            channel_name_length,
            channel_time_cycle,
            channel_time_unit,
            channel_priority,
        ))
    }
}
