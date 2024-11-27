

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GetSectorInfoMode {
    /// get start address for this SECTOR
    StartAddress = 0x00,
    /// get length of this SECTOR [BYTE]
    Length = 0x01,
    /// get name length of this SECTOR
    NameOfLength = 0x02,
    Undefined(u8),
}

impl Into<u8> for GetSectorInfoMode {
    fn into(self) -> u8 {
        match self {
            Self::StartAddress => 0x00,
            Self::Length => 0x01,
            Self::NameOfLength => 0x02,
            Self::Undefined(c) => c,
        }
    }
}

impl From<u8> for GetSectorInfoMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::StartAddress,
            0x01 => Self::Length,
            0x02 => Self::NameOfLength,
            _ => Self::Undefined(value),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ProgrammingMethod {
    #[default]
    SequentialProgramming,
    UserDefined(u8),
    Undefined(u8),
}

impl Into<u8> for ProgrammingMethod {
    fn into(self) -> u8 {
        match self {
            Self::SequentialProgramming => 0,
            Self::UserDefined(u) => u,
            Self::Undefined(u) => u,
        }
    }
}

impl From<u8> for ProgrammingMethod {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::SequentialProgramming,
            0x80..=0xFF => Self::UserDefined(v),
            _ => Self::Undefined(v),
        }
    }
}
