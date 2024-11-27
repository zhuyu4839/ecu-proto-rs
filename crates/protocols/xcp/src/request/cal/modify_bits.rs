use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ModifyBits {
    pub(crate) shift_value: u8,
    pub(crate) and_mask: u16,
    pub(crate) xor_mask: u16,
}

impl ModifyBits {
    pub fn new(shift_value: u8, and_mask: u16, xor_mask: u16) -> Self {
        Self { shift_value, and_mask, xor_mask }
    }

    pub const fn length() -> usize {
        5
    }
}

impl Into<Vec<u8>> for ModifyBits {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.shift_value);
        result.extend(self.and_mask.to_be_bytes());
        result.extend(self.xor_mask.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for ModifyBits {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let shift_value = data[offset];
        offset += 1;
        let and_mask = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let xor_mask = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(shift_value, and_mask, xor_mask))
    }
}
