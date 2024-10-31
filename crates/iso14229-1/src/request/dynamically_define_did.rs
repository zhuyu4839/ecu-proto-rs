//! request of Service 2C


use crate::{DynamicallyDID, DefinitionType, DynamicallyMemAddr, Error, RequestData, Configuration, utils};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DynamicallyDefineDID {
    DefineByIdentifier {
        did: DynamicallyDID,
        source: DynamicallyMemAddr,
        others: Vec<DynamicallyMemAddr>
    },
    DefineByMemoryAddress {
        did: DynamicallyDID,
        memory: (u128, u128),           // (mem_addr, mem_size),
        others: Vec<(u128, u128)>,      // at least one
    },
    ClearDynamicallyDefinedDataIdentifier(Option<DynamicallyDID>),
}

impl RequestData for DynamicallyDefineDID {
    type SubFunc = DefinitionType;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        match sub_func {
            Some(v) => {
                let data_len = data.len();
                let mut offset = 0;
                match v {
                    DefinitionType::DefineByIdentifier => {
                        utils::data_length_check(data_len, offset + 6, false)?;

                        let did = DynamicallyDID::try_from(
                            u16::from_be_bytes([data[offset], data[offset + 1]])
                        )?;
                        offset += 2;
                        let source = DynamicallyMemAddr::try_from(&data[offset..])?;
                        offset += 4;

                        let mut others = Vec::new();
                        while data_len > offset {
                            utils::data_length_check(data_len, offset + 2, false)?;

                            others.push( DynamicallyMemAddr::try_from(&data[offset..])?);
                            offset += 2;
                        }

                        Ok(Self::DefineByIdentifier { did, source, others })
                    },
                    DefinitionType::DefineByMemoryAddress => {
                        utils::data_length_check(data_len, offset + 6, false)?;

                        let did = DynamicallyDID::try_from(
                            u16::from_be_bytes([data[offset], data[offset + 1]])
                        )?;
                        offset += 2;

                        let alfi = data[offset];
                        offset += 1;
                        let mem_addr_len = (alfi & 0x0F) as usize;
                        let mem_size_len = ((alfi & 0xF0) >> 4) as usize;
                        utils::data_length_check(data_len, offset + mem_addr_len + mem_size_len, false)?;

                        let mem_addr = utils::slice_to_u128(&data[offset..offset + mem_addr_len], cfg.bo_addr);
                        offset += mem_addr_len;
                        let mem_size = utils::slice_to_u128(&data[offset..offset + mem_size_len], cfg.bo_mem_size);
                        offset += mem_size_len;

                        let mut others = Vec::new();
                        while data_len > offset {
                            utils::data_length_check(data_len, offset + mem_addr_len + mem_size_len, false)?;

                            let mem_addr = utils::slice_to_u128(&data[offset..offset + mem_addr_len], cfg.bo_addr);
                            offset += mem_addr_len;
                            let mem_size = utils::slice_to_u128(&data[offset..offset + mem_size_len], cfg.bo_mem_size);
                            offset += mem_size_len;
                            others.push((mem_addr, mem_size));
                        }

                        Ok(Self::DefineByMemoryAddress {
                            did, memory: (mem_addr, mem_size), others
                        })
                    },
                    DefinitionType::ClearDynamicallyDefinedDataIdentifier => {
                        let dyn_did = match data_len - offset {
                            0 => Ok(None),
                            2 => Ok(Some(DynamicallyDID::try_from(
                                u16::from_be_bytes([data[offset], data[offset + 1]])
                            )?)),
                            v => Err(Error::InvalidDataLength { expect: 2, actual: v }),
                        }?;

                        Ok(Self::ClearDynamicallyDefinedDataIdentifier(dyn_did))
                    },
                }
            },
            None => panic!("Sub-function required"),
        }
    }
    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        match self {
            Self::DefineByIdentifier {
                did,
                source,
                others,
            } => {
                result.append(&mut did.into());
                result.append(&mut source.into());
                others.into_iter()
                    .for_each(|v| result.append(&mut v.into()));
            },
            Self::DefineByMemoryAddress {
                did,
                // addr_len,
                // size_len,
                memory,
                others,
            } => {
                result.append(&mut did.into());

                let mut max_addr = memory.0;
                let mut max_size = memory.1;
                others.iter()
                    .for_each(|v| {
                        let curr_addr = (*v).0;
                        let curr_size = (*v).1;
                        if curr_addr > max_addr {
                            max_addr = curr_addr;
                        }

                        if curr_size > max_size {
                            max_size = curr_size;
                        }
                    });

                let mem_addr_len = utils::length_of_u_type(max_addr);
                let mem_size_len = utils::length_of_u_type(max_size);
                result.push(((mem_size_len << 4) | mem_addr_len) as u8);

                let mut mem_addr = utils::u128_to_vec(memory.0, mem_addr_len, cfg.bo_addr);
                let mut mem_size = utils::u128_to_vec(memory.1, mem_size_len, cfg.bo_mem_size);
                result.append(&mut mem_addr);
                result.append(&mut mem_size);

                others.into_iter()
                    .for_each(|v| {
                        let mut mem_addr = utils::u128_to_vec(v.0, mem_addr_len, cfg.bo_addr);
                        let mut mem_size = utils::u128_to_vec(v.1, mem_size_len, cfg.bo_mem_size);
                        result.append(&mut mem_addr);
                        result.append(&mut mem_size);
                    });
            },
            Self::ClearDynamicallyDefinedDataIdentifier(did) => {
                if let Some(v) = did {
                    result.append(&mut v.into());
                }
            },
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{Configuration, DynamicallyDID, DynamicallyMemAddr, RequestData};
    use super::DynamicallyDefineDID;

    #[test]
    fn new() -> anyhow::Result<()> {
        let cfg = Configuration::default();
        let source = hex::decode("2C01F30112340102567801019ABC0104")?;
        let request = DynamicallyDefineDID::DefineByIdentifier {
                did: DynamicallyDID::try_from(0xF301)?,
                source: DynamicallyMemAddr {
                            did: 0x1234,
                            position: 1,
                            mem_size: 2,
                        },
                others: vec![
                    DynamicallyMemAddr {
                        did: 0x5678,
                        position: 1,
                        mem_size: 1,
                    },
                    DynamicallyMemAddr {
                        did: 0x9ABC,
                        position: 1,
                        mem_size: 4,
                    }
                ]
            };
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[2..].to_vec());

        let source = hex::decode("2C02F302240009196900012109196900012109196b0102131019950001")?;
        let request = DynamicallyDefineDID::DefineByMemoryAddress {
                did: DynamicallyDID::try_from(0xF302)?,
                memory: (0x00091969, 1),
                others: vec![
                    (0x21091969, 1),
                    (0x2109196B, 0x0102),
                    (0x13101995, 1),
                ],
            };
        let result: Vec<_> = request.to_vec(&cfg);
        assert_eq!(result, source[2..].to_vec());

        Ok(())
    }
}
