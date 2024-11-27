mod c0;
pub use c0::*;
// mod cal;
// pub use cal::*;
mod daq;
pub use daq::*;
mod page;
pub use page::*;
mod pgm;
pub use pgm::*;
mod standard;
pub use standard::*;
pub mod negative;

use getset::{CopyGetters, Getters};
use crate::{ErrorCode, XcpError};

#[derive(Debug, Clone)]
pub enum Response {
    Positive = 0xFF,
    Negative = 0xFE,
    Event = 0xFD,
}

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct Positive<T: Into<Vec<u8>> + for<'a> TryFrom<&'a [u8], Error = XcpError>> {
    pub(crate) data: T,
}

#[derive(Debug, Clone, Getters, CopyGetters, )]
pub struct Negative {
    #[get_copy = "pub"]
    pub(crate) code: ErrorCode,
    #[get = "pub"]
    pub(crate) data: Vec<u8>,
}

impl Negative {
    #[inline]
    pub fn origin_data<T>(&self) -> Result<T, XcpError>
    where
        T: for<'a> TryFrom<&'a [u8], Error = XcpError>
    {
        T::try_from(self.data.as_slice())
    }
}
