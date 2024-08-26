/// Service 87

use crate::error::Error;
use crate::service::{Configuration, LinkCtrlMode, LinkCtrlType, RequestData};
use crate::utils;

#[derive(Debug, Clone)]
pub enum LinkCtrlData {
    VerifyModeTransitionWithFixedParameter(LinkCtrlMode), // 0x01
    VerifyModeTransitionWithSpecificParameter(utils::U24), // 0x02
    TransitionMode,
    VehicleManufacturerSpecific(Vec<u8>),
    SystemSupplierSpecific(Vec<u8>),
}

impl RequestData for LinkCtrlData {
    type SubFunc = LinkCtrlType;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
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
}

impl Into<Vec<u8>> for LinkCtrlData {
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

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::service::{Configuration, LinkCtrlMode, LinkCtrlType, RequestData, Service};
    use crate::utils::U24;
    use super::LinkCtrlData;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex!("870113").as_slice();
        let request = LinkCtrlData::VerifyModeTransitionWithFixedParameter(LinkCtrlMode::CAN1MBaud);
        let result: Vec<_> = request.into();
        assert_eq!(result, source[2..].to_vec());

        let cfg = Configuration::default();
        let request = LinkCtrlData::try_parse(&source[2..], Some(LinkCtrlType::VerifyModeTransitionWithFixedParameter), &cfg)?;
        match request {
            LinkCtrlData::VerifyModeTransitionWithFixedParameter(v) => {
                assert_eq!(v, LinkCtrlMode::CAN1MBaud);
            },
            LinkCtrlData::VerifyModeTransitionWithSpecificParameter(_) |
            LinkCtrlData::TransitionMode |
            LinkCtrlData::VehicleManufacturerSpecific(_) |
            LinkCtrlData::SystemSupplierSpecific(_) => panic!(),
        }

        let source = hex!("8702112233").as_slice();

        let request = LinkCtrlData::try_parse(&source[2..], Some(LinkCtrlType::VerifyModeTransitionWithSpecificParameter), &cfg)?;
        match request {
            LinkCtrlData::VerifyModeTransitionWithFixedParameter(_) => panic!(),
            LinkCtrlData::VerifyModeTransitionWithSpecificParameter(v) => {
                assert_eq!(v, U24(0x112233));
            }
            LinkCtrlData::TransitionMode |
            LinkCtrlData::VehicleManufacturerSpecific(_) |
            LinkCtrlData::SystemSupplierSpecific(_) => panic!(),
        }

        let source = hex!("8703").as_slice();

        let request = LinkCtrlData::try_parse(&source[2..], Some(LinkCtrlType::TransitionMode), &cfg)?;
        match request {
            LinkCtrlData::VerifyModeTransitionWithFixedParameter(_) => panic!(),
            LinkCtrlData::VerifyModeTransitionWithSpecificParameter(_) => panic!(),
            LinkCtrlData::TransitionMode => {},
            LinkCtrlData::VehicleManufacturerSpecific(_) |
            LinkCtrlData::SystemSupplierSpecific(_) => panic!(),
        }

        Ok(())
    }
}
