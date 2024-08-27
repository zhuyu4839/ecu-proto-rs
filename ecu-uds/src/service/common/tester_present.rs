/// Commons of Service 3E

use crate::enum_to_vec;
use crate::error::Error;

enum_to_vec!(
    pub enum TesterPresentType {
        Zero = 0x00,
    }, u8, Error, InvalidParam
);

