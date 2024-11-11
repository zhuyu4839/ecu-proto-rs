/// start sequence of consecutive.
pub const CONSECUTIVE_SEQUENCE_START: u8 = 0x01;
/// MAX st_min(127ms).
pub const MAX_ST_MIN: u8 = 0x7F;

/// Default P2(50ms) of ISO 14229
pub const P2_ISO14229: u16 = 50;
/// Default P2*(5000ms) of ISO 14229
pub const P2_STAR_ISO14229: u32 = 5_000;
/// Default value for Separation time
pub const ST_MIN_ISO15765_2: u8 = 10;
/// Default value for BlockSize
pub const BS_ISO15765_2: u8 = 10;
/// OBD-II value for Separation time
pub const ST_MIN_ISO15765_4: u8 = 0;
/// OBD-II value for BlockSize
pub const BS_ISO15765_4: u8 = 0;
/// Default value for Timeout Ar in ms
pub const TIMEOUT_AR_ISO15765_2: u32 = 1000;
/// Default value for Timeout As in ms
pub const TIMEOUT_AS_ISO15765_2: u32 = 1000;
/// Default value for Timeout Br in ms
pub const TIMEOUT_BR_ISO15765_2: u32 = 1000;
/// Default value for Timeout Bs in ms
pub const TIMEOUT_BS_ISO15765_2: u32 = 1000;
/// Default value for Timeout Cr in ms
pub const TIMEOUT_CR_ISO15765_2: u32 = 1000;
/// Default value for Timeout Cs in ms
pub const TIMEOUT_CS_ISO15765_2: u32 = 1000;

/// OBD-II value for Timeout Ar in ms
pub const TIMEOUT_AR_ISO15765_4: u32 = 33;
/// OBD-II value for Timeout As in ms
pub const TIMEOUT_AS_ISO15765_4: u32 = 33;
/// OBD-II value for Timeout Br in ms
pub const TIMEOUT_BR_ISO15765_4: u32 = 75;
/// OBD-II value for Timeout Bs in ms
pub const TIMEOUT_BS_ISO15765_4: u32 = 75;
/// OBD-II value for Timeout Cr in ms
pub const TIMEOUT_CR_ISO15765_4: u32 = 150;
/// OBD-II value for Timeout Cs in ms (Cs+As < 50ms)
pub const TIMEOUT_CS_ISO15765_4: u32 = 17;
