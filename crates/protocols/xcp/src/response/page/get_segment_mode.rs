use getset::CopyGetters;
use crate::{SegmentMode, XcpError};

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSegmentMode {
    #[getset(skip)]
    reserved: u8,
    pub(crate) mode: SegmentMode,
}

impl GetSegmentMode {
    pub fn new(mode: SegmentMode) -> Self {
        Self { reserved: Default::default(), mode }
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetSegmentMode {
    fn into(self) -> Vec<u8> {
        vec![self.reserved, self.mode.into()]
    }
}

impl TryFrom<&[u8]> for GetSegmentMode {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let offset = 1; // skip reserved
        let mode = SegmentMode::from(data[offset]);

        Ok(Self::new(mode))
    }
}
