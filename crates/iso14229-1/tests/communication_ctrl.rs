//! Service 28

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, CommunicationCtrlType, CommunicationType, Configuration, RequestData, Service, TryFromWithCfg};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let source = hex::decode("280203")?;

        let request = request::CommunicationCtrl::new(
            CommunicationCtrlType::DisableRxAndEnableTx,
            CommunicationType::NormalCommunicationMessages |
                CommunicationType::NetworkManagementCommunicationMessages,
            None,
        )?;
        let result: Vec<_> = request.into();
        assert_eq!(result, source[2..].to_vec());

        let cfg = Configuration::default();
        let request = request::CommunicationCtrl::try_parse(
            &source[2..],
            Some(CommunicationCtrlType::DisableRxAndEnableTx),
            &cfg,
        )?;

        assert_eq!(request.comm_type, CommunicationType::new(
            CommunicationType::NormalCommunicationMessages | CommunicationType::NetworkManagementCommunicationMessages,
            0x00)?
        );
        assert_eq!(request.node_id, None);

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("6801")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<CommunicationCtrlType>()?, CommunicationCtrlType::EnableRxAndDisableTx);
        let data: Vec<u8> = response.data::<_, _>(&cfg)?;
        assert!(data.is_empty());

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F2812")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::CommunicationCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(response.nrc_code()?, response::Code::SubFunctionNotSupported);

        let response = response::Response::new(Service::NRC, None, vec![0x28, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::CommunicationCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(response.nrc_code()?, response::Code::SubFunctionNotSupported);

        Ok(())
    }
}
