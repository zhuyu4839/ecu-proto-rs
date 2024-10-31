use std::fmt::{Display, LowerHex, Write};
use isotp_rs::ByteOrder;
use crate::Error;

#[inline]
pub(crate) fn err_msg<T: Display + LowerHex>(v: T) -> String {
    format!("the value {0:x} is invalid or ISO/SAE reserved", v)
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
    use isotp_rs::ByteOrder;

    #[test]
    fn test_u128_to_vec() -> anyhow::Result<()> {
        let result = super::u128_to_vec(0x00_12_34_78, 3, ByteOrder::Big);
        assert_eq!(result, hex::decode("123478")?);
        let result = super::u128_to_vec(0x00_12_34_78, 3, ByteOrder::Little);
        assert_eq!(result, hex::decode("783412")?);

        let result = super::u128_to_vec(0x12_34_00_78, 4, ByteOrder::Big);
        assert_eq!(result, hex::decode("12340078")?);
        let result = super::u128_to_vec(0x12_34_00_78, 4, ByteOrder::Little);
        assert_eq!(result, hex::decode("78003412")?);

        Ok(())
    }

    #[test]
    fn test_vec_to_u128() -> anyhow:: Result<()> {
        let result = super::slice_to_u128(&hex::decode("78563412")?, ByteOrder::Little);
        assert_eq!(result, 0x12_34_56_78);
        let result = super::slice_to_u128(&hex::decode("12345678")?, ByteOrder::Big);
        assert_eq!(result, 0x12_34_56_78);

        Ok(())
    }

    #[test]
    fn test_u128_to_vec_fix() -> anyhow:: Result<()> {
        let result = super::u128_to_vec_fix(0x00_12_34_78, ByteOrder::Big);
        assert_eq!(result, hex::decode("123478")?);

        Ok(())
    }
}
