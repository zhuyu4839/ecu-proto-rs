//! page 110

use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConnectMode {
    #[default]
    Normal = 0x00,
    UserDefined = 0x01,
    Undefined(u8),
}

impl Into<u8> for ConnectMode {
    fn into(self) -> u8 {
        match self {
            Self::Normal => 0x00,
            Self::UserDefined => 0x01,
            Self::Undefined(v) => v,
        }
    }
}

impl From<u8> for ConnectMode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::Normal,
            0x01 => Self::UserDefined,
            _ => Self::Undefined(byte),
        }
    }
}

#[derive(Debug, Default, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct Connect {
    pub(crate) mode: ConnectMode
}

impl Connect {
    pub fn new(mode: ConnectMode) -> Self {
        Self { mode }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for Connect {
    fn into(self) -> Vec<u8> {
        vec![self.mode.into(), ]
    }
}

impl TryFrom<&[u8]> for Connect {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mode: ConnectMode = data[0].into();
        Ok(Self::new(mode))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() -> anyhow::Result<()> {
        let connect = Connect::default();
        let data: Vec<u8> = connect.into();
        assert_eq!(data, hex::decode("00")?);

        let connect = Connect::try_from(data.as_slice())?;
        assert_eq!(connect.mode, ConnectMode::Normal);

        Ok(())
    }
}
