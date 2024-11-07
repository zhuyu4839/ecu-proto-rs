//! request of Service 24


use crate::{Configuration, DataIdentifier, Error, Placeholder, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadScalingDID(pub DataIdentifier);

impl<'a> TryFrom<&'a [u8]> for ReadScalingDID {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        utils::data_length_check(data.len(), 2, true)?;

        let did = DataIdentifier::from(
            u16::from_be_bytes([data[0], data[1]])
        );

        Ok(Self(did))
    }
}

impl Into<Vec<u8>> for ReadScalingDID {
    fn into(self) -> Vec<u8> {
        let did: u16 = self.0.into();
        did.to_be_bytes().to_vec()
    }
}

impl RequestData for ReadScalingDID {
    type SubFunc = Placeholder;
    #[inline]
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        if sub_func.is_some() {
            return Err(Error::SubFunctionError(Service::ReadScalingDID));
        }

        Self::try_from(data)
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

pub(crate) fn read_scaling_did(
    service: Service,
    sub_func: Option<SubFunction>,
    data: Vec<u8>,
    cfg: &Configuration,
) -> Result<Request, Error> {
    if sub_func.is_some() {
        return Err(Error::SubFunctionError(service));
    }

    let _ = ReadScalingDID::try_parse(data.as_slice(), None, cfg)?;

    Ok(Request { service, sub_func, data })
}
