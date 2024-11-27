use getset::CopyGetters;
use crate::{GetSectorInfoMode, XcpError};

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSectorInfo {
    pub(crate) mode: GetSectorInfoMode,
    pub(crate) sector_number: u8,
}

impl GetSectorInfo {
    pub fn new(mode: GetSectorInfoMode, sector_number: u8) -> Self {
        Self { mode, sector_number }
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetSectorInfo {
    fn into(self) -> Vec<u8> {
        vec![self.mode.into(), self.sector_number]
    }
}

impl TryFrom<&[u8]> for GetSectorInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = GetSectorInfoMode::from(data[offset]);
        offset += 1;
        let sector_number = data[offset];

        Ok(Self::new(mode, sector_number))
    }
}
