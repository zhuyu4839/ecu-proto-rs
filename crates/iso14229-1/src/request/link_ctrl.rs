//! request of Service 87


use crate::{Configuration, UdsError, LinkCtrlMode, LinkCtrlType, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone)]
pub enum LinkCtrl {
    VerifyModeTransitionWithFixedParameter(LinkCtrlMode), // 0x01
    VerifyModeTransitionWithSpecificParameter(utils::U24), // 0x02
    TransitionMode,
    VehicleManufacturerSpecific(Vec<u8>),
    SystemSupplierSpecific(Vec<u8>),
}

impl RequestData for LinkCtrl {
    type SubFunc = LinkCtrlType;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        match sub_func {
            Some(v) => {
                let data_len = data.len();
                let offset = 0;
                match v {
                    LinkCtrlType::VerifyModeTransitionWithFixedParameter => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        Ok(Self::VerifyModeTransitionWithFixedParameter(
                                LinkCtrlMode::try_from(data[offset])?
                            ))
                    },
                    LinkCtrlType::VerifyModeTransitionWithSpecificParameter => {
                        utils::data_length_check(data_len, offset + 3, true)?;

                        Ok(Self::VerifyModeTransitionWithSpecificParameter(
                                utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]])
                            ))
                    },
                    LinkCtrlType::TransitionMode => {
                        utils::data_length_check(data_len, offset, true)?;
                        Ok(Self::TransitionMode)
                    },
                    LinkCtrlType::VehicleManufacturerSpecific(_) => {
                        Ok(Self::VehicleManufacturerSpecific(data[offset..].to_vec()))
                    },
                    LinkCtrlType::SystemSupplierSpecific(_) => {
                        Ok(Self::SystemSupplierSpecific(data[offset..].to_vec()))
                    },
                    LinkCtrlType::Reserved(_) => {
                        Ok(Self::SystemSupplierSpecific(data[offset..].to_vec()))
                    }
                }
            },
            None => panic!("Sub-function required"),
        }
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

impl Into<Vec<u8>> for LinkCtrl {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();

        match self {
            Self::VerifyModeTransitionWithFixedParameter(v) => {
                result.push(v.into());
            },
            Self::VerifyModeTransitionWithSpecificParameter(v) => {
                result.append(&mut v.into());
            },
            Self::TransitionMode => {},
            Self::VehicleManufacturerSpecific(mut v) => {
                result.append(&mut v);
            },
            Self::SystemSupplierSpecific(mut v) => {
                result.append(&mut v);
            },
        }

        result
    }
}

pub(crate) fn link_ctrl(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let sf = LinkCtrlType::try_from(sub_func.unwrap().function)?;
    let _ = LinkCtrl::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Request { service, sub_func, data })
}
