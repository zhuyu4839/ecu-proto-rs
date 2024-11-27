//! page 209

use getset::{CopyGetters, Getters};
use crate::{AddressGranularity, IntoWith, TryFromWith, XcpError, PROGRAMMING_DWORD_PADDING_SIZE};

#[allow(unused_attributes)]
#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct ProgramNext {
    /// Number of data elements [AG] [1..(MAX_CTO_PGM-2)/AG]
    #[get_copy = "pub"]
    pub(crate) remain_size: u8,
    #[get = "pub"]
    pub(crate) elements: Vec<u8>,
}

impl ProgramNext {
    pub fn new(remain_size: u8, elements: Vec<u8>) -> Self {
        Self { remain_size, elements }
    }

    pub fn length(&self) -> usize {
        1
    }
}

impl IntoWith<Vec<u8>, AddressGranularity> for ProgramNext {
    fn into_with(mut self, ag: AddressGranularity) -> Vec<u8> {
        let mut result = match ag {
            AddressGranularity::Byte
            | AddressGranularity::Word => vec![self.remain_size, ],
            // If AG = DWORD, 2 alignment bytes must be used in order to meet alignment requirements.
            AddressGranularity::DWord => vec![self.remain_size, 0x00, 0x00]
        };
        result.append(&mut self.elements);

        result
    }
}

impl TryFromWith<&[u8], AddressGranularity> for ProgramNext {
    type Error = XcpError;

    fn try_from_with(data: &[u8], ag: AddressGranularity) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = 1;
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data.len() });
        }

        let mut offset = 0;
        let remain_size = data[offset];
        offset += 1;

        match ag {
            AddressGranularity::Byte
            | AddressGranularity::Word => {},
            AddressGranularity::DWord => offset += PROGRAMMING_DWORD_PADDING_SIZE,  // skip padding
        }

        let mut next_pos = offset + ag.bytes() * remain_size as usize;
        if next_pos > data_len {
            next_pos = data_len;
        }

        Ok(Self::new(remain_size, data[offset..next_pos].to_vec()))
    }
}
