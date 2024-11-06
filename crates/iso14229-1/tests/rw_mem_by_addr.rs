//! Service 23 | 3D

#[cfg(test)]
mod tests {
    use iso14229_1::{request, AddressAndLengthFormatIdentifier, Configuration, MemoryLocation, RequestData};

    #[test]
    fn test_read_mem_by_addr_request() -> anyhow::Result<()> {
        let source = hex::decode("2312481305")?;
        let cfg = Configuration::default();
        let request = request::ReadMemByAddr(
            MemoryLocation::new(AddressAndLengthFormatIdentifier::new(2, 1)?, 0x4813,0x05,)?);
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[1..].to_vec());

        let cfg = Configuration::default();
        let request = request::ReadMemByAddr::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(request.0.memory_address(), 0x4813);
        assert_eq!(request.0.memory_size(), 0x05);

        let source = hex::decode("2324204813920103")?;
        let request = request::ReadMemByAddr(
            MemoryLocation::new(AddressAndLengthFormatIdentifier::new(4, 2)?,0x20481392,0x0103,)?);
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[1..].to_vec());

        let request = request::ReadMemByAddr::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(request.0.memory_address(), 0x20481392);
        assert_eq!(request.0.memory_size(), 0x0103);

        Ok(())
    }

    #[test]
    fn test_write_mem_by_addr_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("3D4420481213000000051122334455")?;
        let request = request::WriteMemByAddr::new(
            AddressAndLengthFormatIdentifier::new(4, 4)?,
            0x20481213,
            0x05,
            hex::decode("1122334455")?,
        )?;
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[1..].to_vec());

        let request = request::WriteMemByAddr::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(request.mem_loc.memory_address(), 0x20481213);
        assert_eq!(request.mem_loc.memory_size(), 0x05);
        assert_eq!(request.data, hex::decode("1122334455")?);

        Ok(())
    }
}
