use getset::CopyGetters;
use crate::XcpError;

/// This command will initialize a pointer (32Bit address + 8Bit extension) for following
/// memory transfer commands.
///
/// ref [Command::BuildCheckSum](#) [Command::Upload](#) [Command::CALDownload](#)
///
/// [Command::CALDownloadNext](#) [Command::CALDownloadMax](#)
///
/// [Command::CALModifyBits](#)
///
/// [Command::PGMPrgClear](#) [Command::PGMPrg](#) [Command::PGMPrgNext](#) [Command::PGMPrgMax](#)
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct SetMeta {
    #[getset(skip)]
    reserved: u16,
    pub(crate) address_extension: u8,
    pub(crate) address: u32,
}

impl SetMeta {
    pub fn new(address_extension: u8, address: u32) -> Self {
        Self { reserved: Default::default(), address_extension, address }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for SetMeta {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.extend(self.reserved.to_be_bytes());
        result.push(self.address_extension);
        result.extend(self.address.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for SetMeta {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 2; // skip reserved
        let address_extension = data[offset];
        offset += 1;
        let address = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);

        Ok(Self::new(address_extension, address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_meta() -> anyhow::Result<()> {
        let request = SetMeta::new(0x01, 0x12345678);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x00, 0x00, 0x01, 0x12, 0x34, 0x56, 0x78]);

        let request = SetMeta::try_from(data.as_slice())?;
        assert_eq!(request.address_extension, 0x01);
        assert_eq!(request.address, 0x12345678);

        Ok(())
    }
}
