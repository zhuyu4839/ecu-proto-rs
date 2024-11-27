//! page 177

use getset::CopyGetters;
use crate::{ClockFormat, PayloadFormat, SyncState, XcpError};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SlaveClockSyncState {
    /// XCP slave’s clock is in progress of synchronizing to a grandmaster clock
    Synchronizing = 0x00,
    /// XCP slave’s clock is synchronized to a grandmaster clock
    Synchronized = 0x01,
    /// XCP slave clock is in progress of syntonizing to a grandmaster clock
    Syntonizing = 0x02,
    /// XCP slave’s clock is syntonized to a grandmaster  clock
    Syntonized = 0x03,
    /// XCP slave’s clock does not support synchronization/syntonization to a grandmaster clock
    NotSupported = 0x07,
    /// 4, 5, 6 = Reserved
    Reserved(u8),
}

impl Into<u8> for SlaveClockSyncState {
    fn into(self) -> u8 {
        match self {
            Self::Synchronizing => 0x00,
            Self::Synchronized => 0x01,
            Self::Syntonizing => 0x02,
            Self::Syntonized => 0x03,
            Self::NotSupported => 0x07,
            Self::Reserved(val) => val,
        }
    }
}

impl From<u8> for SlaveClockSyncState {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Synchronizing,
            0x01 => Self::Synchronized,
            0x02 => Self::Syntonizing,
            0x03 => Self::Syntonized,
            0x07 => Self::NotSupported,
            _ => Self::Reserved(value),
        }
    }
}

/// GET DAQ CLOCK positive response structure, legacy format
/// GET DAQ CLOCK positive response structure, extended format
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetDAQClock {
    #[getset(skip)]
    reserved: u8,
    pub(crate) trigger_info: u8,
    /// PAYLOAD_FMT (see Table 245, Table 246)
    pub(crate) payload_fmt: PayloadFormat,
    /// Timestamp(DWORD/DLONG) of XCP slave’s clock (type depends on FMT_XCP_SLV), extended format
    pub(crate) timestamp: u64,
    /// If observable:
    /// timestamp of dedicated clock synchronized to grandmaster(type depends on FMT_GRANDM)
    pub(crate) dedicated_timestamp: Option<u64>,
    /// If observable:
    /// timestamp of ECU clock (type depends on FMT_ECU)
    pub(crate) ecu_timestamp: Option<u64>,
    /// SYNC_STATE (see Table 217, Table 218)
    ///
    /// This field must always be sent if at least one of the observable
    /// clocks can be synchronized or syntonized to a grandmaster clock.
    ///
    /// If none of the observable clocks supports synchronization or syntonization,
    /// this field must not be sent.
    pub(crate) synch_state: Option<SyncState>,
}

impl GetDAQClock {
    pub fn new(
        trigger_info: u8,
        payload_fmt: PayloadFormat,
        timestamp: u64,
        dedicated_timestamp: Option<u64>,
        ecu_timestamp: Option<u64>,
        synch_state: Option<SyncState>,
    ) -> Result<Self, XcpError> {
        let ecu_fmt = ClockFormat::from(payload_fmt.fmt_ecu());
        match ecu_fmt {
            ClockFormat::DWord
            | ClockFormat::DLong => {}
            _ => return Err(XcpError::UndefinedError),
        }

        let grandmaster_fmt = ClockFormat::from(payload_fmt.fmt_grandmaster());
        match grandmaster_fmt {
            ClockFormat::DWord
            | ClockFormat::DLong => {},
            _ => return Err(XcpError::UndefinedError),
        }

        let xcp_slave_fmt = ClockFormat::from(payload_fmt.fmt_xcp_slave());
        match xcp_slave_fmt {
            ClockFormat::DWord
            | ClockFormat::DLong => {},
            _ => return Err(XcpError::UndefinedError),
        }

        if dedicated_timestamp.is_some()
            || ecu_timestamp.is_some() {
            if synch_state.is_none() {
                return Err(XcpError::UnexpectInput("synch state is expected".into()));
            }
        }

        Ok(Self {
            reserved: Default::default(),
            trigger_info,
            payload_fmt,
            timestamp,
            dedicated_timestamp,
            ecu_timestamp,
            synch_state,
        })
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for GetDAQClock {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.reserved);
        result.push(self.trigger_info);
        result.push(self.payload_fmt.into());

        let payload_fmt = self.payload_fmt;
        match ClockFormat::from(payload_fmt.fmt_ecu()) {
            ClockFormat::DWord => {
                result.extend((self.timestamp as u32).to_be_bytes());
            },
            ClockFormat::DLong => {
                result.extend(self.timestamp.to_be_bytes());
            },
            _ => unreachable!(""),
        }

        if let Some(dedicated_timestamp) = self.dedicated_timestamp {
            match ClockFormat::from(payload_fmt.fmt_grandmaster()) {
                ClockFormat::DWord => {
                    result.extend((dedicated_timestamp as u32).to_be_bytes());
                },
                ClockFormat::DLong => {
                    result.extend(dedicated_timestamp.to_be_bytes());
                },
                _ => unreachable!(""),
            }
        }
        
        if let Some(ecu_timestamp) = self.ecu_timestamp {
            match ClockFormat::from(payload_fmt.fmt_xcp_slave()) {
                ClockFormat::DWord => {
                    result.extend((ecu_timestamp as u32).to_be_bytes());
                },
                ClockFormat::DLong => {
                    result.extend(ecu_timestamp.to_be_bytes());
                },
                _ => unreachable!(""),
            }
        }

        if let Some(sync_state) = self.synch_state {
            result.push(sync_state.into())
        }

        result
    }
}

impl TryFrom<&[u8]> for GetDAQClock {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 1;
        let trigger_info = data[offset];
        offset += 1;
        let payload_fmt = PayloadFormat::from(data[offset]);
        let timestamp = match ClockFormat::from(payload_fmt.fmt_ecu()) {
            ClockFormat::DWord => {
                let timestamp = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap());
                offset += 4;
                Ok(timestamp as u64)
            },
            ClockFormat::DLong => {
                let expected = offset + 8;
                if data_len < expected {
                    return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                }

                let timestamp = u64::from_be_bytes(data[offset..offset+8].try_into().unwrap());
                offset += 8;
                Ok(timestamp)
            },
            _ => Err(XcpError::UndefinedError)
        }?;

        let dedicated_timestamp = if data_len > offset {
            match ClockFormat::from(payload_fmt.fmt_grandmaster()) {
                ClockFormat::DWord => {
                    let expected = offset + 4;
                    if data_len < expected {
                        return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                    }

                    let timestamp = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap());
                    offset += 4;
                    Ok(Some(timestamp as u64))
                },
                ClockFormat::DLong => {
                    let expected = offset + 8;
                    if data_len < expected {
                        return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                    }

                    let timestamp = u64::from_be_bytes(data[offset..offset+8].try_into().unwrap());
                    offset += 8;
                    Ok(Some(timestamp))
                },
                _ => Err(XcpError::UndefinedError)
            }
        }
        else {
            Ok(None)
        }?;

        let (ecu_timestamp, synch_state) = match dedicated_timestamp {
            Some(_) => if data_len > offset {
                match ClockFormat::from(payload_fmt.fmt_xcp_slave()) {
                    ClockFormat::DWord => {
                        let expected = offset + 5;
                        if data_len < expected {
                            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                        }

                        let timestamp = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap());
                        offset += 4;
                        Ok((Some(timestamp as u64), Some(SyncState::from(data[offset]))))
                    },
                    ClockFormat::DLong => {
                        let expected = offset + 9;
                        if data_len < expected {
                            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                        }

                        let timestamp = u64::from_be_bytes(data[offset..offset+8].try_into().unwrap());
                        offset += 8;
                        Ok((Some(timestamp), Some(SyncState::from(data[offset]))))
                    },
                    _ => Err(XcpError::UndefinedError)
                }
            }
            else {
                let expected = offset + 1;
                if data_len < expected {
                    return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                }

                Ok((None, Some(SyncState::from(data[offset]))))
            },
            None => Ok((None, None))
        }?;

        Ok(Self::new(trigger_info, payload_fmt, timestamp, dedicated_timestamp, ecu_timestamp, synch_state)?)
    }
}
