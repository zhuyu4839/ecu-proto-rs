use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SeedMode {
    FirstPart = 0x00,
    RemainingPart = 0x01,
    Undefined(u8),
}

impl Into<u8> for SeedMode {
    fn into(self) -> u8 {
        match self {
            Self::FirstPart => 0x00,
            Self::RemainingPart => 0x01,
            Self::Undefined(x) => x,
        }
    }
}

impl From<u8> for SeedMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::FirstPart,
            0x01 => Self::RemainingPart,
            _ => Self::Undefined(value),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSeed {
    pub(crate) mode: SeedMode,
    pub(crate) resource: u8,    // Do not care when mode = GetSeedMode::RemainingPart
}

impl GetSeed {
    pub fn new(mode: SeedMode, resource: u8) -> Self {
        Self { mode, resource }
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetSeed {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.mode.into());
        result.push(self.resource);

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
        let mode = SeedMode::from(data[offset]);
        offset += 1;
        let resource = data[offset];

        Ok(Self::new(mode, resource))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seed() -> anyhow::Result<()> {
        let request = GetSeed::new(SeedMode::FirstPart, 1);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x00, 0x01]);

        let request = GetSeed::try_from(data.as_slice())?;
        assert_eq!(request.mode, SeedMode::FirstPart);
        assert_eq!(request.resource, 1);

        Ok(())
    }
}
