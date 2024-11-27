use getset::CopyGetters;
use crate::{CalPageMode, XcpError};

#[derive(Debug, Clone, CopyGetters)]
pub struct SetCalPage {
    pub(crate) mode: CalPageMode,
    pub(crate) segment: u8,     // Logical data segment number
    pub(crate) page: u8,        // Logical data page number
}

impl SetCalPage {
    pub fn new(mode: CalPageMode, segment: u8, page: u8) -> Self {
        Self { mode, segment, page }
    }

    pub const fn length() -> usize {
        3
    }
}

impl Into<Vec<u8>> for SetCalPage {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.mode.into());
        result.push(self.segment);
        result.push(self.page);

        result
    }
}

impl TryFrom<&[u8]> for SetCalPage {
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
        offset += 1;
        let page = data[offset];

        Ok(Self::new(mode, segment, page))
    }
}
