use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ResumeMode {
    pub(crate) session_cfg_id: u16,
    pub(crate) slave_timestamp: Option<u64>,
}

impl ResumeMode {
    pub fn new(
        session_cfg_id: u16,
        slave_timestamp: Option<u64>,
    ) -> Self {
        Self { session_cfg_id, slave_timestamp }
    }

    pub const fn length() -> usize {
        2
    }
}

impl Into<Vec<u8>> for ResumeMode {
    fn into(self) -> Vec<u8> {
        let mut result = self.session_cfg_id.to_be_bytes().to_vec();
        if let Some(timestamp) = self.slave_timestamp {
            result.extend(timestamp.to_be_bytes());
        }

        result
    }
}

impl TryFrom<&[u8]> for ResumeMode {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength {  expected, actual: data_len });
        }

        let mut offset = 0;
        let session_cfg_id = u16::from_be_bytes([data[offset], data[offset+1]]);
        offset += 2;
        let timestamp = match data_len - offset {
            0 => Ok(None),
            8 => Ok(Some(u64::from_be_bytes(data[offset..offset+8].try_into().unwrap()))),
            _ => Err(XcpError::InvalidDataLength {  expected: offset+8, actual: data_len}),
        }?;

        Ok(Self::new(session_cfg_id, timestamp))
    }
}
