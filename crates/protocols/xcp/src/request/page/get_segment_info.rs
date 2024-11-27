//! page 152

use getset::CopyGetters;
use crate::{SegmentInfoMode, XcpError};

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSegmentInfo {
    pub(crate) mode: SegmentInfoMode,
    /// SEGMENT_NUMBER [0,1,..MAX_SEGMENTS-1]
    pub(crate) size: u8,
    /// Mode 0: 0 = address
    ///         1 = length
    /// Mode 1: do not care
    /// Mode 2: 0 = source address
    ///         1 = destination address
    ///         2 = length address
    pub(crate) info: u8,
    /// Mode 0: do not care
    /// Mode 1: do not care
    /// Mode 2: identifier for address mapping range that MAPPING_INFO belongs to
    pub(crate) mapping_index: u8,
}

impl GetSegmentInfo {
    pub fn new(mode: SegmentInfoMode, size: u8, info: u8, mapping_index: u8) -> Self {
        Self { mode, size, info, mapping_index }
    }

    pub const fn length() -> usize {
        4
    }
}

impl Into<Vec<u8>> for GetSegmentInfo {
    #[inline]
    fn into(self) -> Vec<u8> {
        vec![self.mode.into(), self.size, self.info, self.mapping_index]
    }
}

impl TryFrom<&[u8]> for GetSegmentInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = SegmentInfoMode::from(data[offset]);
        offset += 1;
        let size = data[offset];
        offset += 1;
        let info = data[offset];
        offset += 1;
        let mapping_index = data[offset];

        Ok(Self::new(mode, size, info, mapping_index))
    }
}
