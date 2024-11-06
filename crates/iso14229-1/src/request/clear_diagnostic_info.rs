//! request of Service 14


use crate::{Configuration, Error, Placeholder, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClearDiagnosticInfo {
    group: utils::U24,
    #[cfg(any(feature = "std2020"))]
    mem_sel: Option<u8>, // Standard 2020 added
}

impl ClearDiagnosticInfo {
    #[cfg(any(feature = "std2020"))]
    pub fn new(
        group: utils::U24,
        mem_sel: Option<u8>,
    ) -> Self {
        Self { group, mem_sel }
    }

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    pub fn new(
        group: utils::U24,
    ) -> Self {
        Self { group }
    }

    pub fn group(&self) -> u32 {
        self.group.0
    }

    #[cfg(any(feature = "std2020"))]
    pub fn memory_selection(&self) -> Option<u8> {
        self.mem_sel
    }
}

impl<'a> TryFrom<&'a [u8]> for ClearDiagnosticInfo {
    type Error = Error;

    #[cfg(any(feature = "std2020"))]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 3, false)?;
        let mut offset = 0;
        let group = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
        offset += 3;

        let mem_selection = if data_len > offset {
            utils::data_length_check(data_len, 4, true)?;
            Some(data[offset])
        }
        else {
            None
        };

        Ok(Self::new(group, mem_selection))
    }

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 3, true)?;
        let offset = 0;

        let group = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);

        Ok(Self::new(group))
    }
}

impl Into<Vec<u8>> for ClearDiagnosticInfo {
    fn into(self) -> Vec<u8> {
        #[allow(unused_mut)]
        let mut result: Vec<_> = self.group.into();
        #[cfg(any(feature = "std2020"))]
        if let Some(v) = self.mem_sel {
            result.push(v);
        }

        result
    }
}

impl RequestData for ClearDiagnosticInfo {
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

pub(crate) fn clear_diag_info(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = ClearDiagnosticInfo::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}
