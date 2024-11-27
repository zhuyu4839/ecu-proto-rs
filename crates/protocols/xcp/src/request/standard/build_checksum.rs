use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct BuildChecksum {
    #[getset(skip)]
    reserved0: u8,
    #[getset(skip)]
    reserved1: u16,
    pub(crate) block_size: u32,
}

impl BuildChecksum {
    pub fn new(block_size: u32) -> Self {
        Self { reserved0: Default::default(), reserved1: Default::default(), block_size }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for BuildChecksum {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.reserved0);
        result.extend(self.reserved1.to_be_bytes());
        result.extend(self.block_size.to_be_bytes());

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

        let offset = 3; // skip reserved length
        let block_size = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());

        Ok(Self::new(block_size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_checksum() -> anyhow::Result<()> {
        let request = BuildChecksum::new(0x10);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10]);

        let request = BuildChecksum::try_from(data.as_slice())?;
        assert_eq!(request.block_size, 0x10);

        Ok(())
    }
}
