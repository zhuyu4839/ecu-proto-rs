//! response of Service 84


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{AdministrativeParameter, Configuration, error::Iso14229Error, response::{Code, Response, SubFunction}, ResponseData, Service, SignatureEncryptionCalculation, utils};

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
    ) -> Result<Self, Iso14229Error> {
        if signature_data.len() > u16::MAX as usize {
            return Err(Iso14229Error::InvalidParam("length of `Signature/MAC Byte` is out of range".to_string()));
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
    ) -> Result<Self, Iso14229Error> {
        if signature_data.len() > u16::MAX as usize {
            return Err(Iso14229Error::InvalidParam("length of `Signature/MAC Byte` is out of range".to_string()));
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

impl ResponseData for SecuredDataTrans {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::SecuredDataTrans)),
            None => {
                utils::data_length_check(data.len(), 8, false)?;

                Ok(Response {
                    service: Service::SecuredDataTrans,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service;
        if service != Service::SecuredDataTrans
            || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &response.data;
        let data_len = data.len();
        let mut offset = 0;
        let apar = AdministrativeParameter::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;
        if apar.is_request() {
            return Err(Iso14229Error::InvalidData(hex::encode(data)));
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

    fn to_vec(self, _: &Configuration) -> Vec<u8> {
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
