mod get_daq_packed_mode;
pub use get_daq_packed_mode::*;
mod get_version;
pub use get_version::*;

#[cfg(test)]
mod tests {
    use crate::{DAQPackedMode, DAQPackedModeData, DPMTimestampMode};
    use super::*;

    #[test]
    fn test_get_version() -> anyhow::Result<()> {
        let response = GetVersion::new(
            Version::new()
                .with_major(0x10)
                .with_minor(0x10),
            Version::new()
                .with_major(0x01)
                .with_minor(0x01),
        );
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x00, 0x10, 0x10, 0x01, 0x01]);

        let response = GetVersion::try_from(data.as_slice())?;
        assert_eq!(response.protocol_version, Version::new()
            .with_major(0x10)
            .with_minor(0x10));
        assert_eq!(response.transport_version, Version::new()
            .with_major(0x01)
            .with_minor(0x01));

        Ok(())
    }

    #[test]
    fn test_get_daq_packed_mode() -> anyhow::Result<()> {
        let response = GetDAQPackedMode::new(DAQPackedMode::NotPacked, None)?;
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x00, 0x00]);

        let response = GetDAQPackedMode::try_from(data.as_slice())?;
        assert_eq!(response.daq_packed_mode, DAQPackedMode::NotPacked);

        let response = GetDAQPackedMode::new(
            DAQPackedMode::ElementGroup,
            Some(DAQPackedModeData::new(DPMTimestampMode::SingleTimestampOfFirstSample, 0x0010))
        )?;
        let data: Vec<_> = response.into();
        assert_eq!(data, vec![0x00, 0x01, 0x01, 0x00, 0x10]);

        let response = GetDAQPackedMode::try_from(data.as_slice())?;
        assert_eq!(response.daq_packed_mode, DAQPackedMode::ElementGroup);
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.timestamp_mode, DPMTimestampMode::SingleTimestampOfFirstSample);
        assert_eq!(data.sample_count, 0x0010);

        Ok(())
    }
}

