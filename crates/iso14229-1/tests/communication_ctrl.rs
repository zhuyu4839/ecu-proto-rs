//! Service 28

#[cfg(test)]
mod tests {
    use iso14229_1::{request, CommunicationCtrlType, CommunicationType, Configuration, RequestData};

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
}
