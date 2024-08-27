/// Service 28

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::service::response::Code;

lazy_static!(
    pub static ref COMMUNICATION_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
);
