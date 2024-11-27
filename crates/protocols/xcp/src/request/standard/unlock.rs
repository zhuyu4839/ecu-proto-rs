use getset::{CopyGetters, Getters};
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct Unlock {
    #[get_copy = "pub"]
    pub(crate) remain_length: u8,
    #[get = "pub"]
    pub(crate) key: Vec<u8>,
}

impl Unlock {
    pub fn new(remain_length: u8, key: Vec<u8>) -> Self {
        if key.is_empty() {
            log::warn!("empty key");
            Self { remain_length: 0, key }
        }
        else {
            Self { remain_length, key }
        }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for Unlock {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.remain_length, ];
        let key_len = self.key.len();
        if key_len > self.remain_length as usize {
            log::warn!("key length rather than remain length");
            result.extend(&self.key[..self.remain_length as usize]);
        }
        else {
            result.append(&mut self.key);
        }

        result
    }
}

impl TryFrom<&[u8]> for Unlock {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len } );
        }

        let mut offset = 0;
        let length = data[offset];
        offset += 1;
        let mut next = data_len;
        let expected = offset + (length as usize);
        if data_len > expected {
            next = expected;
        }

        Ok(Self::new(length, data[offset..next].to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlock() -> anyhow::Result<()> {
        let request = Unlock::new(0x0A, vec![0x98, 0x76, 0x54, 0x32, 0x10, 0x01]);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x0A, 0x98, 0x76, 0x54, 0x32, 0x10, 0x01]);

        let request = Unlock::try_from(data.as_slice())?;
        assert_eq!(request.remain_length, 0x0A);
        assert_eq!(request.key, vec![0x98, 0x76, 0x54, 0x32, 0x10, 0x01]);

        let request = Unlock::new(0x04, vec![0x23, 0x45, 0x67, 0x89]);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x04, 0x23, 0x45, 0x67, 0x89]);

        let source = vec![0x04, 0x23, 0x45, 0x67, 0x89, 0x00, 0x00];
        let request = Unlock::try_from(source.as_slice())?;
        assert_eq!(request.remain_length, 0x04);
        assert_eq!(request.key, vec![0x23, 0x45, 0x67, 0x89]);

        Ok(())
    }
}
