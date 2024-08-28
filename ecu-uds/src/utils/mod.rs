use std::fmt::{Display, LowerHex, Write};
use isotp_rs::ByteOrder;
use crate::error::Error;

/// Add to_vector function and
/// implement `Debug`, `Copy`, `Clone`, `Eq`, `PartialEq`,
/// `Hash`, `TryFrom`, `Into` trait for enum.
///
/// Example:
/// ```rust
/// use ecu_uds::enum_to_vec;
///
/// #[derive(Debug, thiserror::Error)]
/// pub enum MyError {
///     #[error("Utils error: {0}")]
///     OtherError(String),
/// }
///
/// enum_to_vec!(
///     pub enum AccessType {
///         ReadExtendedTimingParameterSet = 1,
///         SetTimingParametersToDefaultValues = 2,
///         ReadCurrentlyActiveTimingParameters = 3,
///         SetTimingParametersToGivenValues = 4,
///     }, u8, MyError, OtherError
/// );
///
/// let demo: u8 = AccessType::ReadExtendedTimingParameterSet.into();
/// assert_eq!(demo, 1);
/// let demo = AccessType::try_from(1).unwrap();
/// assert_eq!(demo, AccessType::ReadExtendedTimingParameterSet);
/// let demos = AccessType::to_vec();
/// assert_eq!(demos, vec![1, 2, 3, 4]);
/// ```
#[macro_export]
macro_rules! enum_to_vec {
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:expr
            ),* $(,)?
        },
        $value_type:ty,
        $error_type: ident,
        $error_detail: ident
    ) => {
        $(#[$meta])*
        #[repr($value_type)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        $vis enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant = $value
            ),*
        }

        impl $enum_name {
            #[inline]
            pub fn to_vec() -> Vec<$value_type> {
                vec![$($value as $value_type),*]
            }
        }

        // impl PartialEq<$value_type> for $enum_name {
        //     #[inline]
        //     fn eq(&self, other: &$value_type) -> bool {
        //         &self as $value_type == *other
        //     }
        // }

        impl Into<$value_type> for $enum_name {
            #[inline]
            fn into(self) -> $value_type {
                self as $value_type
            }
        }

        impl TryFrom<$value_type> for $enum_name {
            type Error = $error_type;

            #[inline]
            fn try_from(value: $value_type) -> Result<Self, Self::Error> {
                let all_values = Self::to_vec();
                if all_values.contains(&value) {
                    Ok(unsafe { std::mem::transmute(value) })
                } else {
                    Err($error_type::$error_detail(format!("the value {0:x} is invalid or ISO/SAE reserved", value)))
                }
            }
        }
    };
}

#[inline]
pub(crate) fn err_msg<T: Display + LowerHex>(v: T) -> String {
    format!("the value {0:x} is invalid or ISO/SAE reserved", v)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct U24(pub(crate) u32);

impl U24 {
    #[inline]
    pub fn from_be_bytes(data: [u8; 4]) -> Self {
        U24(u32::from_be_bytes(data))
    }

    #[inline]
    pub fn from_le_bytes(data: [u8; 4]) -> Self {
        U24(u32::from_le_bytes(data))
    }

    #[inline]
    pub fn from_ne_bytes(data: [u8; 4]) -> Self {
        U24(u32::from_ne_bytes(data))
    }
}

impl<'a> TryFrom<&'a [u8]> for U24 {
    type Error = Error;

    #[inline]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        data_length_check(data_len, 3, false)?;

        Ok(Self(u32::from_be_bytes([0x00, data[0], data[1], data[2]])))
    }
}

impl Into<Vec<u8>> for U24 {
    #[inline]
    fn into(self) -> Vec<u8> {
        vec![
            ((self.0 & 0xFF0000) >> 16) as u8,
            ((self.0 & 0x00FF00) >> 8) as u8,
            (self.0 & 0x0000FF) as u8
        ]
    }
}

impl From<u32> for U24 {
    #[inline]
    fn from(value: u32) -> Self {
        Self (value & 0xFFFFFF)
    }
}

impl Into<u32> for U24 {
    #[inline]
    fn into(self) -> u32 {
        self.0
    }
}

/// Convert a hex slice to hex string.
///
/// Example:
/// ```rust
/// use ecu_uds::utils;
///
/// let vector = vec![0x11, 0x12, 0x13, 0x14, 0x15];
/// let result = utils::hex_slice_to_string(&vector);
/// assert_eq!(result, "11 12 13 14 15 ".to_string());
/// let result = utils::hex_slice_to_string(vector.as_slice());
/// assert_eq!(result, "11 12 13 14 15 ".to_string());
/// ```
#[inline]
pub fn hex_slice_to_string<T>(data: &[T]) -> String
    where
        T: std::fmt::UpperHex + Copy {
    data.iter().fold(
        String::new(),
        |mut out, &b| {
            let _ = write!(out, "{b:02X} ");
            out
        })
}

#[inline]
pub fn peel_suppress_positive(value: u8) -> (bool, u8) {
    ((value & 0x80) == 0x80, value & 0x7F)
}

#[inline]
pub fn revert_suppress_positive(value: u8, state: bool) -> u8 {
    if state { value | 0x80 } else { value }
}

#[inline]
pub(crate) fn data_length_check(actual: usize, expect: usize, equal: bool) -> Result<(), Error> {
    if equal {
        if actual != expect {
            return Err(Error::InvalidDataLength { expect, actual });
        }
    }
    else {
        if actual < expect {
            return Err(Error::InvalidDataLength { expect, actual });
        }
    }

    Ok(())
}

#[inline]
fn is_big_endian() -> bool {
    1u16.to_ne_bytes()[0] == 0
}

pub(crate) fn u128_to_vec_fix(value: u128, bo: ByteOrder) -> Vec<u8> {
    let mut result = value.to_le_bytes().to_vec();
    let mut count = result.len();

    for i in result.iter().rev() {
        if *i == 0x00 {
            count -= 1;
        }
        else {
            break;
        }
    }

    result.resize(count, Default::default());

    match bo {
        ByteOrder::Big => result.reverse(),
        ByteOrder::Little => {},
        ByteOrder::Native => if is_big_endian() {
            result.reverse();
        },
    }

    result
}

pub(crate) fn u128_to_vec(value: u128, len: usize, bo: ByteOrder) -> Vec<u8> {
    let mut result = value.to_le_bytes().to_vec();
    result.resize(len, Default::default());

    match bo {
        ByteOrder::Big => result.reverse(),
        ByteOrder::Little => {},
        ByteOrder::Native => if is_big_endian() {
            result.reverse();
        },
    }

    result
}

#[inline]
pub(crate) fn slice_to_u128(slice: &[u8], bo: ByteOrder) -> u128 {
    let mut data = slice.to_vec();
    match bo {
        ByteOrder::Big => data.reverse(),
        ByteOrder::Little => {},
        ByteOrder::Native => if is_big_endian() {
            data.reverse();
        },
    }

    data.resize(std::mem::size_of::<u128>(), Default::default());
    data.reverse();
    u128::from_be_bytes(data.try_into().unwrap())
}

#[inline]
pub(crate) fn length_of_u_type<T>(mut value: T) -> usize
    where
        T: std::ops::ShrAssign + std::cmp::PartialOrd + From<u8> {
    let mut result = 0;

    while value > 0.into() {
        result += 1;
        value >>= 8.into();
    }

    result
}

pub fn fix_length(length: u8) -> Option<u8> {
    match length {
        0..=8   => Some(length),
        9..=12  => Some(12),
        13..=16 => Some(16),
        17..=20 => Some(20),
        21..=24 => Some(24),
        25..=32 => Some(32),
        33..=48 => Some(48),
        49..=64 => Some(64),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use isotp_rs::ByteOrder;

    #[test]
    fn test_u128_to_vec() -> anyhow::Result<()> {
        let result = super::u128_to_vec(0x00_12_34_78, 3, ByteOrder::Big);
        assert_eq!(result, hex!("12 34 78").to_vec());
        let result = super::u128_to_vec(0x00_12_34_78, 3, ByteOrder::Little);
        assert_eq!(result, hex!("78 34 12").to_vec());

        let result = super::u128_to_vec(0x12_34_00_78, 4, ByteOrder::Big);
        assert_eq!(result, hex!("12 34 00 78").to_vec());
        let result = super::u128_to_vec(0x12_34_00_78, 4, ByteOrder::Little);
        assert_eq!(result, hex!("78 00 34 12").to_vec());

        Ok(())
    }

    #[test]
    fn test_vec_to_u128() -> anyhow:: Result<()> {
        let result = super::slice_to_u128(hex!("78 56 34 12").as_slice(), ByteOrder::Little);
        assert_eq!(result, 0x12_34_56_78);
        let result = super::slice_to_u128(hex!("12 34 56 78").as_slice(), ByteOrder::Big);
        assert_eq!(result, 0x12_34_56_78);

        Ok(())
    }

    #[test]
    fn test_u128_to_vec_fix() -> anyhow:: Result<()> {
        let result = super::u128_to_vec_fix(0x00_12_34_78, ByteOrder::Big);
        assert_eq!(result, hex!("12 34 78").to_vec());

        Ok(())
    }
}
