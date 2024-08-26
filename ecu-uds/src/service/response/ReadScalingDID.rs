use std::collections::HashSet;
use bitfield_struct::bitfield;
use lazy_static::lazy_static;
use crate::{enum_to_vec, utils};
use crate::error::Error;
use crate::service::{Configuration, DataIdentifier, Placeholder, ResponseData};
use crate::service::response::Code;

lazy_static!(
    pub static ref READ_SCALING_DID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        #[cfg(any(feature = "std2020"))]
        Code::AuthenticationRequired,
    ]);
);

enum_to_vec! (
    /// Table C.2 — scalingByte (High Nibble) parameter definitions
    pub enum ScalingByteType {
        UnSignedNumeric = 0x00,             // (1 to 4 bytes)
        SignedNumeric = 0x10,               // (1 to 4 bytes)
        BitMappedReportedWithOutMask = 0x20,//
        BitMappedReportedWithMask = 0x30,   // 0 byte
        BinaryCodedDecimal = 0x40,
        StateEncodedVariable = 0x50,        // always 1 byte
        ASCII = 0x60,
        SignedFloatingPoint = 0x70,
        Packet = 0x80,
        Formula = 0x90,
        UnitFormat = 0xA0,
        StateAndConnectionType = 0xB0,
    }, u8, Error, InvalidParam
);

/// Table C.6 — formulaIdentifier encoding
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Formula {
    Formula0,     // y = C0 * x + C1
    Formula1,     // y = C0 * (x + C1)
    Formula2,     // y = C0 / (x + C1) + C2
    Formula3,     // y = x / C0 + C1
    Formula4,     // y = (x + C0) / C1
    Formula5,     // y = (x + C0) / C1 + C2
    Formula6,     // y = C0 * x
    Formula7,     // y = x / C0
    Formula8,     // y = x + C0
    Formula9,     // y = x * C0 / C1
    Reserved(u8),
    VehicleManufacturerSpecific(u8),
}

impl From<u8> for Formula {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Formula0,
            0x01 => Self::Formula1,
            0x02 => Self::Formula2,
            0x03 => Self::Formula3,
            0x04 => Self::Formula4,
            0x05 => Self::Formula5,
            0x06 => Self::Formula6,
            0x07 => Self::Formula7,
            0x08 => Self::Formula8,
            0x09 => Self::Formula9,
            0x0A..=0x7F => Self::Reserved(value),
            0x80..=0xFF => Self::VehicleManufacturerSpecific(value),
        }
    }
}

impl Into<u8> for Formula {
    fn into(self) -> u8 {
        match self {
            Self::Formula0 => 0x00,
            Self::Formula1 => 0x01,
            Self::Formula2 => 0x02,
            Self::Formula3 => 0x03,
            Self::Formula4 => 0x04,
            Self::Formula5 => 0x05,
            Self::Formula6 => 0x06,
            Self::Formula7 => 0x07,
            Self::Formula8 => 0x08,
            Self::Formula9 => 0x09,
            Self::VehicleManufacturerSpecific(v) => v,
            Self::Reserved(v) => v,
        }
    }
}

/// Table C.7 — Two byte real number format
#[bitfield(u16, order = Msb)]
pub struct TwoByteRealNumber {
    #[bits(4)]
    exponent: u8,
    #[bits(12)]
    mantissa: u16,
}

impl TwoByteRealNumber {
    pub fn value(&self) -> u128 {
        self.mantissa() as u128 * (10 ^ (self.exponent() as u128))
    }
}

#[derive(Debug, Clone)]
pub struct ScalingByte {
    pub byte_type: ScalingByteType,
    pub byte_len: u8,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ReadScalingDIDData {
    pub did: DataIdentifier,
    pub scaling_byte: ScalingByte,
    pub others: Vec<ScalingByte>, // at least one
}

impl<'a> TryFrom<&'a [u8]> for ReadScalingDIDData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        let byte_context = data[offset];
        offset += 1;
        let byte_type = ScalingByteType::try_from(byte_context & 0xF0)?;
        let mut byte_len = (byte_context & 0x0F) as usize;
        let mut bytes = Vec::new();

        match byte_type {
            ScalingByteType::BitMappedReportedWithOutMask |
            ScalingByteType::Formula |
            ScalingByteType::UnitFormat => {
                utils::data_length_check(data_len, offset + byte_len, false)?;

                bytes.extend(&data[offset..offset + byte_len]);
                offset += byte_len;
            },
            _ => {},
        }

        let mut others = Vec::new();
        while data_len > offset {
            let byte_context = data[offset];
            offset += 1;
            let byte_type = ScalingByteType::try_from(byte_context & 0xF0)?;
            let mut byte_len = (byte_context & 0x0F) as usize;
            let mut bytes = Vec::new();

            match byte_type {
                ScalingByteType::BitMappedReportedWithOutMask |
                ScalingByteType::Formula |
                ScalingByteType::UnitFormat => {
                    if data_len < offset + byte_len {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    bytes.extend(&data[offset..offset + byte_len]);
                    offset += byte_len;
                },
                _ => {},
            }

            others.push(ScalingByte { byte_type, byte_len: byte_len as u8, bytes });
        }

        Ok(Self { did, scaling_byte: ScalingByte { byte_type, byte_len: byte_len as u8, bytes }, others })
    }
}

impl Into<Vec<u8>> for ReadScalingDIDData {
    fn into(self) -> Vec<u8> {
        let did: u16 = self.did.into();
        let mut result = did.to_be_bytes().to_vec();

        let byte_type: u8 = self.scaling_byte.byte_type.into();
        result.push(byte_type | self.scaling_byte.byte_len);

        self.others
            .into_iter()
            .for_each(|mut v| {
                let byte_type: u8 = v.byte_type.into();
                result.push(byte_type | v.byte_len);
                result.append(&mut v.bytes);
            });

        result
    }
}

impl ResponseData for ReadScalingDIDData {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
}

