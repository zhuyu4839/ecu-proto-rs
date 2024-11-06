//! Service 10

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, Error, SessionType, TryFromWithCfg};
    use iso14229_1::response::SessionTiming;

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let source = hex::decode("1001")?;

        let cfg = Configuration::default();
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<SessionType>()?, SessionType::Default);
        assert_eq!(sub_func.is_suppress_positive(), Some(false));

        let source = hex::decode("1081")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<SessionType>()?, SessionType::Default);
        assert_eq!(sub_func.is_suppress_positive(), Some(true));

        let source = hex::decode("100100")?;
        let err = request::Request::try_from_cfg(source, &cfg).unwrap_err();
        match err {
            Error::InvalidData(v) => assert_eq!(v, "00"),
            _ => panic!("Expected Error::InvalidData"),
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let source = hex::decode("5003003201f4")?;

        let cfg = Configuration::default();
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<SessionType>()?, SessionType::Extended);
        assert_eq!(response.is_negative(), false);

        let cfg = Configuration::default();
        let session: SessionTiming = response.data::<SessionType, _>(&cfg)?;
        assert_eq!(session.p2_ms(), 50);
        assert_eq!(session.p2_star_ms(), 5_000);

        Ok(())
    }
}
