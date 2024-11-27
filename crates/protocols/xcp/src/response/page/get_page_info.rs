use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EcuAccessType {
    NotAllowed = 0x00,
    WithoutECU = 0x01,
    WithECU = 0x02,
    NotCare = 0x03,
}

impl EcuAccessType {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub fn from_bits(bits: u8) -> Self {
        Self::from(bits)
    }
}

impl Into<u8> for EcuAccessType {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for EcuAccessType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotAllowed,
            0x01 => Self::WithoutECU,
            0x02 => Self::WithECU,
            _ => Self::NotCare,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum XCPWriteAccessType {
    NotAllowed = 0x00,
    WithoutECU = 0x01,
    WithECU = 0x02,
    NotCare = 0x03,
}

impl Into<u8> for XCPWriteAccessType {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for XCPWriteAccessType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotAllowed,
            0x01 => Self::WithoutECU,
            0x02 => Self::WithECU,
            _ => Self::NotCare,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum XCPReadAccessType {
    NotAllowed = 0x00,
    WithoutECU = 0x01,
    WithECU = 0x02,
    NotCare = 0x03,
}

impl Into<u8> for XCPReadAccessType {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for XCPReadAccessType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotAllowed,
            0x01 => Self::WithoutECU,
            0x02 => Self::WithECU,
            _ => Self::NotCare,
        }
    }
}

/// Bitfield representation of 6-bit `PAGE_PROPERTIES parameter in GET_PAGE_INFO`
///
/// ### Repr: `u8`
///
/// | Field                        | Size (bits) |
/// |------------------------------|-------------|
/// | Reserved                     | 2           |
/// | XCP_WRITE_ACCESS_WITH_ECU    | 1           |
/// | XCP_WRITE_ACCESS_WITHOUT_ECU | 1           |
/// | XCP_READ_ACCESS_WITH_ECU     | 1           |
/// | XCP_READ_ACCESS_WITHOUT_ECU  | 1           |
/// | ECU_ACCESS_WITH_XCP          | 1           |
/// | ECU_ACCESS_WITHOUT_XCP       | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct PageProperty {
    #[bits(2)]
    __: u8,
    #[bits(2)]
    pub xcp_write_access_type: u8,  // XCPWriteAccessType
    #[bits(2)]
    pub xcp_read_access_type: u8,   // XCPReadAccessType
    #[bits(2)]
    pub ecu_access_type: u8,        // ECUAccessType
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetPageInfo {
    pub(crate) property: PageProperty,
    pub(crate) init_segment: u8,
}

impl GetPageInfo {
    pub fn new(property: PageProperty, init_segment: u8) -> Self {
        Self { property, init_segment }
    }
    
    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetPageInfo {
    fn into(self) -> Vec<u8> {
        vec![self.property.into(), self.init_segment]
    }
}

impl TryFrom<&[u8]> for GetPageInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let property = PageProperty::from(data[offset]);
        offset += 1;
        let init_segment = data[offset];

        Ok(Self::new(property, init_segment))
    }
}
