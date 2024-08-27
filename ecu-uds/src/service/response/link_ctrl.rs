/// Service(0x87) 0xC7

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::service::response::Code;

lazy_static!(
    pub static ref LINK_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
    ]);
);
