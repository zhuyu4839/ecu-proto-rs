//! response of Service 37


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::response::Code;

lazy_static!(
    pub static ref REQUEST_TRANSFER_EXIT_NEGATIVES: HashSet<Code>
    = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::GeneralProgrammingFailure,
    ]);
);
