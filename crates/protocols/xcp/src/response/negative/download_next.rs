use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct DownloadNext {
    /// Number of expected data elements
    pub(crate) expected_size: u8,
}

impl DownloadNext {
    pub fn new(expected_size: u8) -> Self {
        Self { expected_size }
    }

    #[inline]
    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for DownloadNext {
    fn into(self) -> Vec<u8> {
        vec![self.expected_size, ]
    }
}

impl TryFrom<&[u8]> for DownloadNext {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        Ok(Self::new(data[0]))
    }
}
