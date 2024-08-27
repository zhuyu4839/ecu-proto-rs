/// Commons of Service 38

use crate::enum_to_vec;
use crate::error::Error;

enum_to_vec! (
    pub enum ModeOfOperation {
        AddFile = 0x01,
        DeleteFile = 0x02,
        ReplaceFile = 0x03,
        ReadFile = 0x04,
        ReadDir = 0x05,
        ResumeFile = 0x06,
    }, u8, Error, InvalidParam
);
