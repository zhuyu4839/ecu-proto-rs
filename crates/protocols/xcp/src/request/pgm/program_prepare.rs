use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ProgramPrepare {
    #[getset(skip)]
    reserved: u8,
    /// Codesize is expressed in BYTE, WORD or DWORD depending upon AG.
    pub(crate) code_size: u16,
}

impl ProgramPrepare {
    pub fn new(code_size: u16) -> Self {
        Self { reserved: Default::default(), code_size }
    }

    pub const fn length() -> usize {
        3
    }
}

impl Into<Vec<u8>> for ProgramPrepare {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.reserved, ];
        result.extend(self.code_size.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for ProgramPrepare {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let offset = 1; // skip reserved
        let code_size = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(code_size))
    }
}
