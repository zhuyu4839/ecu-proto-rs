mod constant;
pub use constant::*;

mod driver;
pub use driver::*;

mod frame;
pub use frame::*;
mod identifier;
pub use identifier::*;

mod isotp;
pub use isotp::*;

mod utils;

use crate::{IsoTpError, FlowControlContext, FlowControlState, FrameType, IsoTpFrame};

/// ISO-TP address format.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressFormat {
    // UNKNOWN = 0xFF,
    // None = 0x00,
    #[default]
    Normal = 0x01,      // 11bit CAN-ID
    NormalFixed = 0x02, // 29bit CAN-ID
    Extend = 0x03,      // 11bit Remote CAN-ID
    ExtendMixed = 0x04, // 11bit and 11bit Remote CAN-ID mixed
    Enhanced = 0x05,    // 11bit(Remote) and 29bot CAN-ID
}

/// ISO-TP address
///
/// * `tx_id`: transmit identifier.
/// * `rx_id`: receive identifier.
/// * `fid`: functional address identifier.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Address {
    pub tx_id: u32,
    pub rx_id: u32,
    pub fid: u32,
}

/// ISO-TP address type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum AddressType {
    #[default]
    Physical,
    Functional,
}

/// ISO-TP frame define.
#[derive(Debug, Clone)]
pub enum CanIsoTpFrame {
    /// The ISO-TP single frame.
    SingleFrame { data: Vec<u8> },
    /// The ISO-TP first frame.
    FirstFrame { length: u32, data: Vec<u8> },
    /// The ISO-TP consecutive frame.
    ConsecutiveFrame { sequence: u8, data: Vec<u8> },
    /// The ISO-TP flow control frame.
    FlowControlFrame(FlowControlContext)
}

impl<'a> From<&'a CanIsoTpFrame> for FrameType {
    fn from(value: &'a CanIsoTpFrame) -> Self {
        match value {
            CanIsoTpFrame::SingleFrame { .. } => Self::Single,
            CanIsoTpFrame::FirstFrame { .. } => Self::First,
            CanIsoTpFrame::ConsecutiveFrame { .. } => Self::Consecutive,
            CanIsoTpFrame::FlowControlFrame(_) => Self::FlowControl,
        }
    }
}

unsafe impl Send for CanIsoTpFrame {}

impl IsoTpFrame for CanIsoTpFrame {
    fn decode<T: AsRef<[u8]>>(data: T) -> Result<Self, IsoTpError> {
        let data = data.as_ref();
        let length = data.len();
        match length {
            0 => Err(IsoTpError::EmptyPdu),
            1..=2 => Err(IsoTpError::InvalidPdu(data.to_vec())),
            3.. => {
                let byte0 = data[0];
                match FrameType::try_from(byte0)? {
                    FrameType::Single => {   // Single frame
                        utils::decode_single(data, byte0, length)
                    },
                    FrameType::First => {   // First frame
                        utils::decode_first(data, byte0, length)
                    },
                    FrameType::Consecutive => {
                        let sequence = byte0 & 0x0F;
                        Ok(Self::ConsecutiveFrame { sequence, data: Vec::from(&data[1..]) })
                    },
                    FrameType::FlowControl => {
                        // let suppress_positive = (data1 & 0x80) == 0x80;
                        let state = FlowControlState::try_from(byte0 & 0x0F)?;
                        let fc = FlowControlContext::new(state, data[1], data[2])?;
                        Ok(Self::FlowControlFrame(fc))
                    },
                }
            }
            // v => Err(IsoTpError::LengthOutOfRange(v)),
        }
    }

    fn encode(self, padding: Option<u8>) -> Vec<u8> {
        match self {
            Self::SingleFrame { data } => {
                utils::encode_single(data, padding)
            },
            Self::FirstFrame { length, data } => {
                utils::encode_first(length, data)
            },
            Self::ConsecutiveFrame { sequence, mut data } => {
                let mut result = vec![FrameType::Consecutive as u8 | sequence];
                result.append(&mut data);
                result.resize(CAN_FRAME_MAX_SIZE, padding.unwrap_or(DEFAULT_PADDING));
                result
            },
            Self::FlowControlFrame(context) => {
                let byte0_h: u8 = FrameType::FlowControl.into();
                let byte0_l: u8 = context.state().into();
                let mut result = vec![
                    byte0_h | byte0_l,
                    context.block_size(),
                    context.st_min(),
                ];
                result.resize(CAN_FRAME_MAX_SIZE, padding.unwrap_or(DEFAULT_PADDING));
                result
            },
        }
    }

    fn from_data<T: AsRef<[u8]>>(data: T) -> Result<Vec<Self>, IsoTpError> {
        utils::from_data(data.as_ref())
    }

    fn single_frame<T: AsRef<[u8]>>(data: T) -> Result<Self, IsoTpError> {
        utils::new_single(data)
    }

    fn flow_ctrl_frame(state: FlowControlState,
                       block_size: u8,
                       st_min: u8,
    ) -> Result<Self, IsoTpError> {
        Ok(Self::FlowControlFrame(
            FlowControlContext::new(state, block_size, st_min)?
        ))
    }
}
