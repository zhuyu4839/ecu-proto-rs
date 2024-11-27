use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ShortUpload {
    /// n = Number of data elements [AG] [1..MAX_CTO/AG -1]
    pub(crate) size: u8,
    #[getset(skip)]
    reserved: u8,
    pub(crate) address_extension: u8,
    pub(crate) address: u32,
}

impl ShortUpload {
    pub fn new(size: u8, address_extension: u8, address: u32) -> Self {
        Self { size, reserved: Default::default(), address_extension, address }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for ShortUpload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.size);
        result.push(self.reserved);
        result.push(self.address_extension);
        result.extend(self.address.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for ShortUpload {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let size = data[offset];
        offset += 1;
        offset += 1;    // skip reserved
        let address_extension = data[offset];
        offset += 1;
        let address = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());

        Ok(Self::new(size, address_extension, address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_upload() -> anyhow::Result<()> {
        let request = ShortUpload::new(0x10, 0x01, 0x12345678);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x10, 0x00, 0x01, 0x12, 0x34, 0x56, 0x78]);

        let request = ShortUpload::try_from(data.as_slice())?;
        assert_eq!(request.size, 0x10);
        assert_eq!(request.address_extension, 0x01);
        assert_eq!(request.address, 0x12345678);

        Ok(())
    }
}
