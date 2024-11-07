//! response of Service 2C

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, DefinitionType, DynamicallyDID, UdsError, Placeholder, response::Code, ResponseData, Service};
use crate::response::{Response, SubFunction};

lazy_static!(
    pub static ref DYNAMICAL_DID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DynamicallyDefineDID(pub Option<DynamicallyDID>);

impl<'a> TryFrom<&'a [u8]> for DynamicallyDefineDID {
    type Error = UdsError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let offset = 0;

        let dynamic = match data_len {
            0 => Ok(None),
            2 => Ok(Some(DynamicallyDID::try_from(
                u16::from_be_bytes([data[offset], data[offset + 1]])
            )?)),
            v => Err(UdsError::InvalidDataLength { expect: 2, actual: v })
        }?;

        Ok(Self(dynamic))
    }
}

impl Into<Vec<u8>> for DynamicallyDefineDID {
    fn into(self) -> Vec<u8> {
        match self.0 {
            Some(v) => v.into(),
            None => vec![],
        }
    }
}

impl ResponseData for DynamicallyDefineDID {
    type SubFunc = DefinitionType;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_none() {
            return Err(UdsError::SubFunctionError(Service::DynamicalDefineDID));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn dyn_define_did(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let sf = DefinitionType::try_from(sub_func.unwrap().0)?;
    let _ = DynamicallyDefineDID::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
