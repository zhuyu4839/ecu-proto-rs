use getset::CopyGetters;
use crate::XcpError;

/// If the DTO packets have an identification field type “absolute ODT number”, FIRST_PID
/// is the absolute ODT number in the DTO packet of the first ODT transferred by this DAQ list.
///
/// The absolute ODT number for any other ODT can be determined by:
///
/// Absolute_ODT_number(ODT i in DAQ list j) = FIRST_PID(DAQ list j) + relative_ODT_NUMBER(ODT i)
///
/// If the DTO packets have an identification field type “relative ODT number and absolute
/// DAQ list number”, FIRST_PID can be ignored.
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct StartStopDaqList {
    pub(crate) first_pid: u8,
}

impl StartStopDaqList {
    pub fn new(first_pid: u8) -> Self {
        Self { first_pid }
    }

    pub const fn length() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for StartStopDaqList {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let first_pid = data[0];

        Ok(Self::new(first_pid))
    }
}
