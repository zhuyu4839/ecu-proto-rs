//! page 147

use getset::{CopyGetters, Getters};
use crate::XcpError;

/// ELEMENT is BYTE, WORD or DWORD, depending upon AG.
///
/// A data block of the specified length, starting at address will be written. The MTA pointer is
/// set to the first data element behind the downloaded data block. If the number of elements
/// exceeds (MAX_CTO-8)/AG, the error code ERR_OUT_OF_RANGE will be returned.
/// After receiving a SHORT_DOWNLOAD command the XCP slave first has to check whether
/// there are enough resources available in order to cover the complete download request. If
/// the XCP slave does not have enough resources, it has to send ERR_MEMORY_OVERFLOW
/// and does not execute any single download request. If a SHORT_DOWNLOAD request will be
/// rejected, there have been no changes to the slave’s memory contents at all.
///
/// This command does not support block transfer and it must not be used within a block
/// transfer sequence.
///
/// Please note that this command will have no effect (no data bytes can be transferred) if
/// MAX_CTO = 8 (e.g. XCP on CAN).
#[derive(Debug, Clone, CopyGetters, Getters)]
pub struct ShortDownload {
    // #[get_copy = "pub"]
    // pub(crate) size: u8,
    #[getset(skip)]
    reserved: u8,
    #[get_copy = "pub"]
    pub(crate) address_extension: u8,
    #[get_copy = "pub"]
    pub(crate) address: u32,
    #[get = "pub"]
    pub(crate) elements: Vec<u8>,
}

impl ShortDownload {
    pub fn new(address_extension: u8, address: u32, elements: Vec<u8>) -> Self {
        Self { reserved: Default::default(), address_extension, address, elements }
    }

    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for ShortDownload {
    fn into(mut self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.elements.len() as u8);
        result.push(self.reserved);
        result.push(self.address_extension);
        result.extend(self.address.to_be_bytes());
        result.append(&mut self.elements);

        result
    }
}

impl TryFrom<&[u8]> for ShortDownload {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let size = data[offset];
        offset += 1;
        offset += 1;    // skip reserved
        let address_extension = data[offset];
        offset += 1;
        let address = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let elements = data[offset..offset+size as usize].to_vec();

        Ok(Self::new(address_extension, address, elements))
    }
}
