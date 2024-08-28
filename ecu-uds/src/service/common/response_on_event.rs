//! Commons of Service 86


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::constant::POSITIVE_OFFSET;
use crate::enum_to_vec;
use crate::error::Error;
use crate::service::Service;

lazy_static!(
    /// Table 91 — Recommended services to be used with the ResponseOnEvent service(2006)
    /// Table 96 — Recommended services to be used with the ResponseOnEvent service(2013)
    /// Table 137 — Recommended services to be used with the ResponseOnEvent service(2020)
    pub static ref RECOMMENDED_SERVICES: HashSet<Service> = HashSet::from([
        Service::ReadDID,
        Service::ReadDTCInfo,
        #[cfg(any(feature = "std2006", feature = "std2013"))]
        Service::RoutineCtrl,
        #[cfg(any(feature = "std2006", feature = "std2013"))]
        Service::IOCtrl,
    ]);
);

enum_to_vec!(
    pub enum ResponseOnEventType {
        StopResponseOnEvent = 0x00,
        OnDTCStatusChange = 0x01,
        OnChangeOfDataIdentifier = 0x02,
        ReportActivatedEvents = 0x04,
        StartResponseOnEvent = 0x05,
        ClearResponseOnEvent = 0x06,
        OnComparisonOfValues = 0x07,
        ReportMostRecentDtcOnStatusChange = 0x08,
        ReportDTCRecordInformationOnDtcStatusChange = 0x09,
    }, u8, Error, InvalidParam
);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EventType {
    pub(crate) store_event: bool,
    pub(crate) event_type: ResponseOnEventType,
}

impl EventType {
    #[inline]
    pub fn new(
        store_event: bool,
        event_type: ResponseOnEventType
    ) -> Self {
        Self { store_event, event_type }
    }

    #[inline]
    pub const fn store_event(&self) -> bool {
        self.store_event
    }

    #[inline]
    pub fn event_type(&self) -> ResponseOnEventType {
        self.event_type
    }
}

impl Into<u8> for EventType {
    #[inline]
    fn into(self) -> u8 {
        let mut result: u8 = self.event_type.into();
        if self.store_event {
            result |= POSITIVE_OFFSET;
        }

        result
    }
}

