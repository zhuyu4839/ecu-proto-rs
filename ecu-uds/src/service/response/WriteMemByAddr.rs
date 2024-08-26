use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::service::{Configuration, MemoryLocation, Placeholder, ResponseData};
use crate::service::response::Code;

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

pub struct WriteMemByAddrData(pub MemoryLocation);

impl<'a> TryFrom<&'a [u8]> for WriteMemByAddrData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self(MemoryLocation::try_from(data)?))
    }
}

impl ResponseData for WriteMemByAddrData {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
}

impl Into<Vec<u8>> for WriteMemByAddrData {
    fn into(self) -> Vec<u8> {
        self.0.into()
    }
}
