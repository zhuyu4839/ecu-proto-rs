//! request of Service 28


use crate::error::Error;
use crate::utils;
use crate::service::{CommunicationCtrlType, CommunicationType, Configuration, RequestData};

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

#[cfg(test)]
mod tests {
    use crate::service::{CommunicationCtrlType, CommunicationType, Configuration, RequestData};
    use super::CommunicationCtrl;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex::decode("280203")?;

        let request = CommunicationCtrl::new(
            CommunicationCtrlType::DisableRxAndEnableTx,
            CommunicationType::NormalCommunicationMessages |
                CommunicationType::NetworkManagementCommunicationMessages,
            None,
        )?;
        let result: Vec<_> = request.into();
        assert_eq!(result, source[2..].to_vec());

        let cfg = Configuration::default();
        let request = CommunicationCtrl::try_parse(
            &source[2..],
            Some(CommunicationCtrlType::DisableRxAndEnableTx),
            &cfg,
        )?;
        
        assert_eq!(request.comm_type, CommunicationType(0x03));
        assert_eq!(request.node_id, None);

        Ok(())
    }
}
