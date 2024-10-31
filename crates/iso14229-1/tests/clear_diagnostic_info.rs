//! Service 14

#[cfg(test)]
mod tests {
    use iso14229_1::request;
    use iso14229_1::utils::U24;

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    #[test]
    fn test_request() -> anyhow::Result<()> {
        let source = hex::decode("14FFFF33")?;
        let request = request::ClearDiagnosticInfo::new(
            U24::from_be_bytes([0x00, 0xFF, 0xFF, 0x33]),
        );
        let result: Vec<_> = request.clone().into();
        assert_eq!(result, source[1..].to_vec());

        assert_eq!(request, request::ClearDiagnosticInfo::try_from(&source[1..])?);

        Ok(())
    }

    #[cfg(any(feature = "std2020"))]
    #[test]
    fn test_request() -> anyhow::Result<()> {
        let source = hex::decode("14FFFF3301")?;

        let request = request::ClearDiagnosticInfo::new(
            U24::from_be_bytes([0x00, 0xFF, 0xFF, 0x33]),
            Some(0x01),
        );
        assert_eq!(request, request::ClearDiagnosticInfo::try_from(&source[1..])?);
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        // let source = hex::decode("24")?;
        // let response = response::Response<>()

        Ok(())
    }
}
