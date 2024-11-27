mod c0;
pub use c0::*;
mod daq;
pub use daq::*;
mod page;
pub use page::*;
mod pgm;
pub use pgm::*;
mod standard;
pub use standard::*;
mod positive;
pub use positive::*;
pub mod negative;
pub use negative::Negative;

use crate::{constants::{EVENT_CODE, NEGATIVE_CODE, POSITIVE_CODE}, event::Event, XcpError};

#[derive(Debug, Clone)]
pub enum Response {
    Positive(Positive),
    Negative(Negative),
    Event(Event),
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        match self {
            Response::Positive(x) => {
                let mut result = vec![POSITIVE_CODE, ];
                result.append(&mut x.into());
                result
            },
            Response::Event(e) => {
                let mut result = vec![EVENT_CODE, ];
                result.append(&mut e.into());
                result
            },
            Response::Negative(x) => {
                let mut result = vec![NEGATIVE_CODE, ];
                result.append(&mut x.into());
                result
            },
        }
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = 1;
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        match data[0] {
            POSITIVE_CODE => Ok(Self::Positive(Positive::from(&data[expected..]))),
            EVENT_CODE => Ok(Self::Event(Event::try_from(&data[expected..])?)),
            NEGATIVE_CODE => Ok(Self::Negative(Negative::try_from(data)?)),
            v => Err(XcpError::UnexpectedResponse(v)),
        }
    }
}
