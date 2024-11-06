//! request of Service 31


use crate::{Configuration,Error, Placeholder, request::{Request, SubFunction}, RequestData, RoutineCtrlType, RoutineId, Service, utils};

#[derive(Debug, Clone)]
pub struct RoutineCtrl {
    pub routine_id: RoutineId,
    pub option_record: Vec<u8>,
}

impl<'a> TryFrom<&'a [u8]> for RoutineCtrl {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 2, false)?;

        let mut offset = 0;
        let routine_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let routine_id = RoutineId::from(routine_id);

        Ok(Self { routine_id, option_record: data[offset..].to_vec() })
    }
}

impl Into<Vec<u8>> for RoutineCtrl {
    fn into(mut self) -> Vec<u8> {
        let routine_id: u16 = self.routine_id.into();
        let mut result = routine_id.to_be_bytes().to_vec();
        result.append(&mut self.option_record);

        result
    }
}

impl RequestData for RoutineCtrl {
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

pub(crate) fn routine_ctrl(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = RoutineCtrl::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}
