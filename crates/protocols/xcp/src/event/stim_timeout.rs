use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum STIMTimeoutInfoType {
    EventChannelNumber = 0x00,
    DAQListNumber = 0x01,
    Undefined(u8),
}

impl Into<u8> for STIMTimeoutInfoType {
    fn into(self) -> u8 {
        match self {
            Self::EventChannelNumber => 0x00,
            Self::DAQListNumber => 0x01,
            Self::Undefined(x) => x,
        }
    }
}

impl From<u8> for STIMTimeoutInfoType {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::EventChannelNumber,
            0x01 => Self::DAQListNumber,
            _ => Self::Undefined(v),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum STIMTimeoutFailureType {
    Timeout = 0x00,
    DTOCTRCheckFailed = 0x01,
    /// 2 .. 127 = reserved
    Undefined(u8),
    /// 128 .. 255 = user defined
    UserDefined(u8),
}

impl Into<u8> for STIMTimeoutFailureType {
    fn into(self) -> u8 {
        match self {
            Self::Timeout => 0x00,
            Self::DTOCTRCheckFailed => 0x01,
            Self::Undefined(x) => x,
            Self::UserDefined(x) => x,
        }
    }
}

impl From<u8> for STIMTimeoutFailureType {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::Timeout,
            0x01 => Self::DTOCTRCheckFailed,
            0x02..=0x7F => Self::Undefined(v),
            _ => Self::UserDefined(v),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct STIMTimeout {
    pub(crate) info_type: STIMTimeoutInfoType,
    pub(crate) failure_type: STIMTimeoutFailureType,
    /// Event channel number or DAQ list number depending on Info Type.
    pub(crate) number: u16,
}

impl STIMTimeout {
    pub fn new(
        info_type: STIMTimeoutInfoType,
        failure_type: STIMTimeoutFailureType,
        number: u16
    ) -> Result<Self, XcpError> {
        match info_type {
            STIMTimeoutInfoType::Undefined(_)  => Err(XcpError::UndefinedError),
            _ => Ok(())
        }?;

        match failure_type {
            STIMTimeoutFailureType::Undefined(_)  => Err(XcpError::UndefinedError),
            _ => Ok(())
        }?;

        Ok(Self { info_type, failure_type, number })
    }

    pub const fn length() -> usize {
        4
    }
}

impl Into<Vec<u8>> for STIMTimeout {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.info_type.into(), self.failure_type.into()];
        result.extend(self.number.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for STIMTimeout {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength {  expected, actual: data_len });
        }

        let mut offset = 0;
        let info_type = STIMTimeoutInfoType::from(data[offset]);
        offset += 1;
        let failure_type = STIMTimeoutFailureType::from(data[offset]);
        offset += 1;
        let number = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Self::new(info_type, failure_type, number)
    }
}
