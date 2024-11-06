//! request of Service 28


use crate::{CommunicationCtrlType, CommunicationType, Configuration, Error, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NodeId(u16);

impl TryFrom<u16> for NodeId {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001..=0xFFFF => Ok(Self(value)),
            v => Err(Error::InvalidParam(utils::err_msg(v))),
        }
    }
}

impl Into<u16> for NodeId {
    fn into(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct CommunicationCtrl {
    pub comm_type: CommunicationType,
    pub node_id: Option<NodeId>,
}

impl CommunicationCtrl {
    pub fn new(
        ctrl_type: CommunicationCtrlType,
        comm_type: CommunicationType,
        node_id: Option<NodeId>,
    ) -> Result<Self, Error> {
        match ctrl_type {
            CommunicationCtrlType::EnableRxAndDisableTxWithEnhancedAddressInformation |
            CommunicationCtrlType::EnableRxAndTxWithEnhancedAddressInformation => {
                match node_id {
                    Some(v) => Ok(Self { comm_type, node_id: Some(v), }),
                    None => Err(Error::InvalidParam("`nodeIdentificationNumber` is required".to_string())),
                }
            },
            _ => Ok(Self {  comm_type, node_id: None, })
        }
    }
}

impl RequestData for CommunicationCtrl {
    type SubFunc = CommunicationCtrlType;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        match sub_func {
            Some(v) => {
                let data_len = data.len();
                utils::data_length_check(data_len, 1, false)?;

                let mut offset = 0;
                let comm_type = data[offset];
                offset += 1;

                let node_id = match v {
                    CommunicationCtrlType::EnableRxAndDisableTxWithEnhancedAddressInformation |
                    CommunicationCtrlType::EnableRxAndTxWithEnhancedAddressInformation => {

                        utils::data_length_check(data_len, offset + 2, true)?;

                        Some(NodeId::try_from(
                            u16::from_be_bytes([data[offset], data[offset + 1]])
                        )?)
                    },
                    _ => None,
                };

                Ok(Self {
                    comm_type: CommunicationType(comm_type),
                    node_id,
                })
            },
            None => panic!("Sub-function required"),
        }
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

impl Into<Vec<u8>> for CommunicationCtrl {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.comm_type.0];
        if let Some(v) = self.node_id {
            let v: u16 = v.into();
            result.extend(v.to_be_bytes());
        }

        result
    }
}

pub(crate) fn communication_ctrl(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_none() {
        return Err(Error::SubFunctionError(service));
    }

    let sf = CommunicationCtrlType::try_from(sub_func.unwrap().function)?;
    let _ = CommunicationCtrl::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Request { service, sub_func, data })
}
