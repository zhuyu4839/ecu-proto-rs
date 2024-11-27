use bitfield_struct::bitfield;

/// Bitfield representation of 5-bit `RESOURCE parameter in CONNECT and GET_SEED`.
/// Bitfield representation of 5-bit `Current resource protection status in GET_STATUS`.
///
/// ### Repr: `u8`
///
/// | Field        | Size (bits) |
/// |--------------|-------------|
/// | Padding      | 2           |
/// | PGM          | 1           |
/// | STIM         | 1           |
/// | DAQ          | 1           |
/// | Reserved     | 1           |
/// | CAL/PAGE     | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct ResourceStatus {
    #[bits(2)]
    __: u8,
    pub debugging: bool,
    pub programming: bool,
    /// STIMulation
    pub stim: bool,
    /// DAQ lists supported
    pub daq: bool,
    __: bool,
    /// CALibration and PAGing
    pub cal_and_page: bool,
}
