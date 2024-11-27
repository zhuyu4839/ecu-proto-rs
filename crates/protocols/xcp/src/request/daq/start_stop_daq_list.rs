use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StartStopDaqListMode {
    Stop = 0x00,
    Start = 0x01,
    Select = 0x02,
    Undefined(u8),
}

impl Into<u8> for StartStopDaqListMode {
    fn into(self) -> u8 {
        match self {
            Self::Stop => 0x00,
            Self::Start => 0x01,
            Self::Select => 0x02,
            Self::Undefined(c) => c,
        }
    }
}

impl From<u8> for StartStopDaqListMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Stop,
            0x01 => Self::Start,
            0x02 => Self::Select,
            _ => Self::Undefined(value),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct StartStopDAQList {
    pub(crate) mode: StartStopDaqListMode,
    pub(crate) daq_list_number: u16,
}

impl StartStopDAQList {
    pub fn new(mode: StartStopDaqListMode, daq_list_number: u16) -> Self {
        Self { mode, daq_list_number }
    }

    pub const fn length() -> usize {
        3
    }
}

impl Into<Vec<u8>> for StartStopDAQList {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.mode.into());
        result.extend(self.daq_list_number.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for StartStopDAQList {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = StartStopDaqListMode::from(data[offset]);
        offset += 1;
        let daq_list_number = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(mode, daq_list_number))
    }
}
