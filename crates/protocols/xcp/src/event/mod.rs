//! 7.7 DESCRIPTION OF EVENTS in page 260

mod resume_mode;
pub use resume_mode::*;
mod time_synch;
pub use time_synch::*;
mod stim_timeout;
pub use stim_timeout::*;
mod ecu_state_change;
pub use ecu_state_change::*;
use crate::XcpError;

#[derive(Debug, Clone)]
pub enum Event {
    ResumeMode(ResumeMode),
    ClearDAQ,
    StoreDAQ,
    StoreCAL,
    CmdPending,
    DAQOverload,
    SessionTerminated,
    TimeSync(TimeSynch),
    STIMTimeout(STIMTimeout),
    Sleep,
    Wakeup,
    EcuStateChange(EcuStateChange),
    UserDefine,
    Transport,
}

impl Into<Vec<u8>> for Event {
    fn into(self) -> Vec<u8> {
        let mut result = vec![0xFD, ];
        match self {
            Self::ResumeMode(v) => {
                result.push(EventCode::ResumeMode.into());
                result.append(&mut v.into());
            }
            Self::ClearDAQ => result.push(EventCode::ClearDAQ.into()),
            Self::StoreDAQ => result.push(EventCode::StoreDAQ.into()),
            Self::StoreCAL => result.push(EventCode::StoreCAL.into()),
            Self::CmdPending => result.push(EventCode::CmdPending.into()),
            Self::DAQOverload => result.push(EventCode::DAQOverload.into()),
            Self::SessionTerminated => result.push(EventCode::SessionTerminated.into()),
            Self::TimeSync(v) => {
                result.push(EventCode::TimeSync.into());
                result.append(&mut v.into());
            }
            Self::STIMTimeout(v) => {
                result.push(EventCode::STIMTimeout.into());
                result.append(&mut v.into());
            }
            Self::Sleep => result.push(EventCode::Sleep.into()),
            Self::Wakeup => result.push(EventCode::Wakeup.into()),
            Self::EcuStateChange(v) => {
                result.push(EventCode::EcuStateChange.into());
                result.append(&mut v.into());
            }
            Self::UserDefine => result.push(EventCode::UserDefine.into()),
            Self::Transport => result.push(EventCode::Transport.into()),
        }

        result
    }
}

impl TryFrom<&[u8]> for Event {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = 1;
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let code = EventCode::from(data[offset]);
        offset += 1;
        match code {
            EventCode::ResumeMode => {
                let rm = ResumeMode::try_from(&data[offset..])?;
                Ok(Self::ResumeMode(rm))
            }
            EventCode::ClearDAQ => Ok(Self::ClearDAQ),
            EventCode::StoreDAQ => Ok(Self::StoreDAQ),
            EventCode::StoreCAL => Ok(Self::StoreCAL),
            EventCode::CmdPending => Ok(Self::CmdPending),
            EventCode::DAQOverload => Ok(Self::DAQOverload),
            EventCode::SessionTerminated => Ok(Self::SessionTerminated),
            EventCode::TimeSync => {
                let ts = TimeSynch::try_from(&data[offset..])?;
                Ok(Self::TimeSync(ts))
            }
            EventCode::STIMTimeout => {
                let st = STIMTimeout::try_from(&data[offset..])?;
                Ok(Self::STIMTimeout(st))
            }
            EventCode::Sleep => Ok(Self::Sleep),
            EventCode::Wakeup => Ok(Self::Wakeup),
            EventCode::EcuStateChange => {
                let esc = EcuStateChange::try_from(&data[offset..])?;
                Ok(Self::EcuStateChange(esc))
            }
            EventCode::Undefined(_) => Err(XcpError::UndefinedError),
            EventCode::UserDefine => Ok(Self::UserDefine),
            EventCode::Transport => Ok(Self::Transport),
        }
    }
}

/// | Even | Code | Description | Severity |
/// | ---- | ---- | ----------- | -------- |
/// | EV_RESUME_MODE | 0x00 | Slave starting in RESUME mode | S0 |
/// | EV_CLEAR_DAQ | 0x01 | The DAQ configuration in nonvolatile memory has been cleared. | S0 |
/// | EV_STORE_DAQ | 0x02 | The DAQ configuration has been stored into non-volatile memory. | S0 |
/// | EV_STORE_CAL | 0x03 | The calibration data has been stored into non-volatile memory. | S0 |
/// | EV_CMD_PENDING | 0x05 | Slave requesting to restart timeout. | S1 |
/// | EV_DAQ_OVERLOAD | 0x06 | DAQ processor overload. | S1 |
/// | EV_SESSION_TERMINATED | 0x07 | Session terminated by slave device. | S3 |
/// | EV_TIME_SYNC | 0x08 | Transfer of externally triggered timestamp | S0 |
/// | EV_STIM_TIMEOUT | 0x09 | Indication of a STIM timeout | S0-S3 |
/// | EV_SLEEP | 0x0A | Slave entering SLEEP mode | S1 |
/// | EV_WAKE_UP | 0x0B | Slave leaving SLEEP mode | S1 |
/// | EV_USER | 0xFE | SUser-defined event | S0 |
/// | EV_TRANSPORT | 0xFF | Transport layer specific event | Ref. Part3 |
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EventCode {
    ResumeMode = 0x00,  //
    ClearDAQ = 0x01,
    StoreDAQ = 0x02,
    StoreCAL = 0x03,
    CmdPending = 0x05,
    DAQOverload = 0x06,
    SessionTerminated = 0x07,
    TimeSync = 0x08,
    STIMTimeout = 0x09,
    Sleep = 0x0A,
    Wakeup = 0x0B,
    EcuStateChange = 0x0C,
    Undefined(u8),
    UserDefine = 0xFE,
    Transport = 0xFF,
}

impl Into<u8> for EventCode {
    fn into(self) -> u8 {
        match self {
            Self::ResumeMode => 0x00,
            Self::ClearDAQ => 0x01,
            Self::StoreDAQ => 0x02,
            Self::StoreCAL => 0x03,
            Self::CmdPending => 0x05,
            Self::DAQOverload => 0x06,
            Self::SessionTerminated => 0x07,
            Self::TimeSync => 0x08,
            Self::STIMTimeout => 0x09,
            Self::Sleep => 0x0A,
            Self::Wakeup => 0x0B,
            Self::EcuStateChange => 0x0C,
            Self::UserDefine => 0xFE,
            Self::Transport => 0xFF,
            Self::Undefined(n) => n,
        }
    }
}

impl From<u8> for EventCode {
    fn from(n: u8) -> Self {
        match n {
            0x00 => Self::ResumeMode,
            0x01 => Self::ClearDAQ,
            0x02 => Self::StoreDAQ,
            0x03 => Self::StoreCAL,
            0x05 => Self::CmdPending,
            0x06 => Self::DAQOverload,
            0x07 => Self::SessionTerminated,
            0x08 => Self::TimeSync,
            0x09 => Self::STIMTimeout,
            0x0A => Self::Sleep,
            0x0B => Self::Wakeup,
            0x0C => Self::EcuStateChange,
            0xFE => Self::UserDefine,
            0xFF => Self::Transport,
            _ => Self::Undefined(n),
        }
    }
}
