//! page 170

use getset::{CopyGetters, Getters};
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct DAQElement {
    /// BIT_OFFSET [0..31] of 1st element
    /// Position of bit in 32-bit variable referenced by the address and extension below
    pub(crate) bit_offset: u8,
    /// Size of x DAQ element
    /// 0 <= size <= MAX_ODT_ENTRY_SIZE_DAQ_x
    pub(crate) size: u8,
    /// Address of x DAQ element
    pub(crate) address: u32,
    /// Address extension of x DAQ element
    pub(crate) address_extension: u8,
    /// Dummy for alignment of the next element
    pub(crate) dummy: u8,
}

impl DAQElement {
    pub fn new(bit_offset: u8, size: u8, address: u32, address_extension: u8) -> Self {
        Self { bit_offset, size, address, address_extension, dummy: Default::default() }
    }

    pub const fn length() -> usize {
        8
    }
}

impl Into<Vec<u8>> for DAQElement {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.bit_offset);
        result.push(self.size);
        result.extend(self.address.to_be_bytes());
        result.push(self.address_extension);
        result.push(self.dummy);

        result
    }
}

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct WriteDAQMultiple {
    pub(crate) elements: Vec<DAQElement>,
}

impl WriteDAQMultiple {
    pub fn new(elements: Vec<DAQElement>) -> Self {
        Self { elements }
    }
}

impl Into<Vec<u8>> for WriteDAQMultiple {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.elements.len() as u8, ];
        self.elements
            .into_iter()
            .for_each(|element| result.append(&mut element.into()));

        result
    }
}

impl TryFrom<&[u8]> for WriteDAQMultiple {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = 1;
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let size = data[offset] as usize;
        offset += 1;

        let element_len = DAQElement::length();
        if (data_len - 1) % element_len != 0 {
            return Err(XcpError::MissData { expected: element_len * size, actual: data_len });
        }

        let mut elements = Vec::with_capacity(size);
        while offset < data_len {
            let bit_offset = data[offset];
            offset += 1;
            let size = data[offset];
            offset += 1;
            let address = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());
            offset += 4;
            let address_extension = data[offset];
            offset += 1;
            // let dummy = data[offset];
            offset += 1;    // skip dummy

            elements.push(DAQElement::new(bit_offset, size.into(), address, address_extension));
        }

        Ok(Self::new(elements))
    }
}
