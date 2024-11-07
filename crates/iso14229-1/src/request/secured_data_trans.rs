//! request of Service 84


use crate::{AdministrativeParameter, Configuration, UdsError, Placeholder, request::{Request, SubFunction}, RequestData, SignatureEncryptionCalculation, utils, Service};

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
    ) -> Result<Self, UdsError> {
        if signature_data.len() > u16::MAX as usize {
            return Err(UdsError::InvalidParam("length of `Signature/MAC Byte` is out of range".to_string()));
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
    type Error = UdsError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 8, false)?;

        let mut offset = 0;
        let apar = AdministrativeParameter::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;
        if !apar.is_request() {
            return Err(UdsError::InvalidData(hex::encode(data)));
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
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::SecuredDataTrans));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn secured_data_trans(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = SecuredDataTrans::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}
