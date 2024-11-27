use crate::XcpError;

#[derive(Debug, Clone)]
pub struct CopyCalPage;

impl CopyCalPage {
    pub fn new() -> Self {
        Self {}
    }

    pub const fn length() -> usize {
        0
    }
}

impl Into<Vec<u8>> for CopyCalPage {
    fn into(self) -> Vec<u8> {
        vec![]
    }
}

impl TryFrom<&[u8]> for CopyCalPage {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        Ok(Self::new())
    }
}
