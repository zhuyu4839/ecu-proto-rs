//! page 174

use getset::{CopyGetters, Getters};
use crate::{DAQPackedMode, DAQPackedModeData, XcpError};

#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct GetDAQPackedMode {
    #[getset(skip)]
    reserved: u8,
    #[get_copy = "pub"]
    pub(crate) daq_packed_mode: DAQPackedMode,
    /// only if mode = 1/2
    #[get = "pub"]
    pub(crate) data: Option<DAQPackedModeData>,
}

impl GetDAQPackedMode {
    pub fn new(mode: DAQPackedMode, data: Option<DAQPackedModeData>) -> Result<Self, XcpError> {
        DAQPackedModeData::check_daq_packed_mode(mode, &data)?;

        Ok(Self { reserved: Default::default(), daq_packed_mode: mode, data })
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetDAQPackedMode {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.reserved, self.daq_packed_mode.into()];
        if let Some(data) = self.data {
            debug_assert!(self.daq_packed_mode != DAQPackedMode::NotPacked);
            result.push(data.timestamp_mode.into());
            result.extend(data.sample_count.to_be_bytes());
        }

        result
    }
}

impl TryFrom<&[u8]> for GetDAQPackedMode {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 1; // skip reserved
        let mode = DAQPackedMode::from(data[offset]);
        offset += 1;
        match mode {
            DAQPackedMode::NotPacked => {
                if data_len < expected {
                    return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                }

                Self::new(mode, None)
            },
            DAQPackedMode::ElementGroup |
            DAQPackedMode::EventGroup => {
                let data = DAQPackedModeData::try_from(&data[offset..])?;

                Self::new(mode, Some(data))
            },
            DAQPackedMode::Undefined(x) => {
                log::warn!("unsupported get_daq_packed_mode value: {}", x);
                Err(XcpError::UndefinedError)
            }
        }
    }
}
