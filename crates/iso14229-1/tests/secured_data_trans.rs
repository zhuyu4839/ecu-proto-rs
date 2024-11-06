//! Service 84

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, AdministrativeParameter, SignatureEncryptionCalculation};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let source = hex::decode("84006100000601242EF123AA55DBD10EDC55AA")?;

        let mut apar = AdministrativeParameter::new();
        apar.signed_set(true)
            .signature_on_response_set(true);

        let request = request::SecuredDataTrans::new(
            apar,
            SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00),
            0x0124,
            0x2E,
            hex::decode("F123AA55")?,
            hex::decode("DBD10EDC55AA")?,
        )?;
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let request = request::SecuredDataTrans::try_from(&source[1..])?;
        assert_eq!(request.apar.is_signed(), true);
        assert_eq!(request.apar.is_signature_on_response(), true);
        assert_eq!(request.signature, SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00));
        assert_eq!(request.anti_replay_cnt, 0x0124);
        assert_eq!(request.service, 0x2E);
        assert_eq!(request.service_data, hex::decode("F123AA55")?);
        assert_eq!(request.signature_data, hex::decode("DBD10EDC55AA")?);

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let source = hex::decode("C4002000000601246EF123FEDB910EDCFF")?;
        let mut apar = AdministrativeParameter::new();
        apar.signed_set(true);
        let response = response::SecuredDataTrans::Successful(
            response::SecuredDataTransPositive::new(
                apar,
                SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00),
                0x0124,
                0x6E,
                hex::decode("F123")?,
                hex::decode("FEDB910EDCFF")?,
            )?
        );
        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let response = response::SecuredDataTrans::try_from(&source[1..])?;
        match response {
            response::SecuredDataTrans::Successful(v) => {
                assert_eq!(v.apar.is_signed(), true);
                assert_eq!(v.signature, SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00));
                assert_eq!(v.anti_replay_cnt, 0x0124);
                assert_eq!(v.response, 0x6E);
                assert_eq!(v.response_params, hex::decode("F123")?);
                assert_eq!(v.signature_data, hex::decode("FEDB910EDCFF")?);
            },
            _ => panic!(),
        }

        let source = hex::decode("C4002000000601367F2E13FEC9A180ECFF")?;
        let mut apar = AdministrativeParameter::new();
        apar.signed_set(true);
        let response = response::SecuredDataTrans::Unsuccessful(
            response::SecuredDataTransNegative::new(
                apar,
                SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00),
                0x0136,
                0x2E,
                0x13,
                hex::decode("FEC9A180ECFF")?,
            )?);
        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let response = response::SecuredDataTrans::try_from(&source[1..])?;
        match response {
            response::SecuredDataTrans::Unsuccessful(v) => {
                assert_eq!(v.apar.is_signed(), true);
                assert_eq!(v.signature, SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00));
                assert_eq!(v.anti_replay_cnt, 0x0136);
                assert_eq!(v.service, 0x2E);
                assert_eq!(v.response, 0x13);
                assert_eq!(v.signature_data, hex::decode("FEC9A180ECFF")?);
            },
            _ => panic!(),
        }

        Ok(())
    }
}
