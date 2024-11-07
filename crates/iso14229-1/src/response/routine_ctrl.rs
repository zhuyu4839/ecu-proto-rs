//! response of Service 31


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, error::Error, response::Code, ResponseData, RoutineCtrlType, RoutineId, utils, Service};
use crate::response::{Response, SubFunction};

lazy_static!(
    pub static ref ROUTINE_CTRL_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::GeneralProgrammingFailure,
    ]);
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoutineCtrl {
    pub routine_id: RoutineId,
    pub routine_info: Option<u8>,
    pub routine_status: Vec<u8>,
}

impl RoutineCtrl {
    pub fn new(
        routine_id: RoutineId,
        routine_info: Option<u8>,
        routine_status: Vec<u8>,
    ) -> Result<Self, Error> {
        if routine_info.is_none() && routine_status.len() > 0 {
            return Err(Error::InvalidData(
                "`routineStatusRecord` mut be empty when `routineInfo` is None".to_string()
            ));
        }

        Ok(Self { routine_id, routine_info, routine_status })
    }
}

impl<'a> TryFrom<&'a [u8]> for RoutineCtrl {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;

        let mut offset = 0;
        let routine_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
        let routine_id = RoutineId::from(routine_id);
        offset += 2;

        let (routine_info, routine_status) = if data_len > offset {
            let routine_info = data[offset];
            offset += 1;
            let routine_status = data[offset..].to_vec();
            (Some(routine_info), routine_status)
        }
        else {
            (None, vec![])
        };

        Ok(Self { routine_id, routine_info, routine_status })
    }
}

impl Into<Vec<u8>> for RoutineCtrl {
    fn into(mut self) -> Vec<u8> {
        let routine_id: u16 = self.routine_id.into();
        let mut result = routine_id.to_be_bytes().to_vec();
        if let Some(routine_info) = self.routine_info {
            result.push(routine_info);
            result.append(&mut self.routine_status);
        }

        result
    }
}

impl ResponseData for RoutineCtrl {
    type SubFunc = RoutineCtrlType;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        if sub_func.is_none() {
            return Err(Error::SubFunctionError(Service::RoutineCtrl));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn routine_ctrl(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Response, Error> {
    if sub_func.is_none() {
        return Err(Error::SubFunctionError(service));
    }

    let sf = RoutineCtrlType::try_from(sub_func.unwrap().0)?;
    let _ = RoutineCtrl::try_parse(data.as_slice(), Some(sf), cfg)?;

    Ok(Response { service, negative: false, sub_func, data })
}
