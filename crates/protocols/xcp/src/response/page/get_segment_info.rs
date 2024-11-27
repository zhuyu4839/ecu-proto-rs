use getset::CopyGetters;
use crate::{SegmentInfoMode, TryFromWith, XcpError};

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSegmentInfoM0 {
    #[getset(skip)]
    reserve0: u8,
    #[getset(skip)]
    reserve1: u16,
    /// If SEGMENT_INFO = 0 , this command returns the address of this SEGMENT in `BASIC_INFO`.
    /// If SEGMENT_INFO = 1 , this command returns the length of this SEGMENT in `BASIC_INFO`.
    pub(crate) segment_info: u32,
}

impl GetSegmentInfoM0 {
    pub fn new(segment_info: u32) -> Self {
        Self { reserve0: Default::default(), reserve1: Default::default(), segment_info }
    }

    pub const fn length() -> usize {
        7
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSegmentInfoM1 {
    pub(crate) number_of_pages: u8,
    pub(crate) address_extension: u8,
    pub(crate) number_of_mapped_addresses: u8,
    pub(crate) compression_method: u8,
    pub(crate) encryption_method: u8,
}

impl GetSegmentInfoM1 {
    pub fn new(
        number_of_pages: u8,
        address_extension: u8,
        number_of_mapped_addresses: u8,
        compression_method: u8,
        encryption_method: u8,
    ) -> Self {
        Self {
            number_of_pages,
            address_extension,
            number_of_mapped_addresses,
            compression_method,
            encryption_method,
        }
    }

    pub const fn length() -> usize {
        5
    }
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetSegmentInfoM2 {
    #[getset(skip)]
    reserve0: u8,
    #[getset(skip)]
    reserve1: u16,
    /// If SEGMENT_INFO = 0 , this command returns the source address for this MAPPING_INDEX in MAPPING_INFO.
    /// If SEGMENT_INFO = 1 , this command returns the destination address for this MAPPING_INDEX in MAPPING_INFO.
    /// If SEGMENT_INFO = 2 , this command returns the length for this MAPPING_INDEX in MAPPING_INFO.
    pub(crate) mapping_info: u32,
}

impl GetSegmentInfoM2 {
    pub fn new(mapping_info: u32) -> Self {
        Self { reserve0: Default::default(), reserve1: Default::default(), mapping_info }
    }

    pub const fn length() -> usize {
        7
    }
}

#[derive(Debug, Clone,)]
pub enum GetSegmentInfo {
    Mode0(GetSegmentInfoM0),
    Mode1(GetSegmentInfoM1),
    Mode2(GetSegmentInfoM2),
}

impl Into<Vec<u8>> for GetSegmentInfo {
    fn into(self) -> Vec<u8> {
        match self {
            GetSegmentInfo::Mode0(v) => {
                let mut result = Vec::with_capacity(GetSegmentInfoM0::length());
                result.push(v.reserve0);
                result.extend(v.reserve1.to_be_bytes());
                result.extend(v.segment_info.to_be_bytes());

                result
            }
            GetSegmentInfo::Mode1(v) => {
                let mut result = Vec::with_capacity(GetSegmentInfoM1::length());
                result.push(v.number_of_pages);
                result.push(v.address_extension);
                result.push(v.number_of_mapped_addresses);
                result.push(v.compression_method);
                result.push(v.encryption_method);

                result
            }
            GetSegmentInfo::Mode2(v) => {
                let mut result = Vec::with_capacity(GetSegmentInfoM0::length());
                result.push(v.reserve0);
                result.extend(v.reserve1.to_be_bytes());
                result.extend(v.mapping_info.to_be_bytes());

                result
            }
        }
    }
}

impl TryFromWith<&[u8], SegmentInfoMode> for GetSegmentInfo {
    type Error = XcpError;

    fn try_from_with(data: &[u8], val: SegmentInfoMode) -> Result<Self, Self::Error> {
        let data_len = data.len();
        match val {
            SegmentInfoMode::BasicAddress => {
                let expected = GetSegmentInfoM0::length();
                if data_len < expected {
                    return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                }

                let offset = 3; // skip reserved
                let segment_info = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());

                Ok(Self::Mode0(GetSegmentInfoM0::new(segment_info)))
            }
            SegmentInfoMode::Standard => {
                let expected = GetSegmentInfoM1::length();
                if data_len < expected {
                    return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                }

                let mut offset = 0;
                let number_of_pages = data[offset];
                offset += 1;
                let address_extension = data[offset];
                offset += 1;
                let number_of_mapped_addresses = data[offset];
                offset += 1;
                let compression_method = data[offset];
                offset += 1;
                let encryption_method = data[offset];

                Ok(Self::Mode1(GetSegmentInfoM1::new(
                    number_of_pages,
                    address_extension,
                    number_of_mapped_addresses,
                    compression_method,
                    encryption_method)
                ))
            }
            SegmentInfoMode::AddressMapping => {
                let expected = GetSegmentInfoM2::length();
                if data_len < expected {
                    return Err(XcpError::InvalidDataLength { expected, actual: data_len });
                }

                let offset = 3; // skip reserved
                let mapping_info = u32::from_be_bytes(data[offset..offset+4].try_into().unwrap());

                Ok(Self::Mode2(GetSegmentInfoM2::new(mapping_info)))
            }
            SegmentInfoMode::Undefined(_) => {
                log::warn!("unsupported segment info mode: {:?}", val);
                Err(XcpError::UndefinedError)
            }
        }
    }
}
