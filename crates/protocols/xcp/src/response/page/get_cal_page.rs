use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetCalPage {
    #[getset(skip)]
    reserved0: u8,
    #[getset(skip)]
    reserved1: u8,
    pub(crate) page_number: u8,
}

impl GetCalPage {
    pub fn new(page_number: u8) -> Self {
        Self {
            reserved0: Default::default(),
            reserved1: Default::default(),
            page_number
        }
    }

    pub const fn length() -> usize {
        3
    }
}

impl Into<Vec<u8>> for GetCalPage {
    fn into(self) -> Vec<u8> {
        vec![self.reserved0, self.reserved1, self.page_number]
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

        let offset = 2; // skip reserved
        let page_number = data[offset];

        Ok(Self::new(page_number))
    }
}
