//! request of Service 31


use crate::error::Error;
use crate::service::{Configuration, Placeholder, RequestData, RoutineCtrlType, RoutineId, Service};
use crate::utils;

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

#[cfg(test)]
mod tests {
    use crate::service::{CheckProgrammingDependencies, RoutineCtrlType};
    use super::RoutineCtrl;

    #[test]
    fn new() -> anyhow::Result<()> {
        let source = hex::decode("3101FF01")?;
        let request = RoutineCtrl {
            routine_id: CheckProgrammingDependencies,
            option_record: vec![],
        };
        let result: Vec<_> = request.into();
        assert_eq!(result, source[2..].to_vec());

        let source = hex::decode("3101FF01112233445566")?;
        let request = RoutineCtrl::try_from(&source[2..])?;

        assert_eq!(request.routine_id, CheckProgrammingDependencies);
        assert_eq!(request.option_record, hex::decode("112233445566")?);

        Ok(())
    }
}

