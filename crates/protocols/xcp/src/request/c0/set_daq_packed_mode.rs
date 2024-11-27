//! page 172

use getset::{CopyGetters, Getters};
use crate::{DAQPackedMode, DAQPackedModeData, XcpError};

#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct SetDAQPackedMode {
    #[get_copy = "pub"]
    pub(crate) daq_list_number: u16,
    #[get_copy = "pub"]
    pub(crate) daq_packed_mode: DAQPackedMode,
    /// only if mode = 1/2
    #[get = "pub"]
    pub(crate) content: Option<DAQPackedModeData>,
}

impl SetDAQPackedMode {
    pub fn new(
        daq_list_number: u16,
        mode: DAQPackedMode,
        content: Option<DAQPackedModeData>,
    ) -> Result<Self, XcpError> {
        DAQPackedModeData::check_daq_packed_mode(mode, &content)?;

        Ok(Self { daq_list_number, daq_packed_mode: mode, content })
    }

    pub const fn length() -> usize {
        3
    }
}

impl Into<Vec<u8>> for SetDAQPackedMode {
    fn into(self) -> Vec<u8> {
        let mut result = self.daq_list_number.to_be_bytes().to_vec();
        result.push(self.daq_packed_mode.into());
        if let Some(data) = self.content {
            debug_assert!(self.daq_packed_mode != DAQPackedMode::NotPacked);
            result.append(&mut data.into());
        }

        result
    }
}

impl TryFrom<&[u8]> for SetDAQPackedMode {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let daq_list_number = u16::from_be_bytes([data[offset], data[offset+1]]);
        offset += 2;
        let daq_packed_mode = DAQPackedMode::from(data[offset]);
        offset += 1;

        match daq_packed_mode {
            DAQPackedMode::NotPacked => {
                if data_len < expected {
                    return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                }

                Self::new(daq_list_number, daq_packed_mode, None)
            },
            DAQPackedMode::ElementGroup |
            DAQPackedMode::EventGroup => {
                let data = DAQPackedModeData::try_from(&data[offset..])?;
                Self::new(daq_list_number, daq_packed_mode, Some(data))
            },
            DAQPackedMode::Undefined(x) => {
                log::warn!("unsupported get_daq_packed_mode value: {}", x);
                Err(XcpError::UndefinedError)
            }
        }
    }
}
