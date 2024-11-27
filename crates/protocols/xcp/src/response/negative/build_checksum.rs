//! page 138

use getset::CopyGetters;
use crate::XcpError;

/// If MTA and block size does not meet alignment requirements, an ERR_OUT_OF_RANGE
/// with the required MTA_BLOCK_SIZE_ALIGN will be returned. If the block size exceeds
/// the allowed maximum value, an ERR_OUT_OF_RANGE will be returned. The maximum
/// block size will be returned in the checksum field.
#[derive(Debug, Clone, CopyGetters)]
pub struct BuildChecksum {
    pub(crate) mta_bs_align: u16,   // MTA_BLOCK_SIZE_ALIGN
    pub(crate) max_bs: u32,         // Maximum block size [AG]
}

impl BuildChecksum {
    pub fn new(mta_bs_align: u16, max_bs: u32) -> Self {
        Self { mta_bs_align, max_bs }
    }

    pub const fn length() -> usize {
        6
    }
}

impl Into<Vec<u8>> for BuildChecksum {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.extend(self.mta_bs_align.to_be_bytes());
        result.extend(self.max_bs.to_be_bytes());

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
        let mta_bs_align = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let max_bs = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);

        Ok(Self::new(mta_bs_align, max_bs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_checksum() -> anyhow::Result<()> {
        let response = BuildChecksum::new(0x1001, 0x12345678);
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x10, 0x01, 0x12, 0x34, 0x56, 0x78]);

        let response = BuildChecksum::try_from(data.as_slice())?;
        assert_eq!(response.mta_bs_align, 0x1001);
        assert_eq!(response.max_bs, 0x12345678);

        Ok(())
    }
}
