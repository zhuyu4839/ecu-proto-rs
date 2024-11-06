//! Service 22 | 2E

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, DIDData, DataIdentifier, ResponseData};

    #[test]
    fn test_write_did_request() -> anyhow::Result<()> {
        let source = hex::decode("2ef1904441564443313030394e544c5036313338")?;
        let request = request::WriteDID(
            DIDData {
                did: DataIdentifier::VIN,
                data: source[3..].to_vec(),  // 17 bytes
            }
        );
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let request = request::WriteDID::try_from(&source[1..])?;
        assert_eq!(request.0.did, DataIdentifier::VIN);
        assert_eq!(request.0.data, hex::decode("4441564443313030394e544c5036313338")?);

        Ok(())
    }

    #[test]
    fn test_read_did_request() -> anyhow::Result<()> {
        let source = hex::decode("22F190F180")?;
        let request = request::ReadDIDD::new(
            DataIdentifier::VIN,
            vec![
                DataIdentifier::BootSoftwareIdentification,
            ]
        );
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let source = hex::decode("22F190F180\
        F181F182F183F184F185F186F187F188F189")?;
        let request = request::ReadDIDD::new(
            DataIdentifier::VIN,
            vec![
                DataIdentifier::BootSoftwareIdentification,
                DataIdentifier::ApplicationSoftwareIdentification,
                DataIdentifier::ApplicationDataIdentification,
                DataIdentifier::BootSoftwareFingerprint,
                DataIdentifier::ApplicationSoftwareFingerprint,
                DataIdentifier::ApplicationDataFingerprint,
                DataIdentifier::ActiveDiagnosticSession,
                DataIdentifier::VehicleManufacturerSparePartNumber,
                DataIdentifier::VehicleManufacturerECUSoftwareNumber,
                DataIdentifier::VehicleManufacturerECUSoftwareVersionNumber,
            ]
        ); // 22 bytes + 1
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let request = request::ReadDIDD::try_from(&source[1..])?;
        assert_eq!(request.did, DataIdentifier::VIN);
        assert_eq!(request.others, vec![
            DataIdentifier::BootSoftwareIdentification,
            DataIdentifier::ApplicationSoftwareIdentification,
            DataIdentifier::ApplicationDataIdentification,
            DataIdentifier::BootSoftwareFingerprint,
            DataIdentifier::ApplicationSoftwareFingerprint,
            DataIdentifier::ApplicationDataFingerprint,
            DataIdentifier::ActiveDiagnosticSession,
            DataIdentifier::VehicleManufacturerSparePartNumber,
            DataIdentifier::VehicleManufacturerECUSoftwareNumber,
            DataIdentifier::VehicleManufacturerECUSoftwareVersionNumber,
        ]);

        Ok(())
    }

    #[test]
    fn test_read_did_response() -> anyhow::Result<()> {
        let source = hex::decode(
            "62\
            f1904441564443313030394e544c5036313338\
            F187445643374532303030303037"
        )?;
        let response = response::ReadDID {
            data: DIDData {
                did: DataIdentifier::VIN,
                data: hex::decode("4441564443313030394e544c5036313338")?
            },
            others: vec![
                DIDData {
                    did: DataIdentifier::VehicleManufacturerSparePartNumber,
                    data: hex::decode("445643374532303030303037")?
                },
            ]};

        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let mut cfg = Configuration::default();
        cfg.did_cfg.insert(DataIdentifier::VIN, 17);
        cfg.did_cfg.insert(DataIdentifier::VehicleManufacturerSparePartNumber, 12);

        let response = response::ReadDID::try_parse(&source[1..], None, &cfg)?;
        let response1 = response.data;
        assert_eq!(response1, DIDData {
            did: DataIdentifier::VIN,
            data: source[3..20].to_vec()
        });

        let response2 = response.others;
        assert_eq!(response2, vec![DIDData {
            did: DataIdentifier::VehicleManufacturerSparePartNumber,
            data: source[22..].to_vec()
        }, ]);

        Ok(())
    }

    #[test]
    fn test_write_did_response() -> anyhow::Result<()> {
        let source = hex::decode("6EF190")?;
        let response = response::WriteDID(DataIdentifier::VIN);
        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let response = response::WriteDID::try_from(&source[1..])?;
        assert_eq!(response.0, DataIdentifier::VIN);

        Ok(())
    }
}
