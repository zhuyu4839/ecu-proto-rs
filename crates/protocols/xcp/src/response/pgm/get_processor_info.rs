use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// only 2bits
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ClearProgrammingMode {
    NotAllowed = 0x00,
    Absolute = 0x01,
    Functional = 0x02,
    Both = 0x03,
}

impl Into<u8> for ClearProgrammingMode {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for ClearProgrammingMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotAllowed,
            0x01 => Self::Absolute,
            0x02 => Self::Functional,
            0x03 => Self::Both,
            _ => Self::Both,
        }
    }
}

/// only 2bits
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ProgrammingCompression {
    NotSupported = 0x00,
    Supported = 0x01,
    NotCare = 0x02,
    SupportedAndRequired = 0x03,
}

impl Into<u8> for ProgrammingCompression {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for ProgrammingCompression {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotSupported,
            0x01 => Self::Supported,
            0x02 => Self::NotCare,
            0x03 => Self::SupportedAndRequired,
            _ => Self::NotCare,
        }
    }
}

/// only 2bits
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ProgrammingEncryption {
    NotSupported = 0x00,
    Supported = 0x01,
    NotCare = 0x02,
    SupportedAndRequired = 0x03,
}

impl Into<u8> for ProgrammingEncryption {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for ProgrammingEncryption {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotSupported,
            0x01 => Self::Supported,
            0x02 => Self::NotCare,
            0x03 => Self::SupportedAndRequired,
            _ => Self::NotCare,
        }
    }
}

/// only 2bits
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ProgrammingNonSequential {
    NotSupported = 0x00,
    Supported = 0x01,
    NotCare = 0x02,
    SupportedAndRequired = 0x03,
}

impl Into<u8> for ProgrammingNonSequential {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for ProgrammingNonSequential {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotSupported,
            0x01 => Self::Supported,
            0x02 => Self::NotCare,
            0x03 => Self::SupportedAndRequired,
            _ => Self::NotCare,
        }
    }
}

/// Bitfield representation of 8-bit `PGM_PROPERTIES parameter in GET_PGM_PROCESSOR_INFO`
///
/// ### Repr: `u8`
///
/// | Field                        | Size (bits) |
/// |------------------------------|-------------|
/// | NON_SEQ_PGM_REQUIRED         | 1           |
/// | NON_SEQ_PGM_SUPPORTED        | 1           |
/// | ENCRYPTION_REQUIRED          | 1           |
/// | ENCRYPTION_SUPPORTED         | 1           |
/// | COMPRESSION_REQUIRED         | 1           |
/// | COMPRESSION_SUPPORTED        | 1           |
/// | FUNCTIONAL_MODE              | 1           |
/// | ABSOLUTE_MODE                | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct ProgramProperty {
    /// ProgrammingNonSequential
    #[bits(2)]
    pub non_seq_pgm: u8,
    /// ProgrammingEncryption
    #[bits(2)]
    pub encryption: u8,
    /// ProgrammingCompression
    #[bits(2)]
    pub compression: u8,
    /// ClearProgrammingMode
    #[bits(2)]
    pub clear_mode: u8,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetProcessorInfo {
    /// PGM_PROPERTIES
    /// General properties for programming
    pub(crate) property: ProgramProperty,
    /// MAX_SECTOR
    /// total number of available sectors
    pub(crate) max_sector: u8,
}

impl GetProcessorInfo {
    pub fn new(property: ProgramProperty, max_sector: u8) -> Self {
        Self { property, max_sector }
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetProcessorInfo {
    fn into(self) -> Vec<u8> {
        vec![self.property.into(), self.max_sector]
    }
}

impl TryFrom<&[u8]> for GetProcessorInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let property = ProgramProperty::from(data[offset]);
        offset += 1;
        let max_sector = data[offset];

        Ok(Self::new(property, max_sector))
    }
}
