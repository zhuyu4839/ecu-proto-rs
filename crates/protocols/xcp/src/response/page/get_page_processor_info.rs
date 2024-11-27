use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// Bitfield representation of 1-bit `PAG_PROPERTIES parameter in GET_PAG_PROCESSOR_INFO`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Reserved                | 7           |
/// | FREEZE_SUPPORTED        | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct ProcessorInfoProperty {
    #[bits(7)]
    __: u8,
    pub freeze_support: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetPageProcessorInfo {
    /// MAX_SEGMENTS
    /// total number of available segments
    pub(crate) segments: u8,
    pub(crate) property: ProcessorInfoProperty,
}

impl GetPageProcessorInfo {
    pub fn new(segments: u8, property: ProcessorInfoProperty) -> Self {
        Self { segments, property }
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for GetPageProcessorInfo {
    fn into(self) -> Vec<u8> {
        vec![self.segments, self.property.into(),]
    }
}

impl TryFrom<&[u8]> for GetPageProcessorInfo {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let segments = data[offset];
        offset += 1;
        let property = ProcessorInfoProperty::from(data[offset]);

        Ok(Self::new(segments, property))
    }
}
