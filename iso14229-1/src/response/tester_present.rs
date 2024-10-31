//! response of Service 3E


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::response::Code;

lazy_static!(
    pub static ref TESTER_PRESENT_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
    ]);
);
