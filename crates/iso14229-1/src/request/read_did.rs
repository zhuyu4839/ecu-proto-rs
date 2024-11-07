//! request of Service 22


use crate::{Configuration, UdsError, DataIdentifier, Placeholder, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone)]
pub struct ReadDID {
    pub did: DataIdentifier,
    pub others: Vec<DataIdentifier>,
}

impl ReadDID {
    pub fn new(
        did: DataIdentifier,
        others: Vec<DataIdentifier>
    ) -> Self {
        Self { did, others }
    }
}

impl<'a> TryFrom<&'a [u8]> for ReadDID {
    type Error = UdsError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;

        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        let mut others = Vec::new();
        while data_len > offset {
            utils::data_length_check(data_len, offset + 2, false)?;

            others.push(DataIdentifier::from(
                u16::from_be_bytes([data[offset], data[offset + 1]])
            ));
            offset += 2;
        }

        Ok(Self::new(did, others))
    }
}

impl Into<Vec<u8>> for ReadDID {
    fn into(self) -> Vec<u8> {
        let did: u16 = self.did.into();
        let mut result: Vec<_> = did.to_be_bytes().to_vec();
        self.others
            .into_iter()
            .for_each(|v| {
                let v: u16 = v.into();
                result.extend(v.to_be_bytes());
            });

        result
    }
}

impl RequestData for ReadDID {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::ReadDID));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn read_did(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = ReadDID::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}

