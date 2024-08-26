/// Service 22

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::error::Error;
use crate::service::{DataIdentifier, DIDData, ResponseData, Placeholder, Configuration};
use crate::service::response::Code;
use crate::utils;

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
pub struct ReadDIDData {
    pub data: DIDData,
    pub others: Vec<DIDData>,
}

impl Into<Vec<u8>> for ReadDIDData {
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

impl ResponseData for ReadDIDData {
    type SubFunc = Placeholder;
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;

        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;
        let did_len = *cfg.did_cfg.get(&did)
            .ok_or(Error::DidCodecNotSupported(did))?;
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
            let did_len = *cfg.did_cfg.get(&did)
                .ok_or(Error::DidCodecNotSupported(did))?;
            utils::data_length_check(data_len, offset + did_len, false)?;

            others.push(DIDData {
                did,
                data: data[offset..offset + did_len].to_vec()
            });
            offset += did_len;
        }

        Ok(Self { data: context, others })
    }
}

// #[cfg(test)]
// mod tests {
//     use hex_literal::hex;
//     use crate::service::{DataIdentifier, DIDData};
//     use super::ReadDID;
//
//     #[test]
//     fn new() -> anyhow::Result<()> {
//         let source = hex!(
//             "62\
//             f1904441564443313030394e544c5036313338\
//             F187445643374532303030303037"
//         ).as_slice();
//         let response = ReadDID::new(
//             DIDData {
//                 did: DataIdentifier::VIN,
//                 data: hex!("4441564443313030394e544c5036313338").to_vec()
//             },
//             vec![
//                 DIDData {
//                     did: DataIdentifier::VehicleManufacturerSparePartNumber,
//                     data: hex!("445643374532303030303037").to_vec()
//                 },
//             ]
//         );
//         let result: Vec<_> = response.into();
//         assert_eq!(result, source);
//
//         let response = ReadDID::try_from(&source[1..])?;
//         let response1 = response.response.get(0).ok_or(
//             anyhow::anyhow!("out of index")
//         )?;
//         assert_eq!(response1, &DIDData {
//             did: DataIdentifier::VIN,
//             data: source[3..20].to_vec()
//         });
//
//         let response2 = response.response.get(1).ok_or(
//             anyhow::anyhow!("out of index")
//         )?;
//         assert_eq!(response2, &DIDData {
//             did: DataIdentifier::VehicleManufacturerSparePartNumber,
//             data: source[22..].to_vec()
//         });
//
//         Ok(())
//     }
// }
