use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChecksumType {
    Add11 = 0x01,
    Add12 = 0x02,
    Add14 = 0x03,
    Add22 = 0x04,
    Add24 = 0x05,
    Add44 = 0x06,
    Crc16 = 0x07,
    Crc16CITT = 0x08,
    Crc32 = 0x09,
    Undefined(u8),
    UserDefine = 0xFF,
}

impl Into<u8> for ChecksumType {
    fn into(self) -> u8 {
        match self {
            Self::Add11 => 0x01,
            Self::Add12 => 0x02,
            Self::Add14 => 0x03,
            Self::Add22 => 0x04,
            Self::Add24 => 0x05,
            Self::Add44 => 0x06,
            Self::Crc16 => 0x07,
            Self::Crc16CITT => 0x08,
            Self::Crc32 => 0x09,
            Self::UserDefine => 0xFF,
            Self::Undefined(x) => x,
        }
    }
}

impl From<u8> for ChecksumType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::Add11,
            0x02 => Self::Add12,
            0x03 => Self::Add14,
            0x04 => Self::Add22,
            0x05 => Self::Add24,
            0x06 => Self::Add44,
            0x07 => Self::Crc16,
            0x08 => Self::Crc16CITT,
            0x09 => Self::Crc32,
            0xFF => Self::UserDefine,
            _ => Self::Undefined(value),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct BuildChecksum {
    pub(crate) r#type: ChecksumType,
    #[getset(skip)]
    reserved: u16,
    pub(crate) checksum: u32,
}

impl BuildChecksum {
    pub fn new(r#type: ChecksumType, checksum: u32) -> Self {
        Self { r#type, reserved: Default::default(), checksum }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for BuildChecksum {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.r#type.into());
        result.extend(self.reserved.to_be_bytes());
        result.extend(self.checksum.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for BuildChecksum {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let r#type = ChecksumType::from(data[offset]);
        offset += 1;
        offset += 2;    // skip reserved length
        let checksum = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);

        Ok(Self::new(r#type, checksum))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_checksum() -> anyhow::Result<()> {
        let response = BuildChecksum::new(ChecksumType::Add11, 0x12345678);
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x01, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78]);

        let response = BuildChecksum::try_from(data.as_slice())?;
        assert_eq!(response.r#type, ChecksumType::Add11);
        assert_eq!(response.checksum, 0x12345678);

        Ok(())
    }
}
