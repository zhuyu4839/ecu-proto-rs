//! Service 11

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, ECUResetType, Error, TryFromWithCfg};
    use iso14229_1::response::PowerDownSeconds;

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let source = hex::decode("1101")?;

        let cfg = Configuration::default();
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<ECUResetType>()?, ECUResetType::HardReset);
        assert_eq!(sub_func.is_suppress_positive(), Some(false));

        let source = hex::decode("1181")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<ECUResetType>()?, ECUResetType::HardReset);
        assert_eq!(sub_func.is_suppress_positive(), Some(true));

        let source = hex::decode("110100")?;
        let err = request::Request::try_from_cfg(source, &cfg).unwrap_err();
        match err {
            Error::InvalidDataLength { expect, actual } => {
                assert_eq!(expect, 0);
                assert_eq!(actual, 1);
            },
            _ => panic!("Expected Error::InvalidData"),
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let source = hex::decode("5101")?;

        let cfg = Configuration::default();
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<ECUResetType>()?, ECUResetType::HardReset);

        let source = hex::decode("5104")?;
        let err = response::Response::try_from_cfg(source, &cfg).unwrap_err();
        match err {
            Error::InvalidDataLength { expect, actual } => {
                assert_eq!(expect, 1);
                assert_eq!(actual, 0);
            },
            _ => panic!("Expected Error::InvalidData"),
        }

        let source = hex::decode("510401")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<ECUResetType>()?, ECUResetType::EnableRapidPowerShutDown);
        let data: PowerDownSeconds = response.data::<ECUResetType, _>(&cfg)?;
        assert_eq!(data.seconds(), Some(1));

        Ok(())
    }
}
