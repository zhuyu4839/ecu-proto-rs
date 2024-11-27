//! page 174

use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDAQPackedMode {
    pub(crate) daq_list_number: u16,
}

impl GetDAQPackedMode {
    pub fn new(daq_list_number: u16) -> Self {
        Self { daq_list_number }
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetDAQPackedMode {
    fn into(self) -> Vec<u8> {
        self.daq_list_number.to_be_bytes().to_vec()
    }
}

impl TryFrom<&[u8]> for GetDAQPackedMode {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, XcpError> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let daq_list_number = u16::from_be_bytes([data[0], data[1]]);

        Ok(GetDAQPackedMode::new(daq_list_number))
    }
}
