use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// Bitfield representation of 8-bit `TIMESTAMP_MODE parameter in GET_DAQ_RESOLUTION_INFO`
///
/// ### Repr: `u8`
///
/// | Field                   | Size (bits) |
/// |-------------------------|-------------|
/// | Unit                    | 4           |
/// | TIMESTAMP_FIXED         | 1           |
/// | Size                    | 3           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct TimestampMode {
    /// TimestampUnit
    #[bits(4)]
    pub unit: u8,
    pub fixed: bool,
    /// 0x00 - no timestamp
    ///
    /// 0x01 - 1 byes
    ///
    /// 0x02 - 2 bytes
    ///
    /// 0x03 - not allowed
    ///
    /// 0x04 - 4 bytes
    #[bits(3)]
    pub size: u8,
}

/// The possible values for GRANULARITY_ODT_ENTRY_SIZE_x are {1,2,4,8}.
///
/// For the address of the element described by an ODT entry, the following has to be fulfilled:
/// Address[AG] mod (GRANULARITY_ODT_ENTRY_SIZE_x[BYTE] / AG[BYTE]) = 0
///
/// For every size of the element described by an ODT entry, the following has to be fulfilled:
/// SizeOf(element described by ODT entry)[AG] mod
/// (GRANULARITY_ODT_ENTRY_SIZE_x[BYTE] / AG[BYTE]) = 0
///
/// The MAX_ODT_ENTRY_SIZE_x parameters indicate the upper limits for the size of the
/// element described by an ODT entry.
///
/// For every size of the element described by an ODT entry the following has to be fulfilled:
/// SizeOf(element described by ODT entry) <= MAX_ODT_ENTRY_SIZE_x
///
/// If the slave does not support a time-stamped mode (no TIMESTAMP_SUPPORTED in
/// GET_DAQ_PROCESSOR_INFO), the parameters TIMESTAMP_MODE and
/// TIMESTAMP_TICKS are invalid.
/// If the slave device supports a time-stamped mode, TIMESTAMP_MODE and
/// TIMESTAMP_TICKS contain information on the resolution of the data acquisition clock.
/// The data acquisition clock is a free running counter, which is never reset or modified and
/// wraps around if an overflow occurs.
///
/// $$
/// t_{\text{physical}}^{k+1} = t_{\text{physical}}^k + \left[ \left( t_{\text{protocol}}^{k+1} - t_{\text{protocol}}^k \right) \cdot \text{TIMESTAMP\_UNIT} \cdot \text{TIMESTAMP\_TICKS} \right]
/// $$
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDaqResolutionInfo {
    /// GRANULARITY_ODT_ENTRY_SIZE_DAQ
    /// Granularity for size of ODT entry (DAQ direction)
    pub(crate) daq_odt_entry_size: u8,
    /// MAX_ODT_ENTRY_SIZE_DAQ
    /// Maximum size of ODT entry (DAQ direction)
    pub(crate) daq_odt_max_size: u8,
    /// GRANULARITY_ODT_ENTRY_SIZE_STIM
    /// Granularity for size of ODT entry (STIM direction)
    pub(crate) stim_odt_entry_size: u8,
    /// MAX_ODT_ENTRY_SIZE_STIM
    /// Maximum size of ODT entry (STIM direction)
    pub(crate) stim_odt_max_size: u8,
    /// TIMESTAMP_MODE
    /// Timestamp unit and size
    pub(crate) timestamp_mode: TimestampMode,
    /// TIMESTAMP_TICKS
    /// Timestamp ticks per unit
    pub(crate) timestamp_ticks: u16,
}

impl GetDaqResolutionInfo {
    pub fn new(
        daq_odt_entry_size: u8,
        daq_odt_max_size: u8,
        stim_odt_entry_size: u8,
        stim_odt_max_size: u8,
        timestamp_mode: TimestampMode,
        timestamp_ticks: u16,
    ) -> Self {
        Self {
            daq_odt_entry_size,
            daq_odt_max_size,
            stim_odt_entry_size,
            stim_odt_max_size,
            timestamp_mode,
            timestamp_ticks
        }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for GetDaqResolutionInfo {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.daq_odt_entry_size);
        result.push(self.daq_odt_max_size);
        result.push(self.stim_odt_entry_size);
        result.push(self.stim_odt_max_size);
        result.push(self.timestamp_mode.into());
        result.extend(self.timestamp_ticks.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for GetDaqResolutionInfo {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let daq_odt_entry_size = data[offset];
        offset += 1;
        let daq_odt_max_size = data[offset];
        offset += 1;
        let stim_odt_entry_size = data[offset];
        offset += 1;
        let stim_odt_max_size = data[offset];
        offset += 1;
        let timestamp_mode = TimestampMode::from(data[offset]);
        offset += 1;
        let timestamp_ticks = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(
            daq_odt_entry_size,
            daq_odt_max_size,
            stim_odt_entry_size,
            stim_odt_max_size,
            timestamp_mode,
            timestamp_ticks,
        ))
    }
}
