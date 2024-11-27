use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct AllocDAQ {
    #[getset(skip)]
    reserved: u8,
    pub(crate) daq_count: u16,
}

impl AllocDAQ {
    pub fn new(daq_count: u16) -> Self {
        Self { reserved: Default::default(), daq_count }
    }

    pub const fn length() -> usize {
        3
    }
}


impl Into<Vec<u8>> for AllocDAQ {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.reserved);
        result.extend(self.daq_count.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for AllocDAQ {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let offset = 1; // skip reserved
        let daq_count = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(daq_count))
    }
}
