//! response of Service 86


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, error::Error, Placeholder, response::Code, ResponseData, Service};
use crate::response::{Response, SubFunction};

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
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        if sub_func.is_some() {
            return Err(Error::SubFunctionError(Service::ResponseOnEvent));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn response_on_event(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = ResponseOnEvent::try_parse(data.as_slice(), None, cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
