//! response of Service 23


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::response::Code;

lazy_static!(
    pub static ref READ_MEM_BY_ADDR_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ResponseTooLong,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ]);
);
