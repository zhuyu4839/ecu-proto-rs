use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::service::response::Code;

lazy_static!(
    pub static ref TRANSFER_DATA_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::TransferDataSuspended,
        Code::GeneralProgrammingFailure,
        Code::WrongBlockSequenceCounter,
        Code::VoltageTooHigh,
        Code::VoltageTooLow,
    ]);
);
