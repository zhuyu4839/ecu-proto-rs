use crate::{ByteOrder, Iso14229Error, SUPPRESS_POSITIVE};

/// Add to_vector function and
/// implement `Debug`, `Copy`, `Clone`, `Eq`, `PartialEq`,
/// `Hash`, `TryFrom`, `Into` trait for enum.
///
/// Example:
/// ```rust
/// use iso14229_1::{enum_extend, Iso14229Error};
///
/// enum_extend!(
///     pub enum AccessType {
///         ReadExtendedTimingParameterSet = 1,
///         SetTimingParametersToDefaultValues = 2,
///         ReadCurrentlyActiveTimingParameters = 3,
///         SetTimingParametersToGivenValues = 4,
///     }, u8
/// );
///
/// let demo: u8 = AccessType::ReadExtendedTimingParameterSet.into();
/// assert_eq!(demo, 1);
/// let demo = AccessType::try_from(1).unwrap();
/// assert_eq!(demo, AccessType::ReadExtendedTimingParameterSet);
/// ```
#[macro_export]
macro_rules! enum_extend {
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:expr
            ),* $(,)?
        },
        $value_type:ty
    ) => {
        $(#[$meta])*
        #[repr($value_type)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        $vis enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant = $value,
            )*
        }

        impl Into<$value_type> for $enum_name {
            #[inline]
            fn into(self) -> $value_type {
                match self {
                    $(
                        $(#[$variant_meta])*
                        Self::$variant => $value,
                    )*
                }
            }
        }

        impl TryFrom<$value_type> for $enum_name {
            type Error = Iso14229Error;
            #[inline]
            fn try_from(value: $value_type) -> Result<Self, Self::Error> {
               match value {
                    $(
                        $(#[$variant_meta])*
                        $value => Ok(Self::$variant),
                    )*
                    _ => Err(Iso14229Error::ReservedError(value.to_string()))
                }
            }
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct U24(pub(crate) u32);

impl U24 {
    #[inline]
    pub fn new(val: u32) -> Self {
        Self(val)
    }
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
    type Error = Iso14229Error;

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

#[inline]
pub(crate) fn data_length_check(actual: usize, expect: usize, equal: bool) -> Result<(), Iso14229Error> {
    if equal {
        if actual != expect {
            return Err(Iso14229Error::InvalidDataLength { expect, actual });
        }
    }
    else {
        if actual < expect {
            return Err(Iso14229Error::InvalidDataLength { expect, actual });
        }
    }

    Ok(())
}

#[inline]
fn is_big_endian() -> bool {
    1u16.to_ne_bytes()[0] == 0
}

/// used only enable std2020 feature
#[allow(unused)]
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

#[inline]
pub fn peel_suppress_positive(value: u8) -> (bool, u8) {
    ((value & SUPPRESS_POSITIVE) == SUPPRESS_POSITIVE, value & 0x7F)
}
