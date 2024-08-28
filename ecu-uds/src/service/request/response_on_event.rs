//! request of Service 86


use bitfield_struct::bitfield;
use crate::enum_to_vec;
use crate::error::Error;
use crate::service::{ResponseOnEventType, Service, EventType, RequestData, Placeholder, Configuration};

enum_to_vec!(
    /// Table 142 — Comparison logic parameter definition
    pub enum ComparisonLogicID {
        LessThan = 0x01,
        LargerThan = 0x02,
        Equal = 0x03,
        NotEqual = 0x04,
    }, u8, Error, InvalidParam
);

/// Table 143 — Localization of value 16 bit bitfield parameter definition
///
/// ### Repr: `u16`
/// | Field   | Size (bits) | note |
/// |---------|-------------|-------------------------------------------------------|
/// | sign    | 1           | 0 means comparison without sign.                      |
/// |         |             | 1 means comparison with sign.                         |
/// | length  | 5           | The value 0x00 shall be used to compare all 4 bytes.  |
/// |         |             | All other values shall set the size in bits.          |
/// |         |             | With 5 bits, the maximal size of a length is 31 bits. |
/// | offset  | 10          | Offset on the positive response message from where to |
/// |         |             | extract the data identifier value.                    |
#[bitfield(u16, order = Msb)]
pub struct Localization {
    #[bits(1)]
    sign: bool,
    #[bits(5)]
    length: u8,
    #[bits(10)]
    offset: u16,
}

impl Localization {
    #[inline]
    pub const fn is_sign(&self) -> bool {
        self.sign()
    }

    #[inline]
    pub fn sign_set(&mut self, value: bool) -> &mut Self {
        self.set_sign(value);
        self
    }

    #[inline]
    pub const fn length_value(&self) -> u8 {
        self.length()
    }

    #[inline]
    pub fn length_set(&mut self, value: u8) -> &mut Self {
        self.set_length(value);
        self
    }

    #[inline]
    pub const fn offset_value(&self) -> u16 {
        self.offset()
    }

    #[inline]
    pub fn offset_set(&mut self, value: u16) -> &mut Self {
        self.set_offset(value);
        self
    }
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum EventTypeParameter {
    StopResponseOnEvent = 0x00,
    OnDTCStatusChange {
        test_failed: u8,
        service: Service,
        sub_func: u8,
        dtc_status_mask: u8,
    } = 0x01,   // Comparison Parameter < Measured Value
    OnChangeOfDataIdentifier {
        did: u16,
        service: Service,
    } = 0x03,   // Comparison Parameter > Measured Value
    ReportActivatedEvents = 0x04,
    StartResponseOnEvent = 0x05,        //
    ClearResponseOnEvent = 0x06,
    OnComparisonOfValues {
        did: u16,
        logic_id: ComparisonLogicID,
        comparison_ref: u32,
        hysteresis_value: u8,
        localization: Localization,
        service: Service,
        response_did: u16,      //
    } = 0x07,        // C2
    ReportMostRecentDtcOnStatusChange {
        report_type: u8,
    } = 0x08,   // C2
    ReportDTCRecordInformationOnDtcStatusChange {
        dtc_status_mask: u8,
        dtc_sub_func: u8,
        dtc_ext_data_record_num: u8,
    } = 0x09, // C2
}

#[derive(Debug, Clone)]
pub struct ResponseOnEvent {
    pub window_time: u8,            // unit of window time is `s`(seconds)
    pub param: EventTypeParameter,
}

#[allow(unused_variables)]
impl<'a> TryFrom<&'a [u8]> for ResponseOnEvent {
    type Error = Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        return Err(Error::OtherError("This library does not yet support".to_string()))
    }
}

impl Into<Vec<u8>> for ResponseOnEvent {
    fn into(self) -> Vec<u8> {
        panic!("This library does not yet support");
    }
}

impl RequestData for ResponseOnEvent {
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

