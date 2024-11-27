//! page 142

use getset::{CopyGetters, Getters};
use crate::{AddressGranularity, TryFromWith, IntoWith, XcpError};

/// If AG = DWORD, 2 alignment bytes must be used in order to meet alignment requirements.
/// ELEMENT is BYTE, WORD or DWORD depending upon AG.
/// The data block of the specified length (size) contained in the CMD will be copied into
/// memory, starting at the MTA. The MTA will be post-incremented by the number of data
/// elements.
///
/// If the slave device does not support block transfer mode, all downloaded data are
/// transferred in a single command packet. Therefore the number of data elements
/// parameter in the request has to be in the range [1..MAX_CTO/AG-2]. An
/// ERR_OUT_OF_RANGE will be returned, if the number of data elements is more than
/// MAX_CTO/AG-2.
///
/// After receiving a DOWNLOAD command the XCP slave first has to check whether there are
/// enough resources available in order to cover the complete download request. If the XCP
/// slave does not have enough resources, it has to send ERR_MEMORY_OVERFLOW and does
/// not execute any single download request. If a DOWNLOAD request will be rejected, there
/// have been no changes to the slave’s memory contents at all.
/// If block transfer mode is supported, the downloaded data are transferred in multiple
/// command packets. For the slave however, there might be limitations concerning the
/// maximum number of consecutive command packets (block size MAX_BS). Therefore the
/// number of data elements (n) can be in the range [1..min(MAX_BS*(MAX_CTO-
/// 2)/AG,255)].
///
/// If AG=1 the master device has to transmit ((n*AG)-1) / (MAX_CTO-2)) additional
/// consecutive DOWNLOAD_NEXT command packets.
///
/// If AG>1 the master device has to transmit ((n*AG)-1) / (MAX_CTO-AG)) additional
/// consecutive DOWNLOAD_NEXT command packets.
///
/// Without any error, the slave device will acknowledge only the last DOWNLOAD_NEXT
/// command packet. The separation time between the command packets and the maximum
/// number of packets are specified in the response for the GET_COMM_MODE_INFO
/// command (MAX_BS, MIN_ST).
#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct Download {
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

impl Download {
    pub fn new(remain_size: u8, elements: Vec<u8>) -> Self {
        Self { remain_size, padding: Default::default(), elements }
    }

    pub const fn length() -> usize {
        1
    }
}

impl IntoWith<Vec<u8>, AddressGranularity> for Download {
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

impl TryFromWith<&[u8], AddressGranularity> for Download {
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

        let mut next_pos = offset + ag.bytes() * remain_size as usize;
        if next_pos > data_len {
            next_pos = data_len;
        }

        Ok(Self::new(remain_size, data[offset..next_pos].to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() -> anyhow::Result<()> {
        let request = Download::new(0x04, vec![1, 2, 3, 4]);
        let data: Vec<_> = request.into_with(AddressGranularity::Byte);
        assert_eq!(data, vec![0x04, 0x01, 0x02, 0x03, 0x04]);

        let request = Download::try_from_with(&data, AddressGranularity::Word)?;
        assert_eq!(request.remain_size, 0x04);
        assert_eq!(request.elements, vec![1, 2, 3, 4]);
        let data: Vec<_> = request.into_with(AddressGranularity::DWord);
        assert_eq!(data, vec![0x04, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04]);
        let request = Download::try_from_with(&data, AddressGranularity::DWord)?;
        assert_eq!(request.remain_size, 0x04);
        assert_eq!(request.elements, vec![1, 2, 3, 4]);

        Ok(())
    }
}
