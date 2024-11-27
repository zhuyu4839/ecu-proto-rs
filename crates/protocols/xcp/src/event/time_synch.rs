//! page 266

use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::{PayloadFormat, SyncState, XcpError};

/// only 2bits
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TimestampSampling {
    /// during command processing at the protocol layer
    /// command processor
    DuringCommandProcessing = 0x00,
    /// low jitter, measured in high-priority interrupt
    LowJitter = 0x01,
    /// upon physical transmission to XCP master
    UponTransmission = 0x02,
    /// upon physical reception of command
    UponReception = 0x03,
}

impl Into<u8> for TimestampSampling {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for TimestampSampling {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::DuringCommandProcessing,
            0x01 => Self::LowJitter,
            0x02 => Self::UponTransmission,
            0x03 => Self::UponReception,
            _ => Self::UponReception,
        }
    }
}

/// only 3bits
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TriggerInitiator {
    /// HW trigger, i.e. Vector Syncline
    Hardware = 0x00,
    /// Event derived from XCP-independent time
    /// synchronization event – e.g. globally synchronized
    /// pulse per second signal
    TimeSyncEvent = 0x01,
    /// GET_DAQ_CLOCK_MULTICAST
    DAQClockMultiCast = 0x02,
    /// GET_DAQ_CLOCK_MULTICAST via Time Sync Bridge
    DAQClockMultiCastViaTimeSyncBridge = 0x03,
    /// State change in syntonization/synchronization to
    /// grandmaster clock (either established or lost,
    /// additional information is provided by the SYNC_STATE
    /// field - see Table 248)
    StateChangeToGrandmaster = 0x04,
    /// Leap second occurred on grandmaster clock
    LeapSecondOnGrandmaster = 0x05,
    /// release of ECU reset
    ReleaseEcuReset = 0x06,
    Reserved = 0x07,
}

impl Into<u8> for TriggerInitiator {
    fn into(self) -> u8 {
        self as _
    }
}

impl From<u8> for TriggerInitiator {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::Hardware,
            0x01 => Self::TimeSyncEvent,
            0x02 => Self::DAQClockMultiCast,
            0x03 => Self::DAQClockMultiCastViaTimeSyncBridge,
            0x04 => Self::StateChangeToGrandmaster,
            0x05 => Self::LeapSecondOnGrandmaster,
            0x06 => Self::ReleaseEcuReset,
            0x07 => Self::Reserved,
            _ => Self::Reserved,
        }
    }
}

#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct TriggerInfo {
    #[bits(3)]
    __: u8,
    #[bits(2)]
    pub ts_sampling: u8,
    #[bits(3)]
    pub trigger_initiator: u8,
}

/// Table 242 EV_TIME_SYNC event packet, legacy format
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct TimeSynchLegacy {
    /// `Only used when extended format for MAX_CTO = 8`
    /// otherwise reserved
    pub(crate) trigger_info: TriggerInfo,
    /// When event is sent as a response to TRIGGER_INITIATOR 2,
    /// the Counter value of the GET_DAQ_CLOCK_MULTICAST is copied here;
    /// otherwise 0.
    ///
    /// `Only used when extended format for MAX_CTO = 8`
    /// otherwise reserved
    pub(crate) counter: u8,
    /// Timestamp of clock that is related to DAQ timestamps
    pub(crate) timestamp: u32,
}

impl TimeSynchLegacy {
    pub fn new(
        trigger_info: Option<TriggerInfo>,
        counter: Option<u8>,
        timestamp: u32
    ) -> Self {
        Self {
            trigger_info: trigger_info.unwrap_or_default(),
            counter: counter.unwrap_or_default(),
            timestamp,
        }
    }

    pub const fn length() -> usize {
        6
    }
}

impl Into<Vec<u8>> for TimeSynchLegacy {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.trigger_info.into(), self.counter];
        result.extend(self.timestamp.to_be_bytes());
        result
    }
}

impl TryFrom<&[u8]> for TimeSynchLegacy {
    type Error = XcpError;
    
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength {  expected, actual: data_len });
        }

        let mut offset = 0;
        let trigger_info = TriggerInfo::from(data[offset]);
        offset += 1;
        let counter = data[offset];
        offset += 1;
        let timestamp = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());

        Ok(Self::new(Some(trigger_info), Some(counter), timestamp))
    }
}

/// Table 247 EV_TIME_SYNC event packet, extended format for MAX_CTO = 8
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct TimeSynchExtended {
    pub(crate) trigger_info: TriggerInfo,
    pub(crate) payload_format: PayloadFormat,
    /// If observable:
    /// timestamp of XCP slave’s clock (type depends on FMT_XCP_SLV)
    pub(crate) timestamp_slave: u64,
    /// If observable:
    /// timestamp of dedicated clock synchronized to grandmaster
    /// (type depends on FMT_GRANDM)
    pub(crate) timestamp_dedicated: u64,
    /// If observable:
    /// timestamp of ECU clock (type depends on FMT_ECU)
    pub(crate) timestamp_ecu: u64,
    /// If required (depends on CLUSTER_IDENTIFIER):
    /// when event is sent as a response to TRIGGER_INITIATOR 2 or 3,
    /// the Cluster Identifier is copied here.
    /// For more information see discussion of detailed description of
    /// TIME_SYNC_BRIDGE in chapter 7.5.6.1.
    pub(crate) cluster_id: u16,
    /// If required (depends on CLUSTER_IDENTIFIER):
    /// when event is sent as a response to TRIGGER_INITIATOR 2
    /// or 3, the Counter value of the GET_DAQ_CLOCK_MULTICAST
    /// is copied here.
    pub(crate) counter: u8,
    /// SYNC_STATE (see Table 217, Table 218)
    /// This field must always be sent if at least one of the observable
    /// clocks can be synchronized or syntonized to a grandmaster
    /// clock. If none of the observable clocks supports
    /// synchronization or syntonization, this field must not be sent.
    pub(crate) sync_state: SyncState,
}

impl TimeSynchExtended {
    pub fn new(
        trigger_info: TriggerInfo,
        payload_format: PayloadFormat,
        timestamp_slave: u64,
        timestamp_dedicated: u64,
        timestamp_ecu: u64,
        cluster_id: u16,
        counter: u8,
        sync_state: SyncState,
    ) -> Self {
        Self {
            trigger_info,
            payload_format,
            timestamp_slave,
            timestamp_dedicated,
            timestamp_ecu,
            cluster_id,
            counter,
            sync_state
        }
    }

    pub const fn length() -> usize {
        30
    }
}

impl Into<Vec<u8>> for TimeSynchExtended {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.trigger_info.into());
        result.push(self.payload_format.into());
        result.extend(self.timestamp_slave.to_be_bytes());
        result.extend(self.timestamp_dedicated.to_be_bytes());
        result.extend(self.timestamp_ecu.to_be_bytes());
        result.extend(self.cluster_id.to_be_bytes());
        result.push(self.counter);
        result.push(self.sync_state.into());

        result
    }
}

impl TryFrom<&[u8]> for TimeSynchExtended {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength {  expected, actual: data_len });
        }

        // let mut offset = 0;
        // let trigger_info = TriggerInfo::from(data[offset]);
        // offset += 1;
        // let payload_fmt = PayloadFormat::from(data[offset]);
        // offset += 1;
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum TimeSynch {
    Legacy(TimeSynchLegacy),
    Extended(TimeSynchExtended),
}

impl Into<Vec<u8>> for TimeSynch {
    fn into(self) -> Vec<u8> {
        match self {
            Self::Legacy(legacy) => legacy.into(),
            Self::Extended(extended) => extended.into(),
        }
    }
}

impl TryFrom<&[u8]> for TimeSynch {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let legacy_length = TimeSynchLegacy::length();
        let extended_length = TimeSynchExtended::length();

        if data_len == legacy_length {
            Ok(Self::Legacy(TimeSynchLegacy::try_from(data)?))
        }
        else if data_len == extended_length {
            Ok(Self::Extended(TimeSynchExtended::try_from(data)?))
        }
        else {
            Err(XcpError::InvalidDataLength {  expected: legacy_length, actual: data_len, })
        }
    }
}
