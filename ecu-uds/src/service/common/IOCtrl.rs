/// Commons of Service 2F

use crate::enum_to_vec;
use crate::error::Error;

enum_to_vec!(
pub enum IOCtrlParameter {
    ReturnControlToEcu = 0x00,
    ResetToDefault = 0x01,
    FreezeCurrentState = 0x02,
    ShortTermAdjustment = 0x03,
}, u8, Error, InvalidParam);

// #[repr(u8)]
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum IOCtrlParameter {
//     ReturnControlToEcu = 0x00,
//     ResetToDefault = 0x01,
//     FreezeCurrentState = 0x02,
//     ShortTermAdjustment = 0x03,
//     Reserved(u8),
// }
//
// impl From<u8> for IOCtrlParameter {
//     fn from(value: u8) -> Self {
//         match value {
//             0x00 => Self::ReturnControlToEcu,
//             0x01 => Self::ResetToDefault,
//             0x02 => Self::FreezeCurrentState,
//             0x03 => Self::ShortTermAdjustment,
//             _ => Self::Reserved(value),
//         }
//     }
// }
//
// impl Into<u8> for IOCtrlParameter {
//     fn into(self) -> u8 {
//         match self {
//             IOCtrlParameter::ReturnControlToEcu => 0x00,
//             IOCtrlParameter::ResetToDefault => 0x01,
//             IOCtrlParameter::FreezeCurrentState => 0x02,
//             IOCtrlParameter::ShortTermAdjustment => 0x03,
//             IOCtrlParameter::Reserved(v) => v,
//         }
//     }
// }

#[derive(Debug, Clone,  Eq, PartialEq)]
pub struct IOCtrlOption {
    pub param: IOCtrlParameter,
    pub state: Vec<u8>,
}

