use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// Bitfield representation of 8-bit `DAQ_PROPERTIES parameter in GET_DAQ_PROCESSOR_INFO`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | OVERLOAD_EVENT          | 1           |
/// | OVERLOAD_MSB            | 1           |
/// | PID_OFF_SUPPORTED       | 1           |
/// | TIMESTAMP_SUPPORTED     | 1           |
/// | BIT_STIM_SUPPORTED      | 1           |
/// | RESUME_SUPPORTED        | 1           |
/// | PRESCALER_SUPPORTED     | 1           |
/// | DAQ_CONFIG_TYPE         | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DAQProperty {
    /// OverloadIndicationType
    #[bits(2)]
    pub overload_indication_type: u8,
    pub pid_off_support: bool,
    pub timestamp_support: bool,
    pub bit_stim_support: bool,
    pub resume_support: bool,
    pub prescaler_support: bool,
    pub daq_cfg_type: bool,
}

/// Bitfield representation of 8-bit `DAQ_KEY_BYTE parameter in GET_DAQ_PROCESSOR_INFO`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | IdentificationFieldType | 2           |
/// | Address_Extension_DAQ   | 1           |
/// | Address_Extension_ODT   | 1           |
/// | Optimisation_Type       | 4           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct DAQKeyByte {
    /// IdentificationFieldType
    #[bits(2)]
    pub id_field_type: u8,
    /// AddressExtensionType
    #[bits(2)]
    pub address_extension: u8,
    /// OptimisationType
    #[bits(4)]
    pub optim_type: u8,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDaqProcessorInfo {
    /// DAQ_PROPERTIES
    /// General properties of DAQ lists
    pub(crate) property: DAQProperty,
    /// MAX_DAQ
    /// Total number of available DAQ lists
    pub(crate) available_daq_list: u16,
    /// MAX_EVENT_CHANNEL
    /// Total number of available event channels
    pub(crate) available_event_channel: u16,
    /// MIN_DAQ
    /// Total number of predefined DAQ lists
    pub(crate) predefine_daq_list: u8,
    /// DAQ_KEY_BYTE
    pub(crate) daq_key_byte: DAQKeyByte,
}

impl GetDaqProcessorInfo {
    pub fn new(
        property: DAQProperty,
        available_daq_list: u16,
        available_event_channel: u16,
        predefine_daq_list: u8,
        daq_key_byte: DAQKeyByte,
    ) -> Self {
        Self {
            property,
            available_daq_list,
            available_event_channel,
            predefine_daq_list,
            daq_key_byte
        }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for GetDaqProcessorInfo {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.property.into());
        result.extend(self.available_daq_list.to_be_bytes());
        result.extend(self.available_event_channel.to_be_bytes());
        result.push(self.predefine_daq_list);
        result.push(self.daq_key_byte.into());

        result
    }
}

impl TryFrom<&[u8]> for GetDaqProcessorInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let property = DAQProperty::from(data[offset]);
        offset += 1;
        let available_daq_list = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let available_event_channel = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let predefine_daq_list = data[offset];
        offset += 1;
        let daq_key_byte = DAQKeyByte::from(data[offset]);

        Ok(Self::new(property, available_daq_list, available_event_channel, predefine_daq_list, daq_key_byte))
    }
}
