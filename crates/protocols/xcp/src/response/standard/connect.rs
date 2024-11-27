//! page 110

use bitfield_struct::bitfield;
use getset::CopyGetters;
use crate::{AddressGranularity, ResourceStatus, XcpError};


/// Bitfield representation of 5-bit `COMM_MODE_BASIC parameter in CONNECT`
///
/// ### Repr: `u8`
///
/// | Field               | Size (bits) |
/// |---------------------|-------------|
/// | Optional            | 1           |
/// | SLAVE_BLOCK_MODE    | 1           |
/// | Reserved            | 3           |
/// | ADDRESS GRANULARITY | 2           |
/// | ByteOrder           | 1           |
#[bitfield(u8, order = Msb, conversion = false)]
#[derive(PartialEq, Eq)]
pub struct CommonMode {
    pub optional: bool,
    pub slave_block_mode: bool,
    #[bits(3)]
    __: u8,
    #[bits(2)]
    pub addr_granularity: u8,   // AddressGranularity
    pub byte_order: bool,
}

/// CONNECT positive response structure
///
/// | Position | Type | Description |
/// | -------- | ---- | ----------- |
/// | 1 | BYTE | RESOURCE |
/// | 2 | BYTE | COMM_MODE_BASIC |
/// | 3 | BYTE | MAX_CTO, Maximum CTO size [BYTE] |
/// | 4 | WORD | MAX_DTO, Maximum DTO size [BYTE] |
/// | 6 | BYTE | XCP Protocol Layer Version Number(most significant byte only) |
/// | 7 | BYTE | XCP Transport Layer Version Number(most significant byte only) |
#[derive(Debug, Clone, CopyGetters)]
#[get_copy = "pub"]
pub struct Connect {
    pub(crate) resource_status: ResourceStatus,
    pub(crate) comm_mode: CommonMode,
    pub(crate) max_cto: u8,
    pub(crate) max_dto: u16,
    pub(crate) protocol_version: u8,
    pub(crate) transport_version: u8,
}

impl Connect {
    pub fn new(
        resource_status: ResourceStatus,
        comm_mode: CommonMode,
        max_cto: u8,
        max_dto: u16,
        protocol_version: u8,
        transport_version: u8,
    ) -> Self {
        Self {
            resource_status,
            comm_mode,
            max_cto,
            max_dto,
            protocol_version,
            transport_version,
        }
    }

    #[inline]
    pub const fn length() -> usize {
        7
    }
}

impl Into<Vec<u8>> for Connect {
    fn into(self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::with_capacity(Self::length());
        result.push(self.resource_status.into());
        result.push(self.comm_mode.into());
        result.push(self.max_cto);
        result.extend(self.max_dto.to_be_bytes());
        result.push(self.protocol_version);
        result.push(self.transport_version);

        result
    }
}

impl TryFrom<&[u8]> for Connect {
    type Error = XcpError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        let expected = Self::length();
        if data_len < expected {
            return Err(XcpError::InvalidDataLength { expected, actual: data_len });
        }

        let mut offset = 0;
        let resource_status = ResourceStatus::from(data[offset]);
        offset += 1;
        let comm_mode = CommonMode::from(data[offset]);
        offset += 1;
        let max_cto = data[offset];
        offset += 1;
        let max_dto = u16::from_be_bytes([data[offset], data[offset+1]]);
        offset += 2;
        let protocol_version = data[offset];
        offset += 1;
        let transport_version = data[offset];

        Ok(Self::new(resource_status, comm_mode, max_cto, max_dto, protocol_version, transport_version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() -> anyhow::Result<()> {
        let resource = ResourceStatus::default()
            .with_cal_and_page(true)
            .with_daq(true);
        let cmd_mode = CommonMode::default()
            .with_addr_granularity(AddressGranularity::DWord.into())
            .with_byte_order(false);
        let max_cto = 0x07;
        let max_dto = 0x10;
        let protocol_version = 0x00;
        let transport_version = 0x01;

        let connect = Connect::new(resource, cmd_mode, max_cto, max_dto, protocol_version, transport_version);
        let data: Vec<_> = connect.into();
        assert_eq!(data, hex::decode("05040700100001")?);

        let connect = Connect::try_from(data.as_slice())?;
        let resource = connect.resource_status;
        assert!(resource.cal_and_page());
        assert!(resource.daq());
        assert!(!resource.stim());
        assert!(!resource.programming());
        assert!(!resource.debugging());

        let comm_mode = connect.comm_mode();
        assert_eq!(comm_mode.byte_order(), false);
        assert_eq!(comm_mode.addr_granularity(), AddressGranularity::DWord.into());
        assert_eq!(comm_mode.slave_block_mode(), false);
        assert_eq!(comm_mode.optional(), false);

        assert_eq!(connect.max_cto(), 0x07);
        assert_eq!(connect.max_dto(), 0x10);
        assert_eq!(connect.protocol_version(), 0x00);
        assert_eq!(connect.transport_version(), 0x01);

        Ok(())
    }
}
