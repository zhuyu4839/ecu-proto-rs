
#[cfg(test)]
mod tests {
    use iso13400_2::{ActiveCode, Eid, LogicAddress, PowerMode};
    use iso14229_1::{request, Service, SessionType, Configuration as Iso14229Cfg};
    use doip::{client::{Configuration, DoIpClient}, DoIpError};

    fn init_client() -> Result<DoIpClient, DoIpError> {
        let cfg = Configuration::new("127.0.0.1", LogicAddress::from(0x0e00))
            .unwrap();
        DoIpClient::new(cfg)
    }

    #[test]
    fn vehicle_identifier() -> anyhow::Result<()> {
        let mut client = init_client()?;
        client.vehicle_identifier()?;

        Ok(())
    }

    #[test]
    fn vehicle_with_eid() -> anyhow::Result<()> {
        let mut client = init_client()?;
        client.vehicle_with_eid(Eid::new(0xE1E2E3E4E5E6)?)?;

        Ok(())
    }

    #[test]
    fn vehicle_with_vin() -> anyhow::Result<()> {
        let mut client = init_client()?;
        client.vehicle_with_vin("12345678901234567")?;
        // block
        // let ret = client.vehicle_with_vin("12345678901234560")?;
        // assert!(ret);

        Ok(())
    }

    #[test]
    fn entity_status() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let ret = client.entity_status()?;
        println!("{:?}", ret);

        Ok(())
    }

    #[test]
    fn diag_power_mode() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let power_mode = client.diag_power_mode()?;
        assert_eq!(power_mode, PowerMode::NotReady);

        Ok(())
    }

    #[test]
    fn alive_check() -> anyhow::Result<()> {
        let mut client = init_client()?;
        client.alive_check()?;

        Ok(())
    }

    #[test]
    fn routing_active() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let (ret, _) = client.routing_active(Default::default(), None)?;
        assert_eq!(ret, ActiveCode::Success);

        Ok(())
    }

    #[test]
    fn diagnostics() -> anyhow::Result<()> {
        let mut client = init_client()?;
        let cfg = Iso14229Cfg::default();
        client.vehicle_identifier()?;
        let (ret, value) = client.routing_active(Default::default(), None)?;
        assert_eq!(ret, ActiveCode::Success);
        assert!(value.is_none());
        let diag_req = request::Request::new(
            Service::SessionCtrl,
            Some(SessionType::Default.into()),
            vec![],
            &cfg
        )?;
        let resp = client.diagnostic(LogicAddress::from(57345), diag_req.into())?;
        println!("{:?}", resp);

        Ok(())
    }
}
