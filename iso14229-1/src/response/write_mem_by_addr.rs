//! response of Service 3D


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, error::Iso14229Error, MemoryLocation, response::{Code, Response, SubFunction}, ResponseData, Service, utils};

lazy_static!(
    pub static ref WRITE_MEM_BY_ADDR_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
        Code::GeneralProgrammingFailure,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WriteMemByAddr(pub MemoryLocation);

impl ResponseData for WriteMemByAddr {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::WriteMemByAddr)),
            None => {
                utils::data_length_check(data.len(), 3, false)?;

                Ok(Response {
                    service: Service::WriteMemByAddr,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(response: &Response, cfg: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::WriteMemByAddr
            || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service))
        }

        Ok(Self(MemoryLocation::from_slice(&response.data, cfg)?))
    }

    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        self.0.to_vec(cfg)
    }
}
