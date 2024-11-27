use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct CopyCalPage {
    pub(crate) src_segment_number: u8,
    pub(crate) src_page_number: u8,
    pub(crate) dist_segment_number: u8,
    pub(crate) dist_page_number: u8,
}

impl CopyCalPage {
    pub fn new(
        src_segment_number: u8,
        src_page_number: u8,
        dist_segment_number: u8,
        dist_page_number: u8
    ) -> Self {
        Self {
            src_segment_number,
            src_page_number,
            dist_segment_number,
            dist_page_number
        }
    }

    pub const fn length() -> usize {
        4
    }
}

impl Into<Vec<u8>> for CopyCalPage {
    fn into(self) -> Vec<u8> {
        vec![
            self.src_segment_number,
            self.src_page_number,
            self.dist_segment_number,
            self.dist_page_number,
        ]
    }
}

impl TryFrom<Vec<u8>> for CopyCalPage {
    type Error = XcpError;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let src_segment_number = data[offset];
        offset += 1;
        let src_page_number = data[offset];
        offset += 1;
        let dist_segment_number = data[offset];
        offset += 1;
        let dist_page_number = data[offset];

        Ok(Self::new(src_segment_number, src_page_number, dist_segment_number, dist_page_number))
    }
}
