use getset::CopyGetters;
use crate::XcpError;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum IdType {
    ASCII = 0x00,
    FilenameWithoutPath = 0x01,
    FilenameWithPath = 0x02,
    FileURLFound = 0x03,
    FileToUpload = 0x04,
    EPK = 0x05,
    ECU = 0x06,
    PODSysId = 0x07,
    UserDefined(u8),
    Undefined(u8),
}

impl Into<u8> for IdType {
    fn into(self) -> u8 {
        match self {
            Self::ASCII => 0x00,
            Self::FilenameWithoutPath => 0x01,
            Self::FilenameWithPath => 0x02,
            Self::FileURLFound => 0x03,
            Self::FileToUpload => 0x04,
            Self::EPK => 0x05,
            Self::ECU => 0x06,
            Self::PODSysId => 0x07,
            Self::UserDefined(x) => x,
            Self::Undefined(x) => x,
        }
    }
}

impl From<u8> for IdType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::ASCII,
            0x01 => Self::FilenameWithoutPath,
            0x02 => Self::FilenameWithPath,
            0x03 => Self::FileURLFound,
            0x04 => Self::FileToUpload,
            0x05 => Self::EPK,
            0x06 => Self::ECU,
            0x07 => Self::PODSysId,
            0x7F..=0xFF => Self::UserDefined(byte),
            _ => Self::Undefined(byte),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetId {
    pub(crate) id_type: IdType,
}

impl GetId {
    pub fn new(id_type: IdType) -> GetId {
        Self { id_type }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for GetId {
    fn into(self) -> Vec<u8> {
        vec![self.id_type.into(), ]
    }
}

impl TryFrom<&[u8]> for GetId {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let id_type = IdType::from(data[0]);

        Ok(Self::new(id_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id() -> anyhow::Result<()> {
        let request = GetId::new(IdType::FilenameWithPath);
        let data: Vec<_> = request.into();
        assert_eq!(data, vec![0x02]);

        let request = GetId::try_from(data.as_slice())?;
        assert_eq!(request.id_type, IdType::FilenameWithPath);

        Ok(())
    }
}
