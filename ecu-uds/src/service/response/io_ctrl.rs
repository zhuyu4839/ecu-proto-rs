//! response of Service 2F


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::service::{Configuration, DataIdentifier, IOCtrlOption, IOCtrlParameter, Placeholder, ResponseData, Service};
use crate::service::response::Code;
use crate::utils;

lazy_static!(
    pub static ref IO_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
    ]);
);

#[derive(Debug, Clone)]
pub struct IOCtrl {
    pub did: DataIdentifier,
    pub status: IOCtrlOption,
}

impl IOCtrl {
    #[inline]
    pub fn new(did: DataIdentifier,
               param: IOCtrlParameter,
               state: Vec<u8>,
    ) -> Self {
        Self {
            did,
            status: IOCtrlOption { param, state }
        }
    }
}

impl Into<Vec<u8>> for IOCtrl {
    fn into(mut self) -> Vec<u8> {
        let did: u16 = self.did.into();

        let mut result = did.to_be_bytes().to_vec();
        result.push(self.status.param.into());
        result.append(&mut self.status.state);

        result
    }
}

impl ResponseData for IOCtrl {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        let ctrl_type = IOCtrlParameter::try_from(data[offset])?;
        offset += 1;
        let record_len = *cfg.did_cfg.get(&did)
            .ok_or(Error::DidNotSupported(did))?;

        utils::data_length_check(data_len, offset + record_len, true)?;

        let record = data[offset..].to_vec();
        Ok(Self::new(did, ctrl_type, record))
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::{Configuration, DataIdentifier, IOCtrlParameter, ResponseData};
    use super::IOCtrl;

    #[test]
    fn new() -> anyhow::Result<()> {
        let did = DataIdentifier::from(0x4101);

        let mut cfg = Configuration::default();
        cfg.did_cfg.insert(did, 2);

        let source = hex!("6f4101030040").as_slice();
        let response = IOCtrl::new(
            did,
            IOCtrlParameter::ShortTermAdjustment,
            hex!("0040").to_vec(),
        );
        let result: Vec<_> = response.into();
        assert_eq!(result, source[1..]);

        let response = IOCtrl::try_parse(&source[1..], None, &cfg)?;
        assert_eq!(response.did, did);
        assert_eq!(response.status.param, IOCtrlParameter::ShortTermAdjustment);
        assert_eq!(response.status.state, hex!("0040"));

        Ok(())
    }
}

