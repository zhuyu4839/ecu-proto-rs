//! response of Service 2F


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, DataIdentifier, UdsError, IOCtrlOption, IOCtrlParameter, Placeholder, response::Code, ResponseData, Service, utils};
use crate::response::{Response, SubFunction};

lazy_static!(
    pub static ref IO_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
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
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::IOCtrl));
        }

        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        let ctrl_type = IOCtrlParameter::try_from(data[offset])?;
        offset += 1;
        let &record_len = cfg.did_cfg.get(&did)
            .ok_or(UdsError::DidNotSupported(did))?;

        utils::data_length_check(data_len, offset + record_len, true)?;

        let record = data[offset..].to_vec();
        Ok(Self::new(did, ctrl_type, record))
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn io_ctrl(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, UdsError> {
    let _ = IOCtrl::try_parse(data.as_slice(), None, cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
