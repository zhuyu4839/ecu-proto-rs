//! request of Service 84


use crate::{AdministrativeParameter, Configuration, Error, Placeholder, RequestData, SignatureEncryptionCalculation, utils};

#[derive(Debug, Clone)]
pub struct SecuredDataTrans {
    pub apar: AdministrativeParameter,
    pub signature: SignatureEncryptionCalculation,
    // pub signature_len: u16,
    pub anti_replay_cnt: u16,
    pub service: u8,
    pub service_data: Vec<u8>,
    pub signature_data: Vec<u8>,
}

impl SecuredDataTrans {
    pub fn new(
        mut apar: AdministrativeParameter,
        signature: SignatureEncryptionCalculation,
        anti_replay_cnt: u16,
        service: u8,
        service_data: Vec<u8>,
        signature_data: Vec<u8>,
    ) -> Result<Self, Error> {
        if signature_data.len() > u16::MAX as usize {
            return Err(Error::InvalidParam("length of `Signature/MAC Byte` is out of range".to_string()));
        }

        if !apar.is_request() {
            apar.request_set(true);
        }

        Ok(Self {
            apar,
            signature,
            // signature_len: signature_data.len() as u16,
            anti_replay_cnt,
            service,
            service_data,
            signature_data,
        })
    }
}

impl<'a> TryFrom<&'a [u8]> for SecuredDataTrans {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 8, false)?;

        let mut offset = 0;
        let apar = AdministrativeParameter::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;
        if !apar.is_request() {
            return Err(Error::InvalidData(hex::encode(data)));
        }
        let signature = SignatureEncryptionCalculation::try_from(data[offset])?;
        offset += 1;

        let signature_len = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let anti_replay_cnt = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;

        let service = data[offset];
        offset += 1;

        utils::data_length_check(data_len, offset + signature_len as usize, false)?;

        let curr_offset = data_len - offset - signature_len as usize;
        let service_data = data[offset..offset + curr_offset].to_vec();
        offset += curr_offset;

        let signature_data = data[offset..].to_vec();

        Self::new(
            apar,
            signature,
            anti_replay_cnt,
            service,
            service_data,
            signature_data,
        )
    }
}

impl Into<Vec<u8>> for SecuredDataTrans {
    fn into(mut self) -> Vec<u8> {
        let mut result: Vec<_> = self.apar.into();
        result.push(self.signature.into());
        let signature_len = self.signature_data.len() as u16;
        result.extend(signature_len.to_be_bytes());
        result.extend(self.anti_replay_cnt.to_be_bytes());
        result.push(self.service);
        result.append(&mut self.service_data);
        result.append(&mut self.signature_data);

        result
    }
}

impl RequestData for SecuredDataTrans {
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
    use crate::{AdministrativeParameter, SignatureEncryptionCalculation};
    use super::SecuredDataTrans;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex::decode("84006100000601242EF123AA55DBD10EDC55AA")?;

        let mut apar = AdministrativeParameter::new();
        apar.signed_set(true)
            .signature_on_response_set(true);

        let request = SecuredDataTrans::new(
            apar,
            SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00),
            0x0124,
            0x2E,
            hex::decode("F123AA55")?,
            hex::decode("DBD10EDC55AA")?,
        )?;
        let result: Vec<_> = request.into();
        assert_eq!(result, source[1..].to_vec());

        let request = SecuredDataTrans::try_from(&source[1..])?;
        assert_eq!(request.apar.is_signed(), true);
        assert_eq!(request.apar.is_signature_on_response(), true);
        assert_eq!(request.signature, SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00));
        assert_eq!(request.anti_replay_cnt, 0x0124);
        assert_eq!(request.service, 0x2E);
        assert_eq!(request.service_data, hex::decode("F123AA55")?);
        assert_eq!(request.signature_data, hex::decode("DBD10EDC55AA")?);

        Ok(())
    }
}
