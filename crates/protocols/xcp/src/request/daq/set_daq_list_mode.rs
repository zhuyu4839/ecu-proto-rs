//! page

use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// Bitfield representation of 4-bit `Mode parameter in SET_DAQ_LIST_MODE`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Reserved                | 2           |
/// | PID_OFF                 | 1           |
/// | TIMESTAMP               | 1           |
/// | DTO_CTR                 | 1           |
/// | Reserved                | 1           |
/// | DIRECTION               | 1           |
/// | ALTERNATING             | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DAQListModeSet {
    #[bits(2)]
    __: u8,
    pub pid_off: bool,
    pub timestamp: bool,
    pub dto_ctr: bool,
    __: bool,
    pub direction: bool,
    pub alternating: bool,
}


#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct SetDAQListMode {
    pub(crate) mode: DAQListModeSet,
    pub(crate) daq_list_number: u16,
    pub(crate) event_channel_number: u16,
    pub(crate) transmission_rate_prescaler: u8,
    pub(crate) daq_list_priority:  u8,
}

impl SetDAQListMode {
    pub fn new(
        mode: DAQListModeSet,
        daq_list_number: u16,
        event_channel_number: u16,
        transmission_rate_prescaler: u8,
        daq_list_priority:  u8,
    ) -> Self {
        Self {
            mode,
            daq_list_number,
            event_channel_number,
            transmission_rate_prescaler,
            daq_list_priority,
        }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for SetDAQListMode {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.mode.into());
        result.extend(self.daq_list_number.to_be_bytes());
        result.extend(self.event_channel_number.to_be_bytes());
        result.push(self.transmission_rate_prescaler);
        result.push(self.daq_list_priority);

        result
    }
}

impl TryFrom<&[u8]> for SetDAQListMode {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = DAQListModeSet::from(data[offset]);
        offset += 1;
        let daq_list_number = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let event_channel_number = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let transmission_rate_prescaler = data[offset];
        offset += 1;
        let daq_list_priority = data[offset];

        Ok(Self::new(
            mode,
            daq_list_number,
            event_channel_number,
            transmission_rate_prescaler,
            daq_list_priority
        ))
    }
}
