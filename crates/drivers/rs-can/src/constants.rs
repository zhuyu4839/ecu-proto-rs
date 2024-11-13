/// Mask for standard identifiers.
pub const SFF_MASK: u32 = 0x0000_07FF;

/// Mask for extended identifiers.
pub const EFF_MASK: u32 = 0x1FFF_FFFF;
/// The max sizeof can-frame's data.
pub const CAN_FRAME_MAX_SIZE: usize = 8;
/// The max sizeof canfd-frame's data.
pub const CANFD_FRAME_MAX_SIZE: usize = 64;
/// Default padding value(0b1010_1010).
pub const DEFAULT_PADDING: u8 = 0xAA;
