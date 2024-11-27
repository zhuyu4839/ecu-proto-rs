use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

#[bitfield(u16, order = Msb)]
#[derive(PartialEq, Eq)]
pub struct Version {
    #[bits(8)]
    pub major: u8,
    #[bits(8)]
    pub minor: u8,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetVersion {
    #[getset(skip)]
    reserved: u8,
    pub(crate) protocol_version: Version,
    pub(crate) transport_version: Version,
}

impl GetVersion {
    pub fn new(protocol_version: Version, transport_version: Version) -> Self {
        Self { reserved: Default::default(), protocol_version, transport_version }
    }

    pub const fn length() -> usize {
        1 + 2 + 2
    }
}

impl Into<Vec<u8>> for GetVersion {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.reserved);
        result.push(self.protocol_version.major());
        result.push(self.protocol_version.minor());
        result.push(self.transport_version.major());
        result.push(self.transport_version.minor());

        result
    }
}

impl TryFrom<&[u8]> for GetVersion {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 1; // skip reserved
        let protocol_version = Version::new()
            .with_major(data[offset])
            .with_minor(data[offset + 1]);
        offset += 2;
        let transport_version = Version::new()
            .with_major(data[offset])
            .with_minor(data[offset + 1]);

        Ok(Self::new(protocol_version, transport_version))
    }
}
