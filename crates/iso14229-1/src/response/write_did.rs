//! response of Service 2E

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{DataIdentifier, Error, response::Code, Service, utils, ResponseData, Placeholder, Configuration};
use crate::response::{Response, SubFunction};

lazy_static!(
    pub static ref WRITE_DID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::GeneralProgrammingFailure,
    ]);
);


pub struct WriteDID(pub DataIdentifier);

impl<'a> TryFrom<&'a [u8]> for WriteDID {
    type Error = Error;
    #[inline]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 2, true)?;
        let offset = 0;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );

        Ok(Self(did))
    }
}

impl Into<Vec<u8>> for WriteDID {
    #[inline]
    fn into(self) -> Vec<u8> {
        let did: u16 = self.0.into();

        did.to_be_bytes().to_vec()
    }
}

impl ResponseData for WriteDID {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        if sub_func.is_some() {
            return Err(Error::SubFunctionError(Service::WriteDID));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn write_did(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }
    
    let _ = WriteDID::try_parse(data.as_slice(), None, cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
