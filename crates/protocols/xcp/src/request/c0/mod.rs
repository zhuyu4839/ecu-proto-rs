use getset::{CopyGetters, Getters};
use crate::{DAQPackedMode, DAQPackedModeData, XcpError};

mod get_daq_packed_mode;
pub use get_daq_packed_mode::*;
mod set_daq_packed_mode;
pub use set_daq_packed_mode::*;

/// level 1 command code
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CmdCode {
    GetVersion = 0x00,
    SetDAQPackedMode = 0x01,
    GetDAQPackedMode = 0x02,
    SwDbgOverXCP = 0xFC,
    PodBS = 0xFD,
    Undefined(u8),
}

impl Into<u8> for CmdCode {
    fn into(self) -> u8 {
        match self {
            Self::GetVersion => 0x00,
            Self::SetDAQPackedMode => 0x01,
            Self::GetDAQPackedMode => 0x02,
            Self::SwDbgOverXCP => 0xFC,
            Self::PodBS => 0xFD,
            Self::Undefined(v) => v,
        }
    }
}

impl From<u8> for CmdCode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::GetVersion,
            0x01 => Self::SetDAQPackedMode,
            0x02 => Self::GetDAQPackedMode,
            0xFC => Self::SwDbgOverXCP,
            0xFD => Self::PodBS,
            _ => Self::Undefined(byte)
        }
    }
}

#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct CmdC0 {
    #[get_copy = "pub"]
    pub(crate) cmd_code: CmdCode,
    #[get = "pub"]
    pub(crate) data: Vec<u8>,
}

impl CmdC0 {
    #[inline(always)]
    pub const fn length() -> usize {
        1
    }

    #[inline(always)]
    pub fn get_version() -> Self {
        Self { cmd_code: CmdCode::GetVersion, data: vec![] }
    }

    #[inline(always)]
    pub fn set_daq_packed_mode(
        daq_list_number: u16,
        daq_packed_mode: DAQPackedMode,
        data: Option<DAQPackedModeData>
    ) -> Result<Self, XcpError> {
        let data = SetDAQPackedMode::new(
            daq_list_number,
            daq_packed_mode,
            data,
        )?;
        Ok(Self { cmd_code: CmdCode::SetDAQPackedMode, data: data.into() })
    }

    #[inline(always)]
    pub fn get_daq_packed_mode(daq_list_number: u16) -> Self {
        let data = GetDAQPackedMode::new(daq_list_number);
        Self { cmd_code: CmdCode::GetDAQPackedMode, data: data.into() }
    }

    #[inline(always)]
    pub fn sw_dbg_over_xcp() -> Self {
        Self { cmd_code: CmdCode::SwDbgOverXCP, data: vec![] }
    }

    #[inline(always)]
    pub fn pod_bs() -> Self {
        Self { cmd_code: CmdCode::PodBS, data: vec![] }
    }
}

impl Into<Vec<u8>> for CmdC0 {
    #[inline(always)]
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.cmd_code.into(), ];
        result.append(&mut self.data);

        result
    }
}

impl TryFrom<&[u8]> for CmdC0 {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let cmd_code = CmdCode::from(data[offset]);
        offset += 1;
        match cmd_code {
            CmdCode::GetVersion => {
                if data_len < expected {
                    return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                }

                Ok(Self::get_version())
            },
            CmdCode::SetDAQPackedMode => {
                let data = SetDAQPackedMode::try_from(&data[offset..])?;
                Ok(Self { cmd_code: CmdCode::SetDAQPackedMode, data: data.into() })
            },
            CmdCode::GetDAQPackedMode => {
                let data = GetDAQPackedMode::try_from(&data[offset..])?;
                Ok(Self { cmd_code: CmdCode::GetDAQPackedMode, data: data.into() })
            },
            CmdCode::SwDbgOverXCP => Ok(Self::sw_dbg_over_xcp()),
            CmdCode::PodBS => Ok(Self::pod_bs()),
            CmdCode::Undefined(c) => {
                log::warn!("Undefined cmd: {}", c);
                Err(XcpError::UndefinedError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DPMTimestampMode;
    use super::*;

    #[test]
    fn test_get_version() -> anyhow::Result<()> {
        let request = CmdC0::get_version();
        let data: Vec<_> = request.into();
        assert_eq!(data, [0x00, ]);

        let request = CmdC0::try_from(data.as_slice())?;
        assert_eq!(request.cmd_code, CmdCode::GetVersion);

        Ok(())
    }

    #[test]
    fn test_set_daq_packed_mode() -> anyhow::Result<()> {
        let request = CmdC0::set_daq_packed_mode(
            0x10,
            DAQPackedMode::NotPacked,
            None
        )?;
        let data: Vec<_> = request.into();
        assert_eq!(data, [0x01, 0x00, 0x10, 0x00]);

        let request = CmdC0::try_from(data.as_slice())?;
        assert_eq!(request.cmd_code, CmdCode::SetDAQPackedMode);
        let data = request.data;
        assert_eq!(data, [0x00, 0x10, 0x00]);

        let request = CmdC0::set_daq_packed_mode(
            0x10,
            DAQPackedMode::ElementGroup,
            Some(DAQPackedModeData::new(DPMTimestampMode::SingleTimestampOfFirstSample, 0x0010))
        )?;
        let data: Vec<_> = request.into();
        assert_eq!(data, [0x01, 0x00, 0x10, 0x01, 0x01, 0x00, 0x10]);

        let request =  CmdC0::try_from(data.as_slice())?;
        assert_eq!(request.cmd_code, CmdCode::SetDAQPackedMode);
        let data = request.data;
        assert_eq!(data, [0x00, 0x10, 0x01, 0x01, 0x00, 0x10]);

        Ok(())
    }
}
