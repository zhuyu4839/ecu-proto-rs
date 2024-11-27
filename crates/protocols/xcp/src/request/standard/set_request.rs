use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// Bitfield representation of 6-bit `Current Resource Protection Status parameter in GET_STATUS and UNLOCK`
///
/// ### Repr: `u8`
///
/// | Field                | Size (bits) |
/// |----------------------|-------------|
/// | Reserved             | 4           |
/// | ClearDAQReq          | 1           |
/// | StoreDAQReqResume    | 1           |
/// | StoreDAQReqNoResume  | 1           |
/// | StoreCALReq          | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct RequestMode {
    #[bits(2)]
    __: u8,
    pub clear_daq_cfg_lost: bool,
    pub clear_cal_page_cfg_lost: bool,
    pub clear_daq: bool,
    pub store_daq_resume: bool,
    pub store_daq_no_resume: bool,
    pub store_cal: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct SetRequest {
    pub(crate) mode: RequestMode,
    pub(crate) session_cfg_id: u16,
}

impl SetRequest {
    pub fn new(mode: RequestMode, session_cfg_id: u16) -> Self {
        Self { mode, session_cfg_id }
    }

    pub const fn length() -> usize {
        3
    }
}

impl Into<Vec<u8>> for SetRequest {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.mode.into());
        result.extend(self.session_cfg_id.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for SetRequest {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let mode = RequestMode::from(data[offset]);
        offset += 1;
        let session_cfg_id = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(Self::new(mode, session_cfg_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_request() -> anyhow::Result<()> {
        let request = SetRequest::new(
            RequestMode::new()
                .with_store_daq_no_resume(true)
                .with_store_cal(true),
            0x1010,
        );
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x03, 0x10, 0x10]);

        let request = SetRequest::try_from(data.as_slice())?;
        assert_eq!(request.mode, RequestMode::new()
            .with_store_daq_no_resume(true)
            .with_store_cal(true));
        assert_eq!(request.session_cfg_id, 0x1010);

        Ok(())
    }
}
