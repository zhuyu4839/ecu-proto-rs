//! response of Service 22

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, DataIdentifier, DIDData, error::UdsError, Placeholder, response::{Code, Response, SubFunction}, ResponseData, utils, Service};

lazy_static!(
    pub static ref READ_DID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ResponseTooLong,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ]);
);

#[derive(Debug, Clone)]
pub struct ReadDID {
    pub data: DIDData,
    pub others: Vec<DIDData>,
}

impl Into<Vec<u8>> for ReadDID {
    fn into(self) -> Vec<u8> {
        let mut result: Vec<_> = self.data.into();
        self.others.into_iter()
            .for_each(|v| {
                let mut tmp: Vec<_> = v.into();
                result.append(&mut tmp);
            });

        result
    }
}

impl ResponseData for ReadDID {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::ReadDID));
        }

        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;

        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;
        let &did_len = cfg.did_cfg.get(&did)
            .ok_or(UdsError::DidNotSupported(did))?;
        utils::data_length_check(data_len, offset + did_len, false)?;

        let context = DIDData {
            did,
            data: data[offset..offset + did_len].to_vec()
        };
        offset += did_len;

        let mut others = Vec::new();
        while data_len > offset {
            utils::data_length_check(data_len, offset + 2, false)?;

            let did = DataIdentifier::from(
                u16::from_be_bytes([data[offset], data[offset + 1]])
            );
            offset += 2;
            let &did_len = cfg.did_cfg.get(&did)
                .ok_or(UdsError::DidNotSupported(did))?;
            utils::data_length_check(data_len, offset + did_len, false)?;

            others.push(DIDData {
                did,
                data: data[offset..offset + did_len].to_vec()
            });
            offset += did_len;
        }

        Ok(Self { data: context, others })
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn read_did(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = ReadDID::try_parse(data.as_slice(), None, cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
