use getset::{CopyGetters, Getters};
use crate::XcpError;

#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct GetSeed {
    #[get_copy = "pub"]
    pub(crate) remain_length: u8,
    #[get = "pub"]
    pub(crate) seed: Vec<u8>,
}

impl GetSeed {
    pub fn new(remain_length: u8, seed: Vec<u8>) -> Self {
        if seed.is_empty() {
            // if seed is empty, the resource is unprotected and no UNLOCK command is necessary
            log::warn!("empty seed");

            Self { remain_length: 0, seed }
        }
        else {
            Self { remain_length, seed }
        }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for GetSeed {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.remain_length, ];
        let seed_len = self.seed.len();
        if seed_len > self.remain_length as usize {
            log::warn!("seed length rather than remain length");
            result.extend(&self.seed[..self.remain_length as usize]);
        }
        else {
            result.append(&mut self.seed);
        }

        result
    }
}

impl TryFrom<&[u8]> for GetSeed {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
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
    fn test_get_seed() -> anyhow::Result<()> {
        let response = GetSeed::new(0x10, vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);

        let response = GetSeed::try_from(data.as_slice())?;
        assert_eq!(response.remain_length, 0x10);
        assert_eq!(response.seed, vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);

        let response = GetSeed::new(0x0A, vec![0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x0A, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);

        let response = GetSeed::try_from(data.as_slice())?;
        assert_eq!(response.remain_length, 0x0A);
        assert_eq!(response.seed, vec![0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);

        let response = GetSeed::new(0x04, vec![0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x00]);
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x04, 0x0C, 0x0D, 0x0E, 0x0F]);

        let source = vec![0x04, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x00];
        let response = GetSeed::try_from(source.as_slice())?;
        assert_eq!(response.remain_length, 0x04);
        assert_eq!(response.seed, vec![0x0C, 0x0D, 0x0E, 0x0F]);

        Ok(())
    }
}
