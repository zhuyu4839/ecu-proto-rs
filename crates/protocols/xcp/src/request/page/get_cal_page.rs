use getset::CopyGetters;
use crate::{CalPageMode, XcpError};

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetCalPage {
    /// Mode may be 0x01 (ECU access) or 0x02 (XCP access). All other values are invalid.
    pub(crate) mode: CalPageMode,
    /// Logical data segment number
    pub(crate) segment: u8,
}

impl GetCalPage {
    pub fn new(mode: CalPageMode, segment: u8) -> Result<Self, XcpError> {
        if mode.all() {
            return Err(XcpError::InvalidECUAccessMode);
        }

        Ok(Self { mode, segment })
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetCalPage {
    fn into(self) -> Vec<u8> {
        vec![self.mode.into(), self.segment]
    }
}

impl TryFrom<&[u8]> for GetCalPage {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = CalPageMode::from(data[offset]);
        offset += 1;
        let segment = data[offset];

        Ok(Self::new(mode, segment)?)
    }
}
