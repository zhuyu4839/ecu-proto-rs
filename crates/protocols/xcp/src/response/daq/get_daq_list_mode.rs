use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// Bitfield representation of 7-bit `Current Mode parameter in GET_DAQ_LIST_MODE`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Resume                  | 1           |
/// | Running                 | 1           |
/// | PID_OFF                 | 1           |
/// | TIMESTAMP               | 1           |
/// | DTO_CTR                 | 1           |
/// | Reserved                | 1           |
/// | DIRECTION               | 1           |
/// | SELECTED                | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DAQListModeGet {
    pub resume: bool,
    pub running: bool,
    pub pid_off: bool,
    pub timestamp: bool,
    pub dto_ctr: bool,
    __: bool,
    pub direction: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDAQListMode {
    pub(crate) mode: DAQListModeGet,
    #[getset(skip)]
    reserved: u16,
    pub(crate) event_channel_number: u16,
    pub(crate) prescaler: u8,
    pub(crate) daq_list_priority: u8,
}

impl GetDAQListMode {
    pub fn new(
        mode: DAQListModeGet,
        event_channel_number: u16,
        prescaler: u8,
        daq_list_priority: u8,
    ) -> Self {
        Self {
            mode,
            reserved: Default::default(),
            event_channel_number,
            prescaler,
            daq_list_priority,
        }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for GetDAQListMode {
    fn into(self) -> Vec<u8> {
        let mut reuslt = Vec::with_capacity(Self::length());
        reuslt.push(self.mode.into());
        reuslt.extend(self.reserved.to_be_bytes());
        reuslt.extend(self.event_channel_number.to_be_bytes());
        reuslt.push(self.prescaler);
        reuslt.push(self.daq_list_priority);

        reuslt
    }
}

impl TryFrom<&[u8]> for GetDAQListMode {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = DAQListModeGet::from(data[offset]);
        offset += 1;
        offset += 2;    // skip reserved
        let event_channel_number = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let prescaler = data[offset];
        offset += 1;
        let daq_list_priority = data[offset];

        Ok(Self::new(mode, event_channel_number, prescaler, daq_list_priority))
    }
}
