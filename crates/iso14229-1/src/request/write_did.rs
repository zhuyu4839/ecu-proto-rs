//! request of Service 2E


use crate::{Configuration, DataIdentifier, DIDData, Error, Placeholder, request::{Request, SubFunction}, RequestData, Service, utils};

/// Service 2E
pub struct WriteDID(pub DIDData);

impl<'a> TryFrom<&'a [u8]> for WriteDID {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 3, false)?;
        let mut offset = 0;
        let did = DataIdentifier::from(
            u16::from_be_bytes([data[offset], data[offset + 1]])
        );
        offset += 2;

        Ok(Self(DIDData { did, data: data[offset..].to_vec() }))
    }
}

impl Into<Vec<u8>> for WriteDID {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.0.into()
    }
}

impl RequestData for WriteDID {
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

pub(crate) fn write_did(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = WriteDID::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}
