//! response of Service 14


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::response::Code;

lazy_static!(
    pub static ref CLEAR_DIAGNOSTIC_INFO_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        Code::GeneralProgrammingFailure,
    ]);
);

