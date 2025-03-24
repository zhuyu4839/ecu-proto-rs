#![allow(unused)]

use rs_can::{MAX_FD_FRAME_SIZE, MAX_FRAME_SIZE};

#[cfg(not(feature = "can-fd"))]
pub const SINGLE_FRAME_SIZE_2004: usize = MAX_FRAME_SIZE - 1;
#[cfg(feature = "can-fd")]
pub const SINGLE_FRAME_SIZE_2004: usize = MAX_FD_FRAME_SIZE - 1;

#[cfg(not(feature = "can-fd"))]
pub const SINGLE_FRAME_SIZE_2016: usize = MAX_FRAME_SIZE - 2;
#[cfg(feature = "can-fd")]
pub const SINGLE_FRAME_SIZE_2016: usize = MAX_FD_FRAME_SIZE - 2;

#[cfg(not(feature = "can-fd"))]
pub const FIRST_FRAME_SIZE_2004: usize = MAX_FRAME_SIZE - 2;
#[cfg(feature = "can-fd")]
pub const FIRST_FRAME_SIZE_2004: usize = MAX_FD_FRAME_SIZE - 2;

#[cfg(not(feature = "can-fd"))]
pub const FIRST_FRAME_SIZE_2016: usize = MAX_FRAME_SIZE - 5;
#[cfg(feature = "can-fd")]
pub const FIRST_FRAME_SIZE_2016: usize = MAX_FD_FRAME_SIZE - 5;

#[cfg(not(feature = "can-fd"))]
pub const CONSECUTIVE_FRAME_SIZE: usize = MAX_FRAME_SIZE - 1;
#[cfg(feature = "can-fd")]
pub const CONSECUTIVE_FRAME_SIZE: usize = MAX_FD_FRAME_SIZE - 1;
