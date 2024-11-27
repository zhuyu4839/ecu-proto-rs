use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct EcuStateChange {
    pub(crate) state: u8,
}

impl EcuStateChange {
    pub fn new(state: u8) -> Self {
        Self { state }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for EcuStateChange {
    fn into(self) -> Vec<u8> {
        vec![self.state, ]
    }
}

impl TryFrom<&[u8]> for EcuStateChange {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength {  expected, actual: data_len });
        }

        let state = data[0];

        Ok(Self::new(state))
    }
}
