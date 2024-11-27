use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ProgramClearMode {
    /// | Parameter   | Description |
    /// |-------------|-------------|
    /// | MTA         | The MTA points to the start of a memory sector inside the slave. |
    /// | MTA         | Memory sectors are described in the ASAM MCD-2 MC slave device |
    /// | MTA         | description file. |
    /// | MTA         | If multiple memory sectors shall be cleared in a certain sequence, the |
    /// | MTA         | master device must repeat the PROGRAM_CLEAR service with a new MTA. |
    /// | MTA         | In this case the master must keep the order information given by the |
    /// | MTA         | Clear Sequence Number of the sectors. |
    /// | Clear range | The Clear Range indicates the length of the memory part to be cleared. |
    /// | Clear range | The PROGRAM_CLEAR service clears a complete sector or multiple |
    /// | Clear range | sectors at once. |
    #[default]
    AbsoluteAccess = 0x00,
    /// | Parameter   | Description |
    /// |-------------|-------------|
    /// | MTA         | The MTA has no influence on the clearing functionality |
    /// | Clear range | This parameter should be interpreted bit after bit: |
    /// | Clear range | basic use-cases: |
    /// | Clear range | 0x00000001 : clear all the calibration data area(s) |
    /// | Clear range | 0x00000002 : clear all the code area(s) (the boot area is not covered) |
    /// | Clear range | 0x00000004 : clear the NVRAM area(s) |
    /// | Clear range | 0x00000008 .. 0x00000080: reserved |
    /// | Clear range | project specific use-cases: |
    /// | Clear range | 0x00000100 ... 0xFFFFFF00 : user-defined |
    FunctionalAccess = 0x01,
    Undefined(u8),
}

impl Into<u8> for ProgramClearMode {
    fn into(self) -> u8 {
        match self {
            Self::AbsoluteAccess => 0x00,
            Self::FunctionalAccess => 0x01,
            Self::Undefined(c) => c,
        }
    }
}

impl From<u8> for ProgramClearMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::AbsoluteAccess,
            0x01 => Self::FunctionalAccess,
            _ => Self::Undefined(value),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ProgramClear {
    pub(crate) mode: ProgramClearMode,
    #[getset(skip)]
    reserved: u16,
    pub(crate) clear_range: u32,
}

impl ProgramClear {
    pub fn new(mode: ProgramClearMode, clear_range: u32) -> Result<Self, XcpError> {
        match mode {
            ProgramClearMode::AbsoluteAccess
            | ProgramClearMode::FunctionalAccess => Ok(()),
            _ => Err(XcpError::UndefinedError),
        }?;

        Ok(Self {
            mode,
            reserved: Default::default(),
            clear_range,
        })
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for ProgramClear {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.mode.into());
        result.extend(self.reserved.to_be_bytes());
        result.extend(self.clear_range.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for ProgramClear {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = ProgramClearMode::from(data[offset]);
        offset += 1;
        offset += 2;    // skip reserved
        let clear_range = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());

        Self::new(mode, clear_range)
    }
}
