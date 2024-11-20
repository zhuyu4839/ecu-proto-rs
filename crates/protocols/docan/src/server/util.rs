use iso14229_1::{response::Code, Service};

#[inline]
pub fn service_not_support(service: u8) -> Vec<u8> {
    vec![Service::NRC.into(), service, Code::ServiceNotSupported.into()]
}

#[inline]
pub fn service_not_support_in_session(service: Service) -> Vec<u8> {
    vec![Service::NRC.into(), service.into(), Code::ServiceNotSupportedInActiveSession.into()]
}

#[inline]
pub fn sub_func_not_support(service: u8) -> Vec<u8> {
    vec![Service::NRC.into(), service, Code::SubFunctionNotSupported.into()]
}
