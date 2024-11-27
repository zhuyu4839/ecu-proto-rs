use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetPageInfo {
    #[getset(skip)]
    reserved: u8,
    pub(crate) segment_number: u8,
    pub(crate) page_number: u8,
}

impl GetPageInfo {
    pub fn new(segment_number: u8, page_number: u8) -> GetPageInfo {
        Self { reserved: Default::default(), segment_number, page_number }
    }

    pub const fn length() -> usize {
        3
    }
}

impl Into<Vec<u8>> for GetPageInfo {
    fn into(self) -> Vec<u8> {
        vec![self.reserved, self.segment_number, self.page_number]
    }
}

impl TryFrom<&[u8]> for GetPageInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 1; // skip reserved
        let segment_number = data[offset];
        offset += 1;
        let page_number = data[offset];

        Ok(Self::new(segment_number, page_number))
    }
}
