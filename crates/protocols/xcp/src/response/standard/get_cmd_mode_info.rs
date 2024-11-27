use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::XcpError;

/// Bitfield representation of 2-bit `COMM_MODE_OPTIONAL parameter in GET_COMM_MODE_INFO`
///
/// ### Repr: `u8`
///
/// | Field               | Size (bits) |
/// |---------------------|-------------|
/// | Reserved            | 6           |
/// | Interleaved Mode    | 1           |
/// | Master Block Mode   | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct CmdModeOpt {
    #[bits(6)]
    __: u8,
    pub interleaved_mode: bool,
    pub master_block_mode: bool,
}

#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct GetCmdModeInfo {
    #[getset(skip)]
    reserved0: u8,
    pub(crate) cmd_mode_opt: CmdModeOpt,
    #[getset(skip)]
    reserved1: u8,
    pub(crate) max_bs: u8,
    pub(crate) min_st: u8,
    pub(crate) queue_size: u8,
    pub(crate) driver_version: u8,
}

impl GetCmdModeInfo {
    pub fn new(cmd_mode_opt: CmdModeOpt, max_bs: u8, min_st: u8, q_size: u8, drv_ver: u8) -> Self {
        Self {
            reserved0: Default::default(),
            cmd_mode_opt,
            reserved1: Default::default(),
            max_bs,
            min_st,
            queue_size: q_size,
            driver_version: drv_ver,
        }
    }

    #[inline(always)]
    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for GetCmdModeInfo {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(Self::length());
        result.push(self.reserved0);
        result.push(self.cmd_mode_opt.into());
        result.push(self.reserved1);
        result.push(self.max_bs);
        result.push(self.min_st);
        result.push(self.queue_size);
        result.push(self.driver_version);

        result
    }
}

impl TryFrom<&[u8]> for GetCmdModeInfo {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 1;
        let cmd_mode_opt = CmdModeOpt::from(data[offset]);
        offset += 1;
        offset += 1;
        let max_block_size = data[offset];
        offset += 1;
        let min_st = data[offset];
        offset += 1;
        let queue_size = data[offset];
        offset += 1;
        let driver_version = data[offset];

        Ok(Self::new(cmd_mode_opt, max_block_size, min_st, queue_size, driver_version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cmd_mode_info() -> anyhow::Result<()> {
        let response = GetCmdModeInfo::new(
            CmdModeOpt::new()
                .with_interleaved_mode(true)
                .with_master_block_mode(true),
            0x10,
            0x01,
            0x20,
            0x00,
        );
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x00, 0x03, 0x00, 0x10, 0x01, 0x20, 0x00]);

        let response = GetCmdModeInfo::try_from(data.as_slice())?;
        let cmd_mode_opt = response.cmd_mode_opt;
        assert!(cmd_mode_opt.interleaved_mode());
        assert!(cmd_mode_opt.master_block_mode());
        assert_eq!(response.max_bs, 0x10);
        assert_eq!(response.min_st, 0x01);
        assert_eq!(response.queue_size, 0x20);
        assert_eq!(response.driver_version, 0x00);

        Ok(())
    }
}
