use bitfield_struct::bitfield;

/// Bitfield representation of 8-bit `MODE parameter bit mask in GET_DTO_CTR_PROPERTIES`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Reserved                | 6           |
/// | STIM_MODE               | 1           |
/// | DAQ_MODE                | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DTOCTRPropertyMode {
    #[bits(6)]
    __: u8,
    /// DTO CTR STIM mode:
    /// When receiving DTOs with CTR field:
    /// 0 = DO_NOT_CHECK_COUNTER
    /// 1 = CHECK_COUNTER
    pub stim_mode: bool,
    /// DTO CTR DAQ mode:
    /// When inserting the DTO CTR field:
    /// 0 = INSERT_COUNTER - use CTR of the related
    /// event
    /// 1 = INSERT_STIM_COUNTER_COPY - use STIM
    /// CTR copy of the related event
    pub daq_mode: bool,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum IdentificationFieldType {
    AbsoluteODTNumber = 0x00,
    RelativeODTNumberAbsoluteDAQListNumberByte = 0x01,
    RelativeODTNumberAbsoluteDAQListNumberWord = 0x02,
    RelativeODTNumberAbsoluteDAQListNumberWordAlign = 0x03,
}

impl IdentificationFieldType {
    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

impl Into<u8> for IdentificationFieldType {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for IdentificationFieldType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::AbsoluteODTNumber,
            0x01 => Self::RelativeODTNumberAbsoluteDAQListNumberByte,
            0x02 => Self::RelativeODTNumberAbsoluteDAQListNumberWord,
            0x03 => Self::RelativeODTNumberAbsoluteDAQListNumberWordAlign,
            _ => Self::RelativeODTNumberAbsoluteDAQListNumberWordAlign,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AddressExtensionType {
    DifferentWithinOneSameODT = 0x00,
    SameForAllEntryWithOneODT = 0x01,
    NotAllowed = 0x02,
    SameForAllEntryWithOneDAQ = 0x03,
}

impl AddressExtensionType {
    #[inline]
    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

impl Into<u8> for AddressExtensionType {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for AddressExtensionType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::DifferentWithinOneSameODT,
            0x01 => Self::SameForAllEntryWithOneODT,
            0x02 => Self::NotAllowed,
            0x03 => Self::SameForAllEntryWithOneDAQ,
            _ => Self::SameForAllEntryWithOneDAQ,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OptimisationType {
    #[default]
    Default = 0x00,
    ODT16 = 0x01,
    ODT32 = 0x02,
    ODT64 = 0x03,
    ODTAlignment = 0x04,
    MaxEntrySize = 0x05,
    ODT16Strict = 0x09,
    ODT32Strict = 0x0A,
    ODT64Strict = 0x0B,
}

impl OptimisationType {
    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

impl Into<u8> for OptimisationType {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for OptimisationType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Default,
            0x01 => Self::ODT16,
            0x02 => Self::ODT32,
            0x03 => Self::ODT64,
            0x04 => Self::ODTAlignment,
            0x05 => Self::MaxEntrySize,
            0x09 => Self::ODT16Strict,
            0x0A => Self::ODT32Strict,
            0x0B => Self::ODT64Strict,
            _ => Self::ODT64Strict,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OverloadIndicationType {
    NoOverload = 0x00,
    OverloadInMSBOfPID = 0x01,
    OverloadByEventPacked = 0x02,
    NotAllowed = 0x03,
}

impl OverloadIndicationType {
    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

impl Into<u8> for OverloadIndicationType {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for OverloadIndicationType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NoOverload,
            0x01 => Self::OverloadInMSBOfPID,
            0x02 => Self::OverloadByEventPacked,
            0x03 => Self::NotAllowed,
            _ => Self::NotAllowed,
        }
    }
}
