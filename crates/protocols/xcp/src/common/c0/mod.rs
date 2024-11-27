use getset::CopyGetters;
use crate::{DAQPackedMode, XcpError};

/// DAQ Packed Mode timestamp mode(2 bit)
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DPMTimestampMode {
    SingleTimestampOfLastSample = 0x00,
    SingleTimestampOfFirstSample = 0x01,
    Undefined(u8),
}

impl Into<u8> for DPMTimestampMode {
    fn into(self) -> u8 {
        match self {
            Self::SingleTimestampOfLastSample => 0x00,
            Self::SingleTimestampOfFirstSample => 0x01,
            Self::Undefined(x) => x,
        }
    }
}

impl From<u8> for DPMTimestampMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::SingleTimestampOfLastSample,
            0x01 => Self::SingleTimestampOfFirstSample,
            _ => Self::Undefined(value),
        }
    }
}


#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct DAQPackedModeData {
    pub(crate) timestamp_mode: DPMTimestampMode,    // DPMTimestampMode
    pub(crate) sample_count: u16,
}

impl DAQPackedModeData {
    #[inline]
    pub fn new(timestamp_mode: DPMTimestampMode, sample_count: u16) -> Self {
        Self { timestamp_mode, sample_count }
    }

    #[inline]
    pub const fn length() -> usize {
        3
    }

    #[inline]
    pub(crate) fn check_daq_packed_mode(
        mode: DAQPackedMode,
        data: &Option<DAQPackedModeData>,
    ) -> Result<(), XcpError> {
        match mode {
            DAQPackedMode::NotPacked => {
                if data.is_some() {
                    return Err(XcpError::UnexpectInput(
                        format!("`DAQPackedModeData` is unnecessary when mode: {:?}", mode)
                    ));
                }
            },
            DAQPackedMode::ElementGroup |
            DAQPackedMode::EventGroup => {
                if data.is_none() {
                    return Err(XcpError::UnexpectInput(
                        format!("`DAQPackedModeData` is necessary when mode: {:?}", mode)
                    ));
                }
            },
            DAQPackedMode::Undefined(x) => {
                log::warn!("unsupported get_daq_packed_mode value: {}", x);
                return Err(XcpError::UndefinedError);
            }
        }

        Ok(())
    }
}

impl Into<Vec<u8>> for DAQPackedModeData {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.timestamp_mode.into(), ];
        result.extend(self.sample_count.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for DAQPackedModeData {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let timestamp_mode = DPMTimestampMode::from(data[offset]);
        offset += 1;
        let sample_count = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(timestamp_mode, sample_count))
    }
}
