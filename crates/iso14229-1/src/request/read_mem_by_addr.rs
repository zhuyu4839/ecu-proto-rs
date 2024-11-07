//! request of Service 23


use crate::{Configuration, UdsError, MemoryLocation, Placeholder, request::{Request, SubFunction}, RequestData, Service};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadMemByAddr(pub MemoryLocation);

impl RequestData for ReadMemByAddr {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, UdsError> {
        if sub_func.is_some() {
            return Err(UdsError::SubFunctionError(Service::ReadMemByAddr));
        }

        Ok(Self(MemoryLocation::from_slice(data, cfg)?))
    }
    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        self.0.to_vec(cfg)
    }
}

pub(crate) fn read_mem_by_addr(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, UdsError> {
    if sub_func.is_some() {
        return Err(UdsError::SubFunctionError(service));
    }

    let _ = ReadMemByAddr::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}
