use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DAQListType {
    NotAllowed = 0x00,
    /// only DAQ direction supported
    DAQOnly = 0x01,
    /// only STIM direction supported
    STIMOnly = 0x02,
    /// both directions supported (but not simultaneously)
    Both = 0x03,
}

impl Into<u8> for DAQListType {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for DAQListType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotAllowed,
            0x01 => Self::DAQOnly,
            0x02 => Self::STIMOnly,
            0x03 => Self::Both,
            _ => Self::NotAllowed,
        }
    }   
}

/// Bitfield representation of 4-bit `DAQ_LIST_PROPERTIES parameter in GET_DAQ_LIST_INFO`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Reserved                | 4           |
/// | STIM                    | 1           |
/// | DAQ                     | 1           |
/// | EVENT_FIXED             | 1           |
/// | PREDEFINED              | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DAQListProperty {
    #[bits(3)]
    __: u8,
    pub packed: bool,
    /// DAQListType
    #[bits(2)]
    pub daq_list_type: u8,
    pub event_fixed: bool,
    pub predefined: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDAQListInfo {
    /// DAQ_LIST_PROPERTIES
    /// Specific properties for this DAQ list
    pub(crate) property: DAQListProperty,
    /// MAX_ODT
    /// Number of ODTs in this DAQ list
    pub(crate) max_odt: u8,
    /// MAX_ODT_ENTRIES
    /// Maximum number of entries in an ODT
    pub(crate) max_odt_entry: u8,
    /// FIXED_EVENT
    /// Number of the fixed event channel for this DAQ list
    pub(crate) fixed_event: u16,
}
impl GetDAQListInfo {
    pub fn new(
        property: DAQListProperty,
        max_odt: u8,
        max_odt_entry: u8,
        fixed_event: u16,
    ) -> Self {
        Self { property, max_odt, max_odt_entry, fixed_event }
    }

    pub const fn length() -> usize {
        5
    }
}

impl Into<Vec<u8>> for GetDAQListInfo {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.property.into());
        result.push(self.max_odt);
        result.push(self.max_odt_entry);
        result.extend(self.fixed_event.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for GetDAQListInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let property = DAQListProperty::from(data[offset]);
        offset += 1;
        let max_odt = data[offset];
        offset += 1;
        let max_odt_entry = data[offset];
        offset += 1;
        let fixed_event = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(property, max_odt, max_odt_entry, fixed_event))
    }
}
