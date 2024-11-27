//! page 221

use getset::Getters;
use crate::{AddressGranularity, IntoWith, TryFromWith, XcpError};

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct ProgramMax {
    pub(crate) elements: Vec<u8>,
}

impl ProgramMax {
    pub fn new(elements: Vec<u8>) -> Self {
        Self { elements }
    }
}

impl IntoWith<Vec<u8>, AddressGranularity> for  ProgramMax {
    fn into_with(mut self, ag: AddressGranularity) -> Vec<u8> {
        // Depending upon AG, 1 or 3 alignment bytes must be used in order to meet alignment requirements.
        match ag {
            AddressGranularity::Byte => self.elements,
            AddressGranularity::Word => {
                let mut result = vec![0x00, ];
                result.append(&mut self.elements);

                result
            }
            AddressGranularity::DWord => {
                let mut result = vec![0x00, 0x00, 0x00];
                result.append(&mut self.elements);

                result
            }
        }
    }
}

impl TryFromWith<&[u8], AddressGranularity> for ProgramMax {
    type Error = XcpError;

    fn try_from_with(data: &[u8], ag: AddressGranularity) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let offset = match ag {
            AddressGranularity::Byte => 0x00,
            AddressGranularity::Word => 0x01,   // skip padding
            AddressGranularity::DWord => 0x03,  // skip padding
        };

        if data_len < offset {
            return Err(XcpError::InvalidDataLength { expected: offset, actual: data_len });
        }

        Ok(Self::new(data[offset..].to_vec()))
    }
}
