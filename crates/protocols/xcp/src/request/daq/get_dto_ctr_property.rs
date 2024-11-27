use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::{DTOCTRPropertyMode, XcpError};

/// Bitfield representation of 8-bit `MODIFIER parameter bit mask in GET_DTO_CTR_PROPERTIES`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Reserved                | 5           |
/// | MODIFY_STIM_MODE        | 1           |
/// | MODIFY_DAQ_MODE         | 1           |
/// | MODIFY_RELATED_EVENT    | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DTOCTRPropertyModifier {
    #[bits(5)]
    __: u8,
    pub modify_stim_mode: bool,
    pub modify_daq_mode: bool,
    pub modify_related_event: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDTOCTRProperty {
    pub(crate) modifier: DTOCTRPropertyModifier,
    pub(crate) event_channel_number: u16,
    pub(crate) related_event_channel_number: u16,
    pub(crate) mode: DTOCTRPropertyMode,
}

impl GetDTOCTRProperty {
    pub fn new(
        modifier: DTOCTRPropertyModifier,
        event_channel_number: u16,
        related_event_channel_number: u16,
        mode: DTOCTRPropertyMode,
    ) -> Self {
        Self {
            modifier,
            event_channel_number,
            related_event_channel_number,
            mode,
        }
    }

    pub const fn length() -> usize {
        6
    }
}

impl Into<Vec<u8>> for GetDTOCTRProperty {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.modifier.into());
        result.extend(self.event_channel_number.to_be_bytes());
        result.extend(self.related_event_channel_number.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for GetDTOCTRProperty {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let modifier = DTOCTRPropertyModifier::from(data[offset]);
        offset += 1;
        let event_channel_number = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let related_event_channel_number = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let mode = DTOCTRPropertyMode::from(data[offset]);

        Ok(Self::new(modifier, event_channel_number, related_event_channel_number, mode))
    }
}
