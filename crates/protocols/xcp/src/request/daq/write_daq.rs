use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct WriteDAQ {
    pub(crate) bit_offset: u8,
    pub(crate) size: u8,
    pub(crate) address_extension: u8,
    pub(crate) address: u32,
}

impl WriteDAQ {
    pub fn new(bit_offset: u8, size: u8, address_extension: u8, address: u32) -> Self {
        Self { bit_offset, size, address_extension, address }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for WriteDAQ {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.bit_offset, self.size, self.address_extension];
        result.extend(self.address.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for WriteDAQ {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let bit_offset = data[offset];
        offset += 1;
        let size = data[offset];
        offset += 1;
        let address_extension = data[offset + 1];
        offset += 1;
        let address = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());

        Ok(Self::new(bit_offset, size, address_extension, address))
    }
}
