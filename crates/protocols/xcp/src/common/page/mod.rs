use bitfield_struct::bitfield;

/// Bitfield representation of 1-bit `Mode parameter in SET_SEGMENT_MODE or GET_SEGMENT_MODE`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Reserved                | 7           |
/// | FREEZE_SUPPORTED        | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct SegmentMode {
    #[bits(7)]
    __: u8,
    pub freeze: bool,
}

/// Bitfield representation of 3-bit `Mode parameter in SET_CAL_PAGE`
///
/// ### Repr: `u8`
///
/// | Field                        | Size (bits) |
/// |------------------------------|-------------|
/// | All                          | 1           |
/// | Reserved                     | 5           |
/// | XCP                          | 1           |
/// | ECU                          | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct CalPageMode {
    pub all: bool,
    #[bits(5)]
    __: u8,
    pub xcp: bool,
    pub ecu: bool,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SegmentInfoMode {
    BasicAddress = 0x00,
    Standard = 0x01,
    AddressMapping = 0x03,
    Undefined(u8),
}

impl Into<u8> for SegmentInfoMode {
    fn into(self) -> u8 {
        match self {
            Self::BasicAddress => 0x00,
            Self::Standard => 0x01,
            Self::AddressMapping => 0x03,
            Self::Undefined(n) => n,
        }
    }
}

impl From<u8> for SegmentInfoMode {
    fn from(n: u8) -> Self {
        match n {
            0x00 => Self::BasicAddress,
            0x01 => Self::Standard,
            0x03 => Self::AddressMapping,
            _ => Self::Undefined(n),
        }
    }
}
