//! request of Service 2F


use crate::{Configuration, UdsError, IOCtrlParameter, IOCtrlOption, DataIdentifier, request::{Request, SubFunction}, RequestData, Placeholder, utils, Service};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOCtrl {
    pub did: DataIdentifier,
    pub option: IOCtrlOption,
    pub mask: Vec<u8>,
}

impl IOCtrl {
    pub fn new(
        did: DataIdentifier,
        param: IOCtrlParameter,
        state: Vec<u8>,
        mask: Vec<u8>,
        cfg: &Configuration,
    ) -> Result<Self, UdsError> {
        match param {
            IOCtrlParameter::ReturnControlToEcu |
            IOCtrlParameter::ResetToDefault |
            IOCtrlParameter::FreezeCurrentState => {
                if !state.is_empty() {
                    return Err(UdsError::InvalidParam("expected empty `controlState`".to_string()));
                }
            }
            IOCtrlParameter::ShortTermAdjustment => {
                let &did_len = cfg.did_cfg.get(&did)
                    .ok_or(UdsError::DidNotSupported(did))?;

                utils::data_length_check(state.len(), did_len, false)?;
            },
        }

        Ok(Self {
            did,
            option: IOCtrlOption { param, state },
            mask
        })
    }

    #[inline]
    pub fn data_identifier(&self) -> DataIdentifier {
        self.did
    }

    #[inline]
    pub fn ctrl_option(&self) -> &IOCtrlOption {
        &self.option
    }

    #[inline]
    pub fn ctrl_enable_mask(&self) -> &Vec<u8> {
        &self.mask
    }
}

impl Into<Vec<u8>> for IOCtrl {
    fn into(mut self) -> Vec<u8> {
        let did: u16 = self.did.into();
        let mut result = did.to_be_bytes().to_vec();
        result.push(self.option.param.into());
        result.append(&mut self.option.state);
        result.append(&mut self.mask);

        result
    }
}

impl RequestData for IOCtrl {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::IOCtrl));
        }

        let data_len = data.len();
        utils::data_length_check(data_len, 3, false)?;
        let mut offset = 0;

        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        let param = IOCtrlParameter::try_from(data[offset])?;
        offset += 1;
        let &did_len = cfg.did_cfg.get(&did)
            .ok_or(UdsError::DidNotSupported(did))?;
        utils::data_length_check(data_len, offset + did_len, false)?;
        let state = data[offset..offset + did_len].to_vec();
        offset += did_len;

        let mask = data[offset..].to_vec();
        Self::new(did, param, state, mask, cfg)
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
) -> Result<Request, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = IOCtrl::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}

