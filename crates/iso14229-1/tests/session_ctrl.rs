//! Service 10

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, Error, Service, SessionType, TryFromWithCfg};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("1001")?;

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
        let cfg = Configuration::default();

        let source = hex::decode("5003003201f4")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<SessionType>()?, SessionType::Extended);
        assert_eq!(response.is_negative(), false);

        let cfg = Configuration::default();
        let session: response::SessionTiming = response.data::<SessionType, _>(&cfg)?;
        assert_eq!(session.p2_ms(), 50);
        assert_eq!(session.p2_star_ms(), 5_000);

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F1012")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::SessionCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(response.nrc_code()?, response::Code::SubFunctionNotSupported);

        let response = response::Response::new(Service::NRC, None, vec![0x10, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::SessionCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(response.nrc_code()?, response::Code::SubFunctionNotSupported);

        Ok(())
    }
}
