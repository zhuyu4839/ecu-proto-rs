//! Commons of Service 36


use crate::{Configuration, UdsError, Placeholder, RequestData, ResponseData, utils, Service};

#[derive(Debug, Clone)]
pub struct TransferData {
    pub sequence: u8,
    pub data: Vec<u8>,
}

impl<'a> TryFrom<&'a [u8]> for TransferData {
    type Error = UdsError;
    #[inline]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 1, false)?;

        let mut offset = 0;
        let sequence = data[offset];
        offset += 1;

        Ok(Self { sequence, data: data[offset..].to_vec() })
    }
}

impl Into<Vec<u8>> for TransferData {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.sequence];
        result.append(&mut self.data);
        result
    }
}

impl RequestData for TransferData {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::TransferData));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

impl ResponseData for TransferData {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::TransferData));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
