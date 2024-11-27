
#[derive(Debug, Clone, thiserror::Error)]
pub enum XcpError {
    #[error("XCP - unknown command: {0:02X}")]
    UnknownCommand(u8),

    #[error("XCP - invalid data length at least {expected} bytes, actual: {actual}")]
    InvalidDataLength { expected: usize, actual: usize },

    #[error("XCP - missing data, {expected} bytes, actual: {actual}")]
    MissData { expected: usize, actual: usize },

    #[error("XCP - invalid ECU access mode")]
    InvalidECUAccessMode,

    #[error("XCP - can't complete because of undefined branch")]
    UndefinedError,

    #[error("XCP - unexpected input: {0}")]
    UnexpectInput(String),
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ErrorCode {
    CmdSynch = 0x00,

    CmdBusy = 0x10,
    DAQActive = 0x11,
    PGMActive = 0x12,

    CmdUnknown = 0x20,
    CmdSyntax = 0x21,
    OutOfRange = 0x22,
    WriteProtected = 0x23,
    AccessDenied = 0x24,
    AccessLocked = 0x25,
    PageNotValid = 0x26,
    ModeNotValid = 0x27,
    SegmentNotValid = 0x28,
    SequenceError = 0x29,
    DAQCfgError = 0x2A,

    MemOverflow = 0x30,
    GenericError = 0x31,
    VerifyError = 0x32,
    ResourceTemporaryNotAccessible = 0x33,
    SubCmdUnknown = 0x34,
    TimeCorrStateChange = 0x35,

    DebugError = 0xFC,

    Undefined(u8),
}

impl Into<u8> for ErrorCode {
    fn into(self) -> u8 {
        match self {
            Self::CmdSynch => 0x00,
            Self::CmdBusy => 0x10,
            Self::DAQActive => 0x11,
            Self::PGMActive => 0x12,
            Self::CmdUnknown => 0x20,
            Self::CmdSyntax => 0x21,
            Self::OutOfRange => 0x22,
            Self::WriteProtected => 0x23,
            Self::AccessDenied => 0x24,
            Self::AccessLocked => 0x25,
            Self::PageNotValid => 0x26,
            Self::ModeNotValid => 0x27,
            Self::SegmentNotValid => 0x28,
            Self::SequenceError => 0x29,
            Self::DAQCfgError => 0x2A,
            Self::MemOverflow => 0x30,
            Self::GenericError => 0x31,
            Self::VerifyError => 0x32,
            Self::ResourceTemporaryNotAccessible => 0x33,
            Self::SubCmdUnknown => 0x34,
            Self::TimeCorrStateChange => 0x35,
            Self::DebugError => 0xFC,
            Self::Undefined(v) => v,
        }
    }
}

impl From<u8> for ErrorCode {
    fn from(code: u8) -> Self {
        match code {
            0x00 => Self::CmdSynch,

            0x10 => Self::CmdBusy,
            0x11 => Self::DAQActive,
            0x12 => Self::PGMActive,

            0x20 => Self::CmdUnknown,
            0x21 => Self::CmdSyntax,
            0x22 => Self::OutOfRange,
            0x23 => Self::WriteProtected,
            0x24 => Self::AccessDenied,
            0x25 => Self::AccessLocked,
            0x26 => Self::PageNotValid,
            0x27 => Self::ModeNotValid,
            0x28 => Self::SegmentNotValid,
            0x29 => Self::SequenceError,
            0x2A => Self::DAQCfgError,

            0x30 => Self::MemOverflow,
            0x31 => Self::GenericError,
            0x32 => Self::VerifyError,
            0x33 => Self::ResourceTemporaryNotAccessible,
            0x34 => Self::SubCmdUnknown,
            0x35 => Self::TimeCorrStateChange,

            0xFC => Self::DebugError,
            _ => Self::Undefined(code),
        }
    }
}
