use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnlockStatus {
    Completed = 0x00,
    Uncompleted = 0x01,
    Undefined(u8),
}

impl Into<u8> for UnlockStatus {
    fn into(self) -> u8 {
        match self {
            Self::Completed => 0x00,
            Self::Uncompleted => 0x01,
            Self::Undefined(x) => x,
        }
    }
}

impl From<u8> for UnlockStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Completed,
            0x01 => Self::Uncompleted,
            _ => Self::Undefined(value),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct Unlock {
    pub(crate) status: UnlockStatus,  // 0x01 not completed, 0x00 completed
}

impl Unlock {
    pub fn new(status: UnlockStatus) -> Self {
        Self { status }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for Unlock {
    #[inline(always)]
    fn into(self) -> Vec<u8> {
        vec![self.status.into(), ]
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

        let status = UnlockStatus::from(data[0]);

        Ok(Self::new(status))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seed() -> anyhow::Result<()> {
        let response = Unlock::new(UnlockStatus::Uncompleted);
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x01]);
        let response = Unlock::try_from(data.as_slice())?;
        assert_eq!(response.status, UnlockStatus::Uncompleted);

        let response = Unlock::new(UnlockStatus::Completed);
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x00]);
        let response = Unlock::try_from(data.as_slice())?;
        assert_eq!(response.status, UnlockStatus::Completed);

        Ok(())
    }
}

