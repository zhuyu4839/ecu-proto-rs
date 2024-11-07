mod error;
pub use error::*;
pub mod client;
pub mod server;

use iso15765_2::constant::{P2_ISO14229, P2_STAR_ISO14229};

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
