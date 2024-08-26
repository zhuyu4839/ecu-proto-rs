use crate::error::Error;
use crate::service::DataIdentifier;
use crate::utils;

#[derive(Debug, Clone)]
pub struct ReadDIDData {
    pub did: DataIdentifier,
    pub others: Vec<DataIdentifier>,
}

impl ReadDIDData {
    pub fn new(
        did: DataIdentifier,
        others: Vec<DataIdentifier>
    ) -> Self {
        Self { did, others }
    }
}

impl<'a> TryFrom<&'a [u8]> for ReadDIDData {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;

        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        let mut others = Vec::new();
        while data_len > offset {
            utils::data_length_check(data_len, offset + 2, false)?;

            others.push(DataIdentifier::from(
                u16::from_be_bytes([data[offset], data[offset + 1]])
            ));
            offset += 2;
        }

        Ok(Self::new(did, others))
    }
}

impl Into<Vec<u8>> for ReadDIDData {
    fn into(self) -> Vec<u8> {
        let did: u16 = self.did.into();
        let mut result: Vec<_> = did.to_be_bytes().to_vec();
        self.others
            .into_iter()
            .for_each(|v| {
                let v: u16 = v.into();
                result.extend(v.to_be_bytes());
            });

        result
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::DataIdentifier;
    use super::ReadDIDData;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex!("22F190F180").as_slice();
        let request = ReadDIDData::new(
            DataIdentifier::VIN,
            vec![
                DataIdentifier::BootSoftwareIdentification,
            ]
        );
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let source = hex!("22F190F180 F181F182F183F184F185F186F187F188F189").as_slice();
        let request = ReadDIDData::new(
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

        let request = ReadDIDData::try_from(&source[1..])?;
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
}

