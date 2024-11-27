use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::{ResourceStatus, XcpError};

/// Bitfield representation of 7-bit `Current Session Status parameter in GET_STATUS`
///
/// ### Repr: `u8`
///
/// | Field                | Size (bits) |
/// |----------------------|-------------|
/// | Resume               | 1           |
/// | DAQ Running          | 1           |
/// | Reserved             | 1           |
/// | DAQ_CFG_LOST         | 1           |
/// | ClearDAQReq          | 1           |
/// | StoreDAQReq          | 1           |
/// | CAL_PAG_CFG_LOST     | 1           |
/// | StoreCALReq          | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct SessionStatus {
    pub resume: bool,
    pub daq_running: bool,
    __: bool,
    pub daq_cfg_lost: bool,
    pub clear_daq: bool,
    pub store_daq: bool,
    pub cal_pag_cfg_lost: bool,
    pub store_cal: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetStatus {
    pub(crate) session_status: SessionStatus,
    pub(crate) resource_status: ResourceStatus,
    pub(crate) state_number: u8,
    pub(crate) session_cfg_id: u16,
}

impl GetStatus {
    pub fn new(
        session_status: SessionStatus,
        resource_status: ResourceStatus,
        state_number: u8,
        session_cfg_id: u16,
    ) -> Self {
        Self { session_status, resource_status, state_number, session_cfg_id }
    }

    pub const fn length() -> usize {
        1 + 1 + 1 + 2
    }
}

impl Into<Vec<u8>> for GetStatus {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.session_status.into());
        result.push(self.resource_status.into());
        result.push(self.state_number);
        result.extend(self.session_cfg_id.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for GetStatus {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let session_status = SessionStatus::from(data[offset]);
        offset += 1;
        let res_protect_status = ResourceStatus::from(data[offset]);
        offset += 1;
        let state_number = data[offset];
        offset += 1;
        let session_cfg_id = u16::from_be_bytes([data[offset], data[offset+1]]);

        Ok(Self::new(session_status, res_protect_status, state_number, session_cfg_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status() -> anyhow::Result<()> {
        let response = GetStatus::new(
            SessionStatus::new()
                .with_resume(true)
                .with_daq_running(true)
                .with_cal_pag_cfg_lost(true),
            ResourceStatus::new()
                .with_debugging(true)
                .with_programming(true),
            0x01,
            0x1010,
        );
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0xC2, 0x30, 0x01, 0x10, 0x10]);

        let response = GetStatus::try_from(data.as_slice())?;
        let session_status = response.session_status();
        assert!(session_status.resume());
        assert!(session_status.daq_running());
        assert!(session_status.cal_pag_cfg_lost());
        assert!(!session_status.daq_cfg_lost());
        assert!(!session_status.clear_daq());
        assert!(!session_status.store_daq());
        assert!(!session_status.store_cal());

        let resource_status = response.resource_status();
        assert!(resource_status.debugging());
        assert!(resource_status.programming());
        assert!(!resource_status.stim());
        assert!(!resource_status.daq());
        assert!(!resource_status.cal_and_page());

        assert_eq!(response.state_number, 0x01);
        assert_eq!(response.session_cfg_id, 0x1010);

        Ok(())
    }
}
