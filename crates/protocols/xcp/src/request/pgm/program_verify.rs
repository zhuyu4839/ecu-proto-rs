use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum VerificationMode {
    RequestStartRoutine = 0x00,
    SendingVerification = 0x01,
    Undefined(u8),
}

impl Into<u8> for VerificationMode {
    fn into(self) -> u8 {
        match self {
            Self::RequestStartRoutine => 0x00,
            Self::SendingVerification => 0x01,
            Self::Undefined(c) => c,
        }
    }
}

impl From<u8> for VerificationMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::RequestStartRoutine,
            0x01 => Self::SendingVerification,
            _ => Self::Undefined(value),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum VerificationType {
    CalibrationArea = 0x0001,
    CodeArea = 0x0002,
    CompleteFlash = 0x0004,
    // Reserved(u16),
    UserDefined(u16),
    Undefined(u16),
}

impl Into<u16> for VerificationType {
    fn into(self) -> u16 {
        match self {
            Self::CalibrationArea => 0x0001,
            Self::CodeArea => 0x0002,
            Self::CompleteFlash => 0x0004,
            // Self::Reserved(c) => c,
            Self::UserDefined(c) => c,
            Self::Undefined(c) => c,
        }
    }
}

impl From<u16> for VerificationType {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => Self::CalibrationArea,
            0x0002 => Self::CodeArea,
            0x0004 => Self::CompleteFlash,
            // 0x0008..=0x0080 => Self::Reserved(value),
            0x0100..=0xFF00 => Self::UserDefined(value),
            _ => Self::Undefined(value),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ProgramVerify {
    pub(crate) mode: VerificationMode,
    pub(crate) r#type: VerificationType,
    pub(crate) value: u32,
}

impl ProgramVerify {
    pub fn new(
        mode: VerificationMode,
        r#type: VerificationType,
        value: u32
    ) -> Result<Self, XcpError> {
        match mode {
            VerificationMode::Undefined(_) => Err(XcpError::UndefinedError),
            _ => Ok(())
        }?;

        match r#type {
            VerificationType::Undefined(_) => Err(XcpError::UndefinedError),
            _ => Ok(())
        }?;

        Ok(Self { mode, r#type, value })
    }

    pub const fn length() -> usize {
        6
    }
}

impl Into<Vec<u8>> for ProgramVerify {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.mode.into());
        let r#type: u16 = self.r#type.into();
        result.extend(r#type.to_be_bytes());
        result.extend(self.value.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for ProgramVerify {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = VerificationMode::from(data[offset]);
        offset += 1;
        let r#type = VerificationType::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;
        let value = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());

        Self::new(mode, r#type, value)
    }
}
