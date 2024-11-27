use getset::{CopyGetters, Getters};
use crate::{AddressGranularity, TryFromWith, IntoWith, XcpError};

#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct DownloadNext {
    /// n = Number of data elements [AG]
    /// [1..(MAX_CTO-2)/AG] Standard mode
    /// [1..min(MAX_BS*(MAX_CTO-2)/AG,255)] Block mode
    #[get_copy = "pub"]
    pub(crate) remain_size: u8,
    /// If AG = DWORD, 2 alignment bytes must be used in order to meet alignment requirements.
    #[getset(skip)]
    padding: u16,
    #[get = "pub"]
    pub(crate) elements: Vec<u8>,
}

impl DownloadNext {
    pub fn new(remain_size: u8, elements: Vec<u8>) -> Self {
        Self { remain_size, padding: Default::default(), elements }
    }

    pub const fn length() -> usize {
        1
    }
}

impl IntoWith<Vec<u8>, AddressGranularity> for DownloadNext {
    fn into_with(mut self, ag: AddressGranularity) -> Vec<u8> {
        let mut result = vec![self.remain_size, ];
        match ag {
            AddressGranularity::Byte
            | AddressGranularity::Word => {},
            AddressGranularity::DWord => {
                result.extend(self.padding.to_be_bytes());
            }
        };
        result.append(&mut self.elements);

        result
    }
}

impl TryFromWith<&[u8], AddressGranularity> for DownloadNext {
    type Error = XcpError;
    fn try_from_with(data: &[u8], ag: AddressGranularity) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let remain_size = data[offset];
        offset += 1;
        match ag {
            AddressGranularity::Byte
            | AddressGranularity::Word => {},
            AddressGranularity::DWord => offset += 2,   // skip padding
        };

        let mut next_pos = offset + ag.bytes() + remain_size as usize;
        if data_len > next_pos {
            next_pos = data_len;
        }

        Ok(Self::new(remain_size, data[offset..next_pos].to_vec()))
    }
}
