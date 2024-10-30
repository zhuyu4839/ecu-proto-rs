mod constant;
pub mod error;
pub mod utils;
pub mod service;
pub mod docan;
pub mod doip;

use isotp_rs::constant::{P2_ISO14229, P2_STAR_ISO14229};
use error::Error;

#[derive(Debug, Clone, Copy)]
pub struct P2Context {
    pub(crate) p2: u16,        // ms
    pub(crate) p2_offset: u16, // ms
    pub(crate) p2_star: u32,   // ms
}

impl Default for P2Context {
    fn default() -> Self {
        Self {
            p2: P2_ISO14229,
            p2_offset: 0,
            p2_star: P2_STAR_ISO14229,
        }
    }
}

/// SecurityAlgo
///
/// # Params
///
/// #1 level of security
///
/// #2 seed
///
/// #3 salt or other params
///
/// # Return
///
/// if all seed is 0x00, return None
/// else all seed is not 0xFF return algo data,
/// otherwise return Error
pub type SecurityAlgo = fn(u8, Vec<u8>, Vec<u8>) -> Result<Option<Vec<u8>>, Error>;
