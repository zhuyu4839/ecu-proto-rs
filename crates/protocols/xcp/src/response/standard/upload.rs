//! page 134

use getset::Getters;
use crate::{AddressGranularity, TryFromWith, IntoWith, XcpError};

/// If the slave device does not support block transfer mode, all uploaded data are transferred
/// in a single response packet. Therefore the number of data elements parameter in the
/// request has to be in the range [1..MAX_CTO/AG-1]. An ERR_OUT_OF_RANGE will be
/// returned, if the number of data elements is more than MAX_CTO/AG-1.
///
/// If block transfer mode is supported, the uploaded data are transferred in multiple
/// responses on the same request packet. For the master there are no limitations allowed
/// concerning the maximum block size. Therefore the number of data elements (n) can be in
/// the range [1..255]. The slave device will transmit ((n*AG)-1) / (MAX_CTO-AG) +1
/// response packets. The separation time between the response packets is depending on
/// the slave device implementation. It’s the responsibility of the master device to keep track
/// of all packets and to check for lost packets. It is slave device implementation specific if the
/// data in different response packets are consistent. For instance, this has to be considered,
/// when block upload mode is used to obtain 8 byte floating point objects.

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct Upload {
    /// Used for alignment only if AG > 1
    // #[getset(skip)]
    // padding: u32,
    /// ELEMENT is BYTE, WORD or DWORD, depending upon AG.
    pub(crate) elements: Vec<u8>,
}

impl Upload {
    pub fn new(elements: Vec<u8>) -> Self {
        Self { elements }
    }
}

impl IntoWith<Vec<u8>, AddressGranularity> for Upload {
    fn into_with(mut self, ag: AddressGranularity) -> Vec<u8> {
        // Depending on AG 1, 2 or 3 alignment bytes must be used in order to meet alignment
        // requirements.
        let mut result = match ag {
            AddressGranularity::Byte => vec![0x00, ],
            AddressGranularity::Word => vec![0x00, 0x00],
            AddressGranularity::DWord => vec![0x00, 0x00, 0x00],
        };
        result.append(&mut self.elements);

        result
    }
}

impl TryFromWith<&[u8], AddressGranularity> for Upload {
    type Error = XcpError;
    fn try_from_with(data: &[u8], ag: AddressGranularity) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let offset = match ag {
            AddressGranularity::Byte => 0x01,
            AddressGranularity::Word => 0x02,
            AddressGranularity::DWord => 0x03,
        };

        if data_len < offset {
            return Err(XcpError::InvalidDataLength { expected: offset, actual: data_len });
        }

        Ok(Self::new(data[offset..].to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload() -> anyhow::Result<()> {
        let response = Upload::new(vec![0, 1, 2, 3, 4, 5, 6]);
        let data: Vec<_> = response.into_with(AddressGranularity::Byte);
        assert_eq!(data, vec![0, 0, 1, 2, 3, 4, 5, 6]);

        let response = Upload::try_from_with(&data, AddressGranularity::Byte)?;
        assert_eq!(response.elements, vec![0, 1, 2, 3, 4, 5, 6]);

        Ok(())
    }
}
