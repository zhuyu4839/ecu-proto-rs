use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// Bitfield representation of 3-bit `COMM_MODE_PGM parameter in PROGRAM_START`
///
/// ### Repr: `u8`
///
/// | Field               | Size (bits) |
/// |---------------------|-------------|
/// | Reserved            | 1           |
/// | Slave Block Mode    | 1           |
/// | Reserved            | 4           |
/// | Interleaved Mode    | 1           |
/// | Master Block Mode   | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct ProgrammingMode {
    __: bool,
    pub slave_block_mode: bool,
    #[bits(4)]
    __: u8,
    pub interleaved_mode: bool,
    pub master_block_mode: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct ProgramStart {
    #[getset(skip)]
    reserved: u8,
    pub(crate) mode: ProgrammingMode,
    pub(crate) max_cto: u8,
    pub(crate) max_bs: u8,
    pub(crate) st_min: u8,
    pub(crate) queue_size: u8,
}

impl ProgramStart {
    pub fn new(
        mode: ProgrammingMode,
        max_cto: u8,
        max_bs: u8,
        st_min: u8,
        queue_size: u8,
    ) -> Self {
        Self {
            reserved: Default::default(),
            mode,
            max_cto,
            max_bs,
            st_min,
            queue_size,
        }
    }

    pub const fn length() -> usize {
        6
    }
}

impl Into<Vec<u8>> for ProgramStart {
    fn into(self) -> Vec<u8> {
        vec![
            self.reserved,
            self.mode.into(),
            self.max_cto,
            self.max_bs,
            self.st_min,
            self.queue_size,
        ]
    }
}

impl TryFrom<&[u8]> for ProgramStart {
    type Error = XcpError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 1; // skip reserved
        let mode = ProgrammingMode::from(data[offset]);
        offset += 1;
        let max_cto = data[offset];
        offset += 1;
        let max_bs = data[offset];
        offset += 1;
        let st_min = data[offset];
        offset += 1;
        let queue_size = data[offset];

        Ok(Self::new(mode, max_cto, max_bs, st_min, queue_size))
    }
}
