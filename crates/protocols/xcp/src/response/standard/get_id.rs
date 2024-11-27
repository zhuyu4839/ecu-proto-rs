use bitfield_struct::bitfield;
use getset::{CopyGetters, Getters};
use crate::XcpError;

/// Bitfield representation of 2-bit `Mode parameter in GET_ID`
///
/// ### Repr: `u8`
///
/// | Field                | Size (bits) |
/// |----------------------|-------------|
/// | Reserved             | 6           |
/// | Compressed Encrypted | 1           |
/// | Transfer Mode        | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct GetIdMode {
    #[bits(6)]
    __: u8,
    pub compressed_encrypted: bool,
    pub transfer_mode: bool,
}

#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct GetId {
    #[getset(get_copy = "pub")]
    pub(crate) mode: GetIdMode,
    #[getset(skip)]
    reserved: u16,
    // length 4 byte
    #[getset(get = "pub")]
    pub(crate) data: Vec<u8>,
}

impl GetId {
    pub fn new(mode: GetIdMode, data: Vec<u8>) -> Self {
       Self { mode, reserved: Default::default(), data }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for GetId {
    fn into(mut self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.mode.into());
        result.extend(self.reserved.to_be_bytes());
        let len = self.data.len() as u32;
        result.extend(len.to_be_bytes());
        result.append(&mut self.data);

        result
    }
}

impl TryFrom<&[u8]> for GetId {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = GetIdMode::from(data[offset]);
        offset += 1;
        offset += 2;    // skip reserved data
        let len = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
        offset += 4;
        let expected = offset + len;
        if data_len < expected {
            return Err(XcpError::MissData { expected, actual: data_len });
        }

        let data = data[offset..].to_vec();

        Ok(Self::new(mode, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id() -> anyhow::Result<()> {
        let binding = String::from("abcdef/abc");
        let src = binding.as_bytes();
        let response = GetId::new(
            GetIdMode::new()
                .with_transfer_mode(true),
            src.to_vec(),
        );
        let data: Vec<_> = response.into();

        let mut expect = vec![0x01, 0x00, 0x00];
        expect.extend((src.len() as u32).to_be_bytes());
        expect.extend(src);
        assert_eq!(data, expect);

        let response = GetId::try_from(data.as_slice())?;
        assert_eq!(response.mode, GetIdMode::new().with_transfer_mode(true));
        // assert_eq!(response.data.len(), src.len());
        assert_eq!(response.data, src);

        Ok(())
    }
}
