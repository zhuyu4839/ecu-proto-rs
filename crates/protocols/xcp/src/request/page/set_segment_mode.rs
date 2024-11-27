use getset::CopyGetters;
use crate::{SegmentMode, XcpError};

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct SetSegmentMode {
    pub(crate) mode: SegmentMode,
    pub(crate) number: u8,
}

impl SetSegmentMode {
    pub fn new(mode: SegmentMode, number: u8) -> Self {
        Self { mode, number }
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for SetSegmentMode {
    fn into(self) -> Vec<u8> {
        vec![self.mode.into(), self.number]
    }
}

impl TryFrom<&[u8]> for SetSegmentMode {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = SegmentMode::from(data[offset]);
        offset += 1;
        let number = data[offset];

        Ok(Self::new(mode, number))
    }
}

