#![allow(unused_imports, dead_code)]

#[cfg(feature = "std2004")]
mod std2004;
#[cfg(feature = "std2004")]
pub(crate) use std2004::*;
#[cfg(feature = "std2016")]
mod std2016;
#[cfg(feature = "std2016")]
pub(crate) use std2016::*;


use crate::can::{CanIsoTpFrame, constant::{CAN_FRAME_MAX_SIZE, CONSECUTIVE_FRAME_SIZE}};

#[inline]
fn can_fd_resize(length: usize) -> Option<usize> {
    match length {
        ..=CAN_FRAME_MAX_SIZE => Some(length),
        9..=12 =>  Some(12),
        13..=16 => Some(16),
        17..=20 => Some(20),
        21..=24 => Some(24),
        25..=32 => Some(32),
        33..=48 => Some(48),
        49..=64 => Some(64),
        _ => None,
    }
}

fn parse<const FIRST_FRAME_SIZE: usize>(data: &[u8],
                                        offset: &mut usize,
                                        sequence: &mut u8,
                                        results: &mut Vec<CanIsoTpFrame>,
                                        length: usize,
) {
    loop {
        match *offset {
            0 => {
                *offset += FIRST_FRAME_SIZE;
                let frame = CanIsoTpFrame::FirstFrame {
                    length: length as u32,
                    data: Vec::from(&data[..*offset])
                };
                results.push(frame);

                continue;
            },
            _ => {
                if *offset + CONSECUTIVE_FRAME_SIZE >= length {
                    let frame = CanIsoTpFrame::ConsecutiveFrame {
                        sequence: *sequence,
                        data: Vec::from(&data[*offset..length])
                    };
                    results.push(frame);
                    break;
                }

                let frame = CanIsoTpFrame::ConsecutiveFrame {
                    sequence: *sequence,
                    data: Vec::from(&data[*offset..*offset + CONSECUTIVE_FRAME_SIZE])
                };
                *offset += CONSECUTIVE_FRAME_SIZE;
                if *sequence >= 0x0F {
                    *sequence = 0;
                }
                else {
                    *sequence += 1;
                }

                results.push(frame);
            }
        }
    }
}

