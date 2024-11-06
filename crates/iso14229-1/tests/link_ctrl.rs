//! Service 87

#[cfg(test)]
mod tests {
    use iso14229_1::{request, Configuration, LinkCtrlMode, LinkCtrlType, RequestData};
    use iso14229_1::utils::U24;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex::decode("870113")?;
        let request = request::LinkCtrl::VerifyModeTransitionWithFixedParameter(LinkCtrlMode::CAN1MBaud);
        let result: Vec<_> = request.into();
        assert_eq!(result, source[2..].to_vec());

        let cfg = Configuration::default();
        let request = request::LinkCtrl::try_parse(&source[2..], Some(LinkCtrlType::VerifyModeTransitionWithFixedParameter), &cfg)?;
        match request {
            request::LinkCtrl::VerifyModeTransitionWithFixedParameter(v) => {
                assert_eq!(v, LinkCtrlMode::CAN1MBaud);
            },
            request::LinkCtrl::VerifyModeTransitionWithSpecificParameter(_) |
            request::LinkCtrl::TransitionMode |
            request::LinkCtrl::VehicleManufacturerSpecific(_) |
            request::LinkCtrl::SystemSupplierSpecific(_) => panic!(),
        }

        let source = hex::decode("8702112233")?;

        let request = request::LinkCtrl::try_parse(&source[2..], Some(LinkCtrlType::VerifyModeTransitionWithSpecificParameter), &cfg)?;
        match request {
            request::LinkCtrl::VerifyModeTransitionWithFixedParameter(_) => panic!(),
            request::LinkCtrl::VerifyModeTransitionWithSpecificParameter(v) => {
                assert_eq!(v, U24::new(0x112233));
            }
            request::LinkCtrl::TransitionMode |
            request::LinkCtrl::VehicleManufacturerSpecific(_) |
            request::LinkCtrl::SystemSupplierSpecific(_) => panic!(),
        }

        let source = hex::decode("8703")?;

        let request = request::LinkCtrl::try_parse(&source[2..], Some(LinkCtrlType::TransitionMode), &cfg)?;
        match request {
            request::LinkCtrl::VerifyModeTransitionWithFixedParameter(_) => panic!(),
            request::LinkCtrl::VerifyModeTransitionWithSpecificParameter(_) => panic!(),
            request::LinkCtrl::TransitionMode => {},
            request::LinkCtrl::VehicleManufacturerSpecific(_) |
            request::LinkCtrl::SystemSupplierSpecific(_) => panic!(),
        }

        Ok(())
    }
}
