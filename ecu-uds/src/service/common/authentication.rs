/// Commons of Service 29


use crate::error::Error;
use crate::{enum_to_vec, utils};

pub(crate) const ALGORITHM_INDICATOR_LENGTH: usize = 16;

enum_to_vec!(
    pub enum AuthenticationTask {
        DeAuthenticate = 0x00,
        VerifyCertificateUnidirectional = 0x01,
        VerifyCertificateBidirectional = 0x02,
        ProofOfOwnership = 0x03,
        TransmitCertificate = 0x04,
        RequestChallengeForAuthentication = 0x05,
        VerifyProofOfOwnershipUnidirectional = 0x06,
        VerifyProofOfOwnershipBidirectional = 0x07,
        AuthenticationConfiguration = 0x08,
    }, u8, Error, InvalidParam
);

#[derive(Debug, Clone)]
pub struct NotNullableData(pub(crate) Vec<u8>);

impl NotNullableData {
    #[inline]
    pub fn new(
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        if data.is_empty() || data.len() > u16::MAX as usize {
            return Err(Error::InvalidParam("Data must not be empty, and the length of the data must be less than or equal to 0xFFFF".to_string()));
        }

        Ok(Self(data))
    }
}

impl Into<Vec<u8>> for NotNullableData {
    #[inline]
    fn into(mut self) -> Vec<u8> {
        let len = self.0.len() as u16;
        let mut result = len.to_be_bytes().to_vec();
        result.append(&mut self.0);

        result
    }
}

#[derive(Debug, Clone)]
pub struct NullableData(pub(crate) Vec<u8>);

impl NullableData {
    #[inline]
    pub fn new(
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        if data.len() > u16::MAX as usize {
            return Err(Error::InvalidParam("the length of data must be less than or equal to 0xFFFF!".to_string()));
        }

        Ok(Self(data))
    }
}

impl Into<Vec<u8>> for NullableData {
    #[inline]
    fn into(mut self) -> Vec<u8> {
        let len = self.0.len() as u16;
        let mut result = len.to_be_bytes().to_vec();
        result.append(&mut self.0);

        result
    }
}

#[derive(Debug, Clone)]
pub struct AlgorithmIndicator(pub [u8; ALGORITHM_INDICATOR_LENGTH]);

impl Into<Vec<u8>> for AlgorithmIndicator {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[inline]
pub(crate) fn parse_nullable(
    data: &[u8],
    data_len: usize,
    offset: &mut usize,
) -> Result<NullableData, Error> {
    utils::data_length_check(data_len, *offset + 2, false)?;

    let len = u16::from_be_bytes([data[*offset], data[*offset + 1]]) as usize;
    *offset += 2;
    utils::data_length_check(data_len, *offset + len, false)?;

    let result = data[*offset..*offset + len].to_vec();
    *offset += len;

    Ok(NullableData(result))
}

#[inline]
pub(crate) fn parse_not_nullable(
    data: &[u8],
    data_len: usize,
    offset: &mut usize,
) -> Result<NotNullableData, Error> {
    utils::data_length_check(data_len, *offset + 2, false)?;

    let len = u16::from_be_bytes([data[*offset], data[*offset + 1]]) as usize;
    *offset += 2;
    if len == 0 {
        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
    }
    utils::data_length_check(data_len, *offset + len, false)?;

    let result = data[*offset..*offset + len].to_vec();
    *offset += len;

    Ok(NotNullableData(result))
}

#[inline]
pub(crate) fn parse_algo_indicator(
    data: &[u8],
    offset: &mut usize,
) -> AlgorithmIndicator {
    let result = &data[*offset..*offset + ALGORITHM_INDICATOR_LENGTH];
    *offset += ALGORITHM_INDICATOR_LENGTH;

    AlgorithmIndicator(result.try_into().unwrap())
}
