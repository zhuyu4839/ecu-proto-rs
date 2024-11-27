use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StartStopSynchMode {
    StopAll = 0x00,
    StartSelect = 0x01,
    StopSelect = 0x02,
    PrepareForStartSelect = 0x03,
    Undefined(u8),
}

impl Into<u8> for StartStopSynchMode {
    fn into(self) -> u8 {
        match self {
            Self::StopAll => 0,
            Self::StartSelect => 1,
            Self::StopSelect => 2,
            Self::PrepareForStartSelect => 3,
            Self::Undefined(i) => i,
        }
    }
}

impl From<u8> for StartStopSynchMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::StopAll,
            0x01 => Self::StartSelect,
            0x02 => Self::StopSelect,
            0x03 => Self::PrepareForStartSelect,
            _ => Self::Undefined(value),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct StartStopSynch {
    pub(crate) mode: StartStopSynchMode,
}

impl StartStopSynch {
    pub fn new(mode: StartStopSynchMode) -> Self {
        Self { mode }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for StartStopSynch {
    fn into(self) -> Vec<u8> {
        vec![self.mode.into(), ]
    }
}

impl TryFrom<&[u8]> for StartStopSynch {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mode = StartStopSynchMode::from(data[0]);

        Ok(Self::new(mode))
    }
}
