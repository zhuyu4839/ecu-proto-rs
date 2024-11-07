//! request of Service 27

use crate::{Configuration, UdsError, request::{Request, SubFunction}, Service, SecurityAccessLevel, RequestData};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityAccess {
    pub data: Vec<u8>
}

impl RequestData for SecurityAccess {
    type SubFunc = SecurityAccessLevel;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError>
    where
        Self: Sized,
    {
        if sub_func.is_none() {
            return Err(UdsError::SubFunctionError(Service::SecurityAccess));
        }

        let level = sub_func.unwrap().0;
        if level % 2 != 1 {
            return Err(UdsError::InvalidParam(format!("Security access level: {}", level)));
        }

        Ok(Self { data: data.to_vec() })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}

pub(crate) fn security_access(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    _: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_none() {
        return Err(UdsError::SubFunctionError(service));
    }

    let level = sub_func.unwrap().function;
    let _ = SecurityAccessLevel::try_from(level)?;

    Ok(Request { service, sub_func, data })
}