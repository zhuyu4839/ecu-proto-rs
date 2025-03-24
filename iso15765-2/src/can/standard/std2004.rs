#![allow(unused_imports)]

#[cfg(feature = "can-fd")]
use rs_can::utils::can_dlc;
use rs_can::{MAX_FRAME_SIZE, MAX_FD_FRAME_SIZE, DEFAULT_PADDING};

use crate::can::constants::{CONSECUTIVE_FRAME_SIZE, FIRST_FRAME_SIZE_2004, SINGLE_FRAME_SIZE_2004};
use crate::constants::MAX_LENGTH_2004;
use crate::error::Error;
use crate::frame::{Frame, FrameType};

use super::parse_frame_util as parse;

pub(crate) fn decode_single(
    data: &[u8],
    byte0: u8,
    length: usize
) -> Result<Frame, Error> {
    #[cfg(feature = "can-fd")]
    let max_len = MAX_FD_FRAME_SIZE;
    #[cfg(not(feature = "can-fd"))]
    let max_len = MAX_FRAME_SIZE;

    if length > max_len {
        return Err(Error::LengthOutOfRange(length));
    }

    let pdu_len = byte0 & 0x0F;
    if length < pdu_len as usize + 1 {
        return Err(Error::InvalidPdu(Vec::from(data)));
    }

    Ok(Frame::SingleFrame { data: Vec::from(&data[1..=pdu_len as usize]) })
}

pub(crate) fn decode_first(
    data: &[u8],
    byte0: u8,
    length: usize,
) -> Result<Frame, Error> {
    #[cfg(not(feature = "can-fd"))]
    if length != MAX_FRAME_SIZE {
        return Err(Error::InvalidDataLength { actual: length, expect: MAX_FRAME_SIZE })
    }
    #[cfg(feature = "can-fd")]
    if length != MAX_FD_FRAME_SIZE {
        return Err(Error::InvalidDataLength { actual: length, expect: MAX_FD_FRAME_SIZE })
    }

    let pdu_len = (byte0 as u16 & 0x0F) << 8 | data[1] as u16;
    Ok(Frame::FirstFrame { length: pdu_len as u32, data: Vec::from(&data[2..]) })
}

pub(crate) fn encode_single(mut data: Vec<u8>, padding: Option<u8>) -> Vec<u8> {
    let length = data.len();
    let mut result = vec![FrameType::Single as u8 | length as u8];
    result.append(&mut data);
    #[cfg(not(feature = "can-fd"))]
    result.resize(MAX_FRAME_SIZE, padding.unwrap_or(DEFAULT_PADDING));
    #[cfg(feature = "can-fd")]
    if let Some(resize) = can_dlc(length, true) {
        result.resize(resize, padding.unwrap_or(DEFAULT_PADDING));
    }

    result
}

pub(crate) fn encode_first(length: u32, mut data: Vec<u8>) -> Vec<u8> {
    let len_h = ((length & 0x0F00) >> 8) as u8;
    let len_l = (length & 0x00FF) as u8;
    let mut result = vec![FrameType::First as u8 | len_h, len_l];
    result.append(&mut data);
    result
}

pub fn new_single<T: AsRef<[u8]>>(data: T) -> Result<Frame, Error> {
    let data = data.as_ref();
    let length = data.len();
    match length {
        0 => Err(Error::EmptyPdu),
        1..=SINGLE_FRAME_SIZE_2004 => {
            let mut result = vec![FrameType::Single as u8 | length as u8];
            result.append(&mut data.to_vec());
            result.resize(SINGLE_FRAME_SIZE_2004, DEFAULT_PADDING);
            Ok(Frame::SingleFrame { data: result })
        },
        v => Err(Error::LengthOutOfRange(v)),
    }
}

pub fn from_data(data: &[u8]) -> Result<Vec<Frame>, Error> {
    let length = data.len();
    match length {
        0 => Err(Error::EmptyPdu),
        1..=CONSECUTIVE_FRAME_SIZE => Ok(vec![Frame::SingleFrame { data: Vec::from(data) }]),
        ..=MAX_LENGTH_2004 => {
            let mut offset = 0;
            let mut sequence = 1;
            let mut results = Vec::new();

            parse::<FIRST_FRAME_SIZE_2004>(data, &mut offset, &mut sequence, &mut results, length);

            Ok(results)
        },
        v => Err(Error::LengthOutOfRange(v)),
    }
}
