//! request of Service 2A


use crate::{Configuration, enum_to_vec, Error, Placeholder, request::{Request, SubFunction}, RequestData, utils, Service};

enum_to_vec!(
    /// Table C.10 — transmissionMode parameter definitions
    pub enum TransmissionMode {
        SendAtSlowRate = 0x01,
        SendAtMediumRate = 0x02,
        SendAtFastRate = 0x03,
        StopSending = 0x04,
    }, u8);

pub struct ReadDataByPeriodId {
    mode: TransmissionMode,
    did: Vec<u8>,
}

impl ReadDataByPeriodId {
    pub fn new(
        mode: TransmissionMode,
        did: Vec<u8>
    ) -> Result<Self, Error> {
        match mode {
            TransmissionMode::SendAtSlowRate |
            TransmissionMode::SendAtMediumRate |
            TransmissionMode::SendAtFastRate => {
                if did.is_empty() {
                    return Err(Error::InvalidParam("empty period_id".to_string()));
                }

                Ok(())
            },
            TransmissionMode::StopSending => Ok(()),
        }?;

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

impl<'a> TryFrom<&'a [u8]> for ReadDataByPeriodId {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 1, false)?;

        let mut offset = 0;
        let mode = TransmissionMode::try_from(data[offset])?;
        offset += 1;

        let did = data[offset..].to_vec();

        Self::new(mode, did)
    }
}

impl Into<Vec<u8>> for ReadDataByPeriodId {
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![self.mode.into(), ];
        result.append(&mut self.did);

        result
    }
}

impl RequestData for ReadDataByPeriodId {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], _: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn read_data_by_pid(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = ReadDataByPeriodId::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}
