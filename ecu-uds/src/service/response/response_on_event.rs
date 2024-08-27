/// 0x86

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::service::{Configuration, Placeholder, ResponseData};
use crate::service::response::Code;

lazy_static!(
    pub static ref RESPONSE_ON_EVENT_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
);

#[derive(Debug, Clone)]
pub struct ResponseOnEvent {
    
}

#[allow(unused_variables)]
impl<'a> TryFrom<&'a [u8]> for ResponseOnEvent {
    type Error = Error;
    // #[deprecated(since = "0.1.0", note = "This library does not yet support")]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        return Err(Error::OtherError("This library does not yet support".to_string()))
    }
}

impl Into<Vec<u8>> for ResponseOnEvent {
    // #[deprecated(since = "0.1.0", note = "This library does not yet support")]
    fn into(self) -> Vec<u8> {
        panic!("This library does not yet support")
    }
}

impl ResponseData for ResponseOnEvent {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
