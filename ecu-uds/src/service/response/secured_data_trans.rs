//! response of Service 84


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::service::{AdministrativeParameter, Configuration, Placeholder, ResponseData, Service, SignatureEncryptionCalculation};
use crate::service::response::Code;
use crate::utils;

lazy_static!(
    pub static ref SECURED_DATA_TRANS_NEGATIVES: HashSet<Code>
    = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        #[cfg(any(feature = "std2020"))]
        Code::SecureDataVerificationFailed,

        // Code::GeneralSecurityViolation,
        // Code::SecuredModeRequested,
        // Code::InsufficientProtection,
        // Code::TerminationWithSignatureRequested,
        // Code::AccessDenied,
        // Code::VersionNotSupported,
        // Code::SecuredLinkNotSupported,
        // Code::CertificateNotAvailable,
        // Code::AuditTrailInformationNotAvailable,
    ]);
);

/// Table 492 — Positive response message definition(successful)
#[derive(Debug, Clone)]
pub struct SecuredDataTransPositive {   // min_len 9
    pub apar: AdministrativeParameter,
    pub signature: SignatureEncryptionCalculation,
    // pub signature_len: u16,
    pub anti_replay_cnt: u16,

    pub response: u8,   // Internal Message Service Response ID
    pub response_params: Vec<u8>,
    pub signature_data: Vec<u8>,
}

impl SecuredDataTransPositive {

    pub fn new(
        mut apar: AdministrativeParameter,
        signature: SignatureEncryptionCalculation,
        anti_replay_cnt: u16,
        response: u8,
        response_params: Vec<u8>,
        signature_data: Vec<u8>,
    ) -> Result<Self, Error> {
        if signature_data.len() > u16::MAX as usize {
            return Err(Error::InvalidParam("length of `Signature/MAC Byte` is out of range".to_string()));
        }

        if apar.is_request() {
            apar.request_set(false);
        }

        Ok(Self {
            apar,
            signature,
            anti_replay_cnt,
            response,
            response_params,
            signature_data,
        })
    }
}

/// Table 494 — Positive response message definition(unsuccessful)
#[derive(Debug, Clone)]
pub struct SecuredDataTransNegative {   // min_len 11
    pub apar: AdministrativeParameter,
    pub signature: SignatureEncryptionCalculation,
    // pub signature_len: u16,
    pub anti_replay_cnt: u16,

    // pub nrc: Service,           // always Service::NRC
    pub service: u8,            // Internal Message Service Request ID
    pub response: u8,           // Internal Message responseCode
    pub signature_data: Vec<u8>,
}

impl SecuredDataTransNegative {
    #[must_use]
    pub fn new(
        mut apar: AdministrativeParameter,
        signature: SignatureEncryptionCalculation,
        anti_replay_cnt: u16,
        service: u8,
        response: u8,
        signature_data: Vec<u8>,
    ) -> Result<Self, Error> {
        if signature_data.len() > u16::MAX as usize {
            return Err(Error::InvalidParam("length of `Signature/MAC Byte` is out of range".to_string()));
        }

        if apar.is_request() {
            apar.request_set(false);
        }

        Ok(Self {
            apar,
            signature,
            anti_replay_cnt,
            service,
            response,
            signature_data,
        })
    }
}

#[derive(Debug, Clone)]
pub enum SecuredDataTrans {
    Successful(SecuredDataTransPositive),
    Unsuccessful(SecuredDataTransNegative),
}

impl<'a> TryFrom<&'a [u8]> for SecuredDataTrans {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 8, false)?;
        let mut offset = 0;
        let apar = AdministrativeParameter::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;
        if apar.is_request() {
            return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
        }
        let signature = SignatureEncryptionCalculation::try_from(data[offset])?;
        offset += 1;

        let signature_len = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let anti_replay_cnt = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;

        let code = data[offset];
        offset += 1;
        if code == Service::NRC.into() {
            utils::data_length_check(data_len,offset + signature_len as usize + 2, false)?;

            let service = data[offset];
            offset += 1;
            let response = data[offset];
            offset += 1;
            let signature_data = data[offset..].to_vec();

            Ok(Self::Unsuccessful(
                SecuredDataTransNegative::new(
                    apar,
                    signature,
                    anti_replay_cnt,
                    service,
                    response,
                    signature_data,
                )?
            ))
        }
        else {
            utils::data_length_check(data_len,offset + signature_len as usize, false)?;

            let curr_offset = data_len - offset - signature_len as usize;
            let response_params = data[offset..offset + curr_offset].to_vec();
            offset += curr_offset;

            let signature_data = data[offset..].to_vec();
            Ok(Self::Successful(
                SecuredDataTransPositive::new(
                    apar,
                    signature,
                    anti_replay_cnt,
                    code,
                    response_params,
                    signature_data,
                )?
            ))
        }
    }
}

impl Into<Vec<u8>> for SecuredDataTrans {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            Self::Successful(mut v) => {
                result.append(&mut v.apar.into());
                result.push(v.signature.into());
                let signature_len = v.signature_data.len() as u16;
                result.extend(signature_len.to_be_bytes());
                result.extend(v.anti_replay_cnt.to_be_bytes());
                result.push(v.response);
                result.append(&mut v.response_params);
                result.append(&mut v.signature_data);
            },
            Self::Unsuccessful(mut v) => {
                result.append(&mut v.apar.into());
                result.push(v.signature.into());
                let signature_len = v.signature_data.len() as u16;
                result.extend(signature_len.to_be_bytes());
                result.extend(v.anti_replay_cnt.to_be_bytes());
                result.push(Service::NRC.into());
                result.push(v.service);
                result.push(v.response);
                result.append(&mut v.signature_data);
            },
        }

        result
    }
}

impl ResponseData for SecuredDataTrans {
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
    use hex_literal::hex;
    use crate::service::{AdministrativeParameter, SignatureEncryptionCalculation};
    use super::{SecuredDataTrans, SecuredDataTransNegative, SecuredDataTransPositive};

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex!("C4002000000601246EF123FEDB910EDCFF").as_slice();
        let mut apar = AdministrativeParameter::new();
        apar.signed_set(true);
        let response = SecuredDataTrans::Successful(
                SecuredDataTransPositive::new(
                    apar,
                    SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00),
                    0x0124,
                    0x6E,
                    hex!("F123").to_vec(),
                    hex!("FEDB910EDCFF").to_vec(),
                )?
            );
        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let response = SecuredDataTrans::try_from(&source[1..])?;
        match response {
            SecuredDataTrans::Successful(v) => {
                assert_eq!(v.apar.is_signed(), true);
                assert_eq!(v.signature, SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00));
                assert_eq!(v.anti_replay_cnt, 0x0124);
                assert_eq!(v.response, 0x6E);
                assert_eq!(v.response_params, hex!("F123"));
                assert_eq!(v.signature_data, hex!("FEDB910EDCFF"));
            },
            SecuredDataTrans::Unsuccessful(_) => panic!(),
        }

        let source = hex!("C4002000000601367F2E13FEC9A180ECFF").as_slice();
        let mut apar = AdministrativeParameter::new();
        apar.signed_set(true);
        let response = SecuredDataTrans::Unsuccessful(
            SecuredDataTransNegative::new(
                apar,
                SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00),
                0x0136,
                0x2E,
                0x13,
                hex!("FEC9A180ECFF").to_vec(),
            )?);
        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let response = SecuredDataTrans::try_from(&source[1..])?;
        match response {
            SecuredDataTrans::Successful(_) => panic!(),
            SecuredDataTrans::Unsuccessful(v) => {
                assert_eq!(v.apar.is_signed(), true);
                assert_eq!(v.signature, SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00));
                assert_eq!(v.anti_replay_cnt, 0x0136);
                assert_eq!(v.service, 0x2E);
                assert_eq!(v.response, 0x13);
                assert_eq!(v.signature_data, hex!("FEC9A180ECFF"));
            },
        }

        Ok(())
    }
}
