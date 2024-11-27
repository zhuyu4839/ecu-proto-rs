//! negative data

mod build_checksum;
pub use build_checksum::*;
mod download_next;
pub use download_next::*;
mod program_next;
pub use program_next::*;

use getset::{CopyGetters, Getters};
use crate::{Command, ErrorCode, XcpError};

#[derive(Debug, Clone, Getters, CopyGetters, )]
pub struct Negative {
    #[get_copy = "pub"]
    pub(crate) code: ErrorCode,
    #[get = "pub"]
    pub(crate) data: Vec<u8>,
}

impl Negative {
    pub fn common(command: Command, code: ErrorCode) -> Option<Self> {
        match command {
            Command::BuildChecksum
            | Command::CALDownloadNext
            | Command::PGMPrgNext => None,
            _ => Some(Self { code, data: vec![] }),
        }
    }

    /// If MTA and block size does not meet alignment requirements, an ERR_OUT_OF_RANGE
    /// with the required MTA_BLOCK_SIZE_ALIGN will be returned. If the block size exceeds
    /// the allowed maximum value, an ERR_OUT_OF_RANGE will be returned. The maximum
    /// block size will be returned in the checksum field
    pub fn build_checksum(mta_bs_align: u16, max_bs: u32) -> Self {
        let response = BuildChecksum::new(mta_bs_align, max_bs);
        Self { code: ErrorCode::OutOfRange, data: response.into() }
    }

    pub fn download_next(expected_size: u8) -> Self {
        let response = DownloadNext::new(expected_size);
        Self { code: ErrorCode::SequenceError, data: response.into() }
    }

    pub fn program_next(expected_size: u8) -> Self {
        let response = ProgramNext::new(expected_size);
        Self { code: ErrorCode::SequenceError, data: response.into() }
    }

    #[inline]
    pub fn origin_data<T>(&self) -> Result<T, XcpError>
    where
        T: for<'a> TryFrom<&'a [u8], Error = XcpError>
    {
        T::try_from(self.data.as_slice())
    }
}

impl Into<Vec<u8>> for Negative {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![0xFE, self.code.into()];
        result.append(&mut self.data);

        result
    }
}

impl TryFrom<&[u8]> for Negative {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = 1;
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let code = ErrorCode::from(data[offset]);
        offset += 1;
        match code {
            ErrorCode::Undefined(_) => Err(XcpError::UndefinedError),
            _ => Ok(())
        }?;

        Ok(Self { code, data: data[offset..].to_vec() })
    }
}
