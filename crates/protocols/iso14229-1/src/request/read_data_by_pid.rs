//! request of Service 2A


use crate::{Configuration, enum_extend, UdsError, request::{Request, SubFunction}, RequestData, utils, Service};

enum_extend!(
    /// Table C.10 — transmissionMode parameter definitions
    pub enum TransmissionMode {
        SendAtSlowRate = 0x01,
        SendAtMediumRate = 0x02,
        SendAtFastRate = 0x03,
        StopSending = 0x04,
    }, u8);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadDataByPeriodId {
    pub mode: TransmissionMode,
    pub did: Vec<u8>,
}

impl ReadDataByPeriodId {
    pub fn new(
        mode: TransmissionMode,
        did: Vec<u8>
    ) -> Result<Self, UdsError> {
        if did.is_empty() {
            return Err(UdsError::InvalidParam("empty period_id".to_string()));
        }

        Ok(Self { mode, did })
    }

    #[inline]
    pub fn transmission_mode(&self) -> TransmissionMode {
        self.mode.clone()
    }

    #[inline]
    pub fn period_did(&self) -> &Vec<u8> {
        &self.did
    }
}

impl RequestData for ReadDataByPeriodId {
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, UdsError> {
        match sub_func {
            Some(_) => Err(UdsError::SubFunctionError(Service::ReadDataByPeriodId)),
            None => {
                utils::data_length_check(data.len(), 2, false)?;

                Ok(Request { service: Service::ReadDataByPeriodId, sub_func: None, data: data.to_vec(), })
            },
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, UdsError> {
        let service = request.service();
        if service != Service::ReadDataByPeriodId
            || request.sub_func.is_some() {
            return Err(UdsError::ServiceError(service))
        }

        let data = &request.data;
        let mut offset = 0;
        let mode = TransmissionMode::try_from(data[offset])?;
        offset += 1;

        Ok(Self { mode, did: data[offset..].to_vec() })
    }

    fn to_vec(mut self, _: &Configuration) -> Vec<u8> {
        let mut result = vec![self.mode.into(), ];
        result.append(&mut self.did);

        result
    }
}