//! Commons of Service 2C

use crate::{DataIdentifier, enum_extend, UdsError, utils};

enum_extend!(
    pub enum DefinitionType {
        DefineByIdentifier = 0x01,
        DefineByMemoryAddress = 0x02,
        ClearDynamicallyDefinedDataIdentifier = 0x03,
    }, u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DynamicallyDID(pub(crate) u16);

impl TryFrom<u16> for DynamicallyDID {
    type Error = UdsError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match DataIdentifier::from(value) {
            DataIdentifier::Periodic(_) |
            DataIdentifier::DynamicallyDefined(_) => {
                Ok(Self(value))
            },
            _ => Err(UdsError::InvalidDynamicallyDefinedDID(value))
        }
    }
}

impl Into<u16> for DynamicallyDID {
    #[inline]
    fn into(self) -> u16 {
        self.0
    }
}

impl Into<Vec<u8>> for DynamicallyDID {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DynamicallyMemAddr {
    pub did: u16,
    pub position: u8,
    pub mem_size: u8,
}

impl<'a> TryFrom<&'a [u8]> for DynamicallyMemAddr {
    type Error = UdsError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 4, false)?;

        let mut offset = 0;
        let did = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let position = data[offset];
        offset += 1;
        let mem_size = data[offset];

        Ok(Self { did, position, mem_size })
    }
}

impl Into<Vec<u8>> for DynamicallyMemAddr {
    fn into(self) -> Vec<u8> {
        let mut result = self.did.to_be_bytes().to_vec();
        result.push(self.position);
        result.push(self.mem_size);
        result
    }
}
