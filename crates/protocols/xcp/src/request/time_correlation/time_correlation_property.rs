//! page 223

use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimeSyncBridge {
    DoNotChangeValue = 0x00,
    /// Enable time synchronization bridging functionality
    EnableTimeSyncBridge = 0x01,
    /// Disable time synchronization bridging functionality
    DisableTimeSyncBridge = 0x02,
    Reserved = 0x03,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResponseFormat {
    DoNotChangeValue = 0x00,
    /// Send EV_TIME_SYNC event packet as response to
    /// TRIGGER_INITIATOR 0, 2 and 3 only. Sending
    /// EV_TIME_SYNC event packet for other
    /// TRIGGER_INITIATOR values is not allowed (see Table 244).
    SendEventTimeSyncForPartTrigger = 0x01,
    /// Send EV_TIME_SYNC event packet for all trigger conditions
    SendEventTimeSyncForAllTrigger = 0x02,
    Reserved = 0x03,
}

/// Bitfield representation of 8-bit `SET_PROPERTIES parameter bit mask structure in TIME_CORRELATION_PROPERTIES`
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
pub struct TimeCorrelationSetProperty {
    #[bits(3)]
    __: u8,
    /// 0 = Do not change value
    /// 1 = Assign XCP slave to the logical time correlation cluster
    /// given by CLUSTER_ID parameter
    pub set_cluster_id: bool,
    /// TimeSyncBridge
    #[bits(2)]
    pub time_sync_bridge: u8,
    /// ResponseFormat
    #[bits(2)]
    pub response_fmt: u8,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct TimeCorrelationProperty {
    pub(crate) set_property: TimeCorrelationSetProperty,
    pub(crate) get_property_request: u8,
    #[getset(skip)]
    reserved: u8,
    pub(crate) cluster_id: u16,
}

impl TimeCorrelationProperty {
    pub fn new(
        set_property: TimeCorrelationSetProperty,
        get_property_request: u8,
        cluster_id: u16,
    ) -> Self {
        Self {
            set_property,
            get_property_request,
            reserved: Default::default(),
            cluster_id,
        }
    }

    pub const fn length() -> usize {
        5
    }
}

impl Into<Vec<u8>> for TimeCorrelationProperty {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.set_property.into(), self.get_property_request, self.reserved];
        result.extend(self.cluster_id.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for TimeCorrelationProperty {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let set_property = TimeCorrelationSetProperty::from(data[offset]);
        offset += 1;
        let property_request = data[offset];
        offset += 1;
        offset += 1;    // skip reserved
        let cluster_id = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(set_property, property_request, cluster_id))
    }
}
