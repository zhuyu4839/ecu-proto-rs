//! request of Service 2F


use crate::{Configuration, Error, IOCtrlParameter, IOCtrlOption, DataIdentifier, request::{Request, SubFunction}, RequestData, Placeholder, utils, Service};

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
    ) -> Result<Self, Error> {
        match param {
            IOCtrlParameter::ReturnControlToEcu |
            IOCtrlParameter::ResetToDefault |
            IOCtrlParameter::FreezeCurrentState => {
                if !state.is_empty() {
                    return Err(Error::InvalidParam("expected empty `controlState`".to_string()));
                }
            }
            IOCtrlParameter::ShortTermAdjustment => {
                // let cfg_len = *UDS_CFG.did_cfg.get(&did)
                //     .ok_or(Error::DidCodecNotSupported(did))?;
                // if state.len() != cfg_len {
                //     return Err(Error::InvalidParam("`controlState` length doesn't match DID configuration".to_string()));
                // }
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
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 3, false)?;
        let mut offset = 0;

        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        let param = IOCtrlParameter::try_from(data[offset])?;
        offset += 1;
        match param {
            IOCtrlParameter::ReturnControlToEcu |
            IOCtrlParameter::ResetToDefault |
            IOCtrlParameter::FreezeCurrentState => {
                let mask = data[offset..].to_vec();
                Self::new(did, param, vec![], mask)
            },
            IOCtrlParameter::ShortTermAdjustment => {
                let record_len = *cfg.did_cfg.get(&did)
                    .ok_or(Error::DidNotSupported(did))?;
                utils::data_length_check(data_len, offset + record_len, false)?;

                let state = data[offset..offset + record_len].to_vec();
                offset += record_len;

                let mask = data[offset..].to_vec();

                Self::new(did, param, state, mask)
            },
        }
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
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = IOCtrl::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}

