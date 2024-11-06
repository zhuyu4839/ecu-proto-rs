//! Service 2F

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, DataIdentifier, IOCtrlParameter, RequestData, ResponseData};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let did = DataIdentifier::from(0x4101);

        let mut cfg = Configuration::default();
        cfg.did_cfg.insert(did, 2);

        let source = hex::decode("2f4101030040ffff")?;
        let request = request::IOCtrl::new(
            did,
            IOCtrlParameter::ShortTermAdjustment,
            hex::decode("0040")?,
            hex::decode("ffff")?,
        )?;
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[1..].to_vec());

        let request = request::IOCtrl::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(request.did, did);
        assert_eq!(request.option.param, IOCtrlParameter::ShortTermAdjustment);
        assert_eq!(request.option.state, hex::decode("0040")?);
        assert_eq!(request.mask, hex::decode("ffff")?);

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let did = DataIdentifier::from(0x4101);

        let mut cfg = Configuration::default();
        cfg.did_cfg.insert(did, 2);

        let source = hex::decode("6f4101030040")?;
        let response = response::IOCtrl::new(
            did,
            IOCtrlParameter::ShortTermAdjustment,
            hex::decode("0040")?,
        );
        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let response = response::IOCtrl::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(response.did, did);
        assert_eq!(response.status.param, IOCtrlParameter::ShortTermAdjustment);
        assert_eq!(response.status.state, hex::decode("0040")?);

        Ok(())
    }
}

