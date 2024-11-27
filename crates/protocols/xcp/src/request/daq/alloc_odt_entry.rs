use getset::CopyGetters;
use crate::XcpError;

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct AllocODTEntry {
    #[getset(skip)]
    reserved: u8,
    pub(crate) daq_list_number: u16,
    pub(crate) odt_number: u8,
    pub(crate) odt_entry_count: u8,
}

impl AllocODTEntry {
    pub fn new(daq_list_number: u16, odt_number: u8, odt_entry_count: u8) -> Self {
        Self { reserved: Default::default(), daq_list_number, odt_number, odt_entry_count }
    }

    pub const fn length() -> usize {
        4
    }
}

impl Into<Vec<u8>> for AllocODTEntry {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.reserved);
        result.extend(self.daq_list_number.to_be_bytes());
        result.push(self.odt_number);
        result.push(self.odt_entry_count);

        result
    }
}

impl TryFrom<&[u8]> for AllocODTEntry {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 1; // skip reserved
        let daq_list_number = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let odt_number = data[offset];
        offset += 1;
        let odt_entry_count = data[offset];

        Ok(Self::new(daq_list_number, odt_number, odt_entry_count))
    }
}
