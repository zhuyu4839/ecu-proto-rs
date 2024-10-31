//! response of Service 3D


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, error::Error, MemoryLocation, Placeholder, response::Code, ResponseData};

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

pub struct WriteMemByAddr(pub MemoryLocation);

impl ResponseData for WriteMemByAddr {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        Ok(Self(MemoryLocation::from_slice(data, cfg)?))
    }
    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        self.0.to_vec(cfg)
    }
}
