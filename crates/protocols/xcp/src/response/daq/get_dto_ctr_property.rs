use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::{DTOCTRPropertyMode, XcpError};

/// Bitfield representation of 8-bit `PROPERTIES parameter bit mask in GET_DTO_CTR_PROPERTIES`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | EVT_CTR_PRESENT         | 1           |
/// | STIM_CTR_CPY_PRESENT    | 1           |
/// | STIM_MODE_PRESENT       | 1           |
/// | DAQ_MODE_PRESENT        | 1           |
/// | RELATED_EVENT_PRESENT   | 1           |
/// | STIM_MODE_FIXED         | 1           |
/// | DAQ_MODE_FIXED          | 1           |
/// | RELATED_EVENT_FIXED     | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DTOCTRProperty {
    pub evt_ctr_present: bool,
    pub stim_ctr_copy_present: bool,
    pub stim_mode_present: bool,
    pub daq_mode_present: bool,
    pub related_event_present: bool,
    pub stim_mode_fixed: bool,
    pub daq_mode_fixed: bool,
    pub related_event_fixed: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDTOCTRProperty {
    pub(crate) property: DTOCTRProperty,
    pub(crate) related_event_channel_number: u16,
    pub(crate) mode: DTOCTRPropertyMode,
}

impl GetDTOCTRProperty {

    pub const fn new(
        property: DTOCTRProperty,
        related_event_channel_number: u16,
        mode: DTOCTRPropertyMode,
    ) -> Self {
        Self {
            property,
            related_event_channel_number,
            mode,
        }
    }

    pub const fn length() -> usize {
        4
    }
}

impl Into<Vec<u8>> for GetDTOCTRProperty {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.property.into());
        result.extend(self.related_event_channel_number.to_be_bytes());
        result.push(self.mode.into());

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
        let property = DTOCTRProperty::from(data[offset]);
        offset += 1;
        let related_event_channel_number = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let mode = DTOCTRPropertyMode::from(data[offset]);

        Ok(Self::new(property, related_event_channel_number, mode))
    }
}
