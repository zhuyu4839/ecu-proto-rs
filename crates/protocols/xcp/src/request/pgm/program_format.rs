use getset::CopyGetters;
use crate::{ProgrammingMethod, XcpError};

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CompressionMethod {
    #[default]
    Uncompressed,
    UserDefined(u8),
    Undefined(u8),
}

impl Into<u8> for CompressionMethod {
    fn into(self) -> u8 {
        match self {
            Self::Uncompressed => 0,
            Self::UserDefined(u) => u,
            Self::Undefined(u) => u,
        }
    }
}

impl From<u8> for CompressionMethod {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Uncompressed,
            0x80..=0xFF => Self::UserDefined(v),
            _ => Self::Undefined(v),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EncryptionMethod {
    #[default]
    Unencrypted,
    UserDefined(u8),
    Undefined(u8),
}

impl Into<u8> for EncryptionMethod {
    fn into(self) -> u8 {
        match self {
            Self::Unencrypted => 0,
            Self::UserDefined(u) => u,
            Self::Undefined(u) => u,
        }
    }
}

impl From<u8> for EncryptionMethod {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Unencrypted,
            0x80..=0xFF => Self::UserDefined(v),
            _ => Self::Undefined(v),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccessMode {
    /// The MTA uses physical addresses
    #[default]
    Absolute,
    /// The MTA functions as a block sequence number of the new flash content file.
    Functional,
    /// It is possible to use different access modes for clearing and programming.
    UserDefined(u8),
    Undefined(u8),
}

impl Into<u8> for AccessMode {
    fn into(self) -> u8 {
        match self {
            Self::Absolute => 0,
            Self::Functional => 1,
            Self::UserDefined(u) => u,
            Self::Undefined(u) => u,
        }
    }
}

impl From<u8> for AccessMode {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Absolute,
            1 => Self::Functional,
            0x80..=0xFF => Self::UserDefined(v),
            _ => Self::Undefined(v),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ProgramFormat {
    pub(crate) compression_method: CompressionMethod,
    pub(crate) encryption_method: EncryptionMethod,
    pub(crate) programming_method: ProgrammingMethod,
    pub(crate) access_mode: AccessMode,
}

impl ProgramFormat {
    pub fn new(
        compression_method: CompressionMethod,
        encryption_method: EncryptionMethod,
        programming_method: ProgrammingMethod,
        access_mode: AccessMode,
    ) -> Result<Self, XcpError> {
        match access_mode {
            AccessMode::Undefined(_) => Err(XcpError::UndefinedError),
            _ => Ok(())
        }?;

        Ok(Self {
            compression_method,
            encryption_method,
            programming_method,
            access_mode,
        })
    }

    pub const fn length() -> usize {
        4
    }
}

impl Into<Vec<u8>> for &ProgramFormat {
    fn into(self) -> Vec<u8> {
        vec![
            self.compression_method.into(),
            self.encryption_method.into(),
            self.programming_method.into(),
            self.access_mode.into(),
        ]
    }
}

impl TryFrom<&[u8]> for ProgramFormat {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let compression_method = CompressionMethod::from(data[offset]);
        offset += 1;
        let encryption_method = EncryptionMethod::from(data[offset]);
        offset += 1;
        let programming_method = ProgrammingMethod::from(data[offset]);
        offset += 1;
        let access_mode = AccessMode::from(data[offset]);

        Self::new(compression_method, encryption_method, programming_method, access_mode)
    }
}
