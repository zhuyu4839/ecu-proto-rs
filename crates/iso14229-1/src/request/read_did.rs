//! request of Service 22


use crate::{Configuration, Error, DataIdentifier, Placeholder, RequestData, utils};

#[derive(Debug, Clone)]
pub struct ReadDIDD {
    pub did: DataIdentifier,
    pub others: Vec<DataIdentifier>,
}

impl ReadDIDD {
    pub fn new(
        did: DataIdentifier,
        others: Vec<DataIdentifier>
    ) -> Self {
        Self { did, others }
    }
}

impl<'a> TryFrom<&'a [u8]> for ReadDIDD {
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

impl Into<Vec<u8>> for ReadDIDD {
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

impl RequestData for ReadDIDD {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::DataIdentifier;
    use super::ReadDIDD;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex::decode("22F190F180")?;
        let request = ReadDIDD::new(
            DataIdentifier::VIN,
            vec![
                DataIdentifier::BootSoftwareIdentification,
            ]
        );
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let source = hex::decode("22F190F180\
        F181F182F183F184F185F186F187F188F189")?;
        let request = ReadDIDD::new(
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

        let request = ReadDIDD::try_from(&source[1..])?;
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

