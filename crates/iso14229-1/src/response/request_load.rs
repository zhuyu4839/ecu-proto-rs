//! response of Service 34|35


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{ByteOrder, Configuration, error::UdsError, LengthFormatIdentifier, Placeholder, ResponseData, utils, Service};

use super::{Code, Response, SubFunction};

lazy_static!(
    pub static ref REQUEST_LOAD_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
        Code::UploadDownloadNotAccepted,
    ]);
);

#[derive(Debug, Clone)]
pub struct RequestLoad {
    pub lfi: LengthFormatIdentifier,
    pub max_num_of_block_len: u128,
}

impl RequestLoad {
    pub fn new(
        max_num_of_block_len: u128
    ) -> Result<Self, UdsError> {
        if max_num_of_block_len == 0 {
            return Err(UdsError::InvalidParam("`maxNumberOfBlockLength` must be rather than 0".to_string()));
        }

        let lfi = utils::length_of_u_type(max_num_of_block_len) as u8;

        Ok(Self {
            lfi: LengthFormatIdentifier(lfi << 4),
            max_num_of_block_len,
        })
    }
}

impl<'a> TryFrom<&'a [u8]> for RequestLoad {
    type Error = UdsError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let mut offset = 0;
        utils::data_length_check(data.len(), 1, false)?;
        let lfi = LengthFormatIdentifier::try_from(data[offset])?;
        offset += 1;

        let remain = &data[offset..];
        utils::data_length_check(lfi.max_number_of_block_length(), remain.len(), true)?;

        let max_num_of_block_len = utils::slice_to_u128(remain, ByteOrder::Big);
        if max_num_of_block_len == 0 {
            return Err(UdsError::InvalidParam("`maxNumberOfBlockLength` must be rather than 0".to_string()));
        }

        Ok(Self {
            lfi,
            max_num_of_block_len,
        })
    }
}

impl Into<Vec<u8>> for RequestLoad {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.lfi.0];
        let mut max_num_of_block_len = self.max_num_of_block_len.to_le_bytes().to_vec();
        max_num_of_block_len.resize(self.lfi.max_number_of_block_length(), Default::default());
        max_num_of_block_len.reverse();

        result.append(&mut max_num_of_block_len);

        result
    }
}

impl ResponseData for RequestLoad {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::RequestDownload));  // TODO Service::RequestUpload
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn request_download(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = RequestLoad::try_parse(data.as_slice(), None, cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}

pub(crate) fn request_upload(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = RequestLoad::try_parse(data.as_slice(), None, cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
