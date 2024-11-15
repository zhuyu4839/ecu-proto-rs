mod error;
pub use error::*;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::*;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;

pub(crate) mod buffer;

/// ISO-TP address format.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressFormat {
    // UNKNOWN = 0xFF,
    // None = 0x00,
    #[default]
    Normal = 0x01,      // 11bit CAN-ID
    NormalFixed = 0x02, // 29bit CAN-ID
    Extend = 0x03,      // 11bit Remote CAN-ID
    ExtendMixed = 0x04, // 11bit and 11bit Remote CAN-ID mixed
    Enhanced = 0x05,    // 11bit(Remote) and 29bot CAN-ID
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
pub type SecurityAlgo = fn(u8, Vec<u8>, Vec<u8>) -> Result<Option<Vec<u8>>, DoCanError>;
