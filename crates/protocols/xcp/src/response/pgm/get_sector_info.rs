use getset::CopyGetters;
use crate::{GetSectorInfoMode, ProgrammingMethod, TryFromWith, XcpError};

#[derive(Debug, Clone)]
pub enum GetSectorInfo {
    M01(GetSectorInfoM01),
    M2(GetSectorInfoM2),
}

impl Into<Vec<u8>> for GetSectorInfo {
    fn into(self) -> Vec<u8> {
        match self {
            Self::M01(info) => info.into(),
            Self::M2(info) => info.into(),
        }
    }
}

impl TryFromWith<&[u8], GetSectorInfoMode> for GetSectorInfo {
    type Error = XcpError;

    fn try_from_with(data: &[u8], mode: GetSectorInfoMode) -> Result<Self, Self::Error> {
        match mode {
            GetSectorInfoMode::StartAddress
            | GetSectorInfoMode::Length => {
                Ok(Self::M01(GetSectorInfoM01::try_from(data)?))
            }
            GetSectorInfoMode::NameOfLength => {
                Ok(Self::M2(GetSectorInfoM2::try_from(data)?))
            }
            GetSectorInfoMode::Undefined(_) => Err(XcpError::UndefinedError),
        }
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSectorInfoM01 {
    /// Clear Sequence Number
    pub(crate) clear_sequence_number: u8,
    /// Program Sequence Number
    pub(crate) program_sequence_number: u8,
    /// Programming method
    pub(crate) programming_method: ProgrammingMethod,
    /// mode = 0 : Start address for this SECTOR
    /// mode = 1 : Length of this SECTOR [BYTE]
    pub(crate) sector_info: u32,
}

impl GetSectorInfoM01 {
    pub fn new(
        clear_sequence_number: u8,
        program_sequence_number: u8,
        programming_method: ProgrammingMethod,
        sector_info: u32,
    ) -> Self {
        Self {
            clear_sequence_number,
            program_sequence_number,
            programming_method,
            sector_info,
        }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for GetSectorInfoM01 {
    fn into(self) -> Vec<u8> {
        let mut result = vec![
            self.clear_sequence_number,
            self.program_sequence_number,
            self.programming_method.into(),
        ];
        result.extend(self.sector_info.to_be_bytes());

        result
    }
}

impl TryFrom<&[u8]> for GetSectorInfoM01 {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let clear_sequence_number = data[offset];
        offset += 1;
        let program_sequence_number = data[offset];
        offset += 1;
        let programming_method = ProgrammingMethod::from(data[offset]);
        offset += 1;
        let sector_info = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());

        Ok(Self::new(clear_sequence_number, program_sequence_number, programming_method, sector_info))
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSectorInfoM2 {
    /// Number of Data Elements UPLOAD [AG] = (Length GET_SECTOR_INFO[BYTE]) / AG
    /// SECTOR_NAME_LENGTH in bytes
    /// 0 – if not available
    pub(crate) size: u8,
}

impl GetSectorInfoM2 {
    pub fn new(size: u8) -> Self {
        Self { size }
    }

    pub const fn length() -> usize {
        1
    }
}

impl Into<Vec<u8>> for GetSectorInfoM2 {
    fn into(self) -> Vec<u8> {
        vec![self.size, ]
    }
}

impl TryFrom<&[u8]> for GetSectorInfoM2 {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let length = data[0];
        Ok(Self::new(length))
    }
}
