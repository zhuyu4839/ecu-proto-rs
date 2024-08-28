//! Commons of Service 19

use bitflags::bitflags;
use crate::enum_to_vec;
use crate::error::Error;
use crate::utils::U24;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DTCStatusMask: u8 {
        const TestFailed = 0x01;
        const TestFailedThisOperationCycle = 0x02;
        const PendingDTC = 0x04;
        const ConfirmedDTC = 0x08;
        const TestNotCompletedSinceLastClear = 0x10;
        const TestFailedSinceLastClear = 0x20;
        const TestNotCompletedThisOperationCycle = 0x40;
        const WarningIndicatorRequested = 0x80;
    }
}

enum_to_vec!(
    /// Table 317 — Request message SubFunction definition
    pub enum DTCReportType {
        ReportNumberOfDTCByStatusMask = 0x01,
        ReportDTCByStatusMask = 0x02,
        #[cfg(any(feature = "std2006", feature = "std2013"))]
        ReportMirrorMemoryDTCByStatusMask = 0x0F,
        #[cfg(any(feature = "std2006", feature = "std2013"))]
        ReportNumberOfMirrorMemoryDTCByStatusMask = 0x11,
        #[cfg(any(feature = "std2006", feature = "std2013"))]
        ReportNumberOfEmissionsOBDDTCByStatusMask = 0x12,
        #[cfg(any(feature = "std2006", feature = "std2013"))]
        ReportEmissionsOBDDTCByStatusMask = 0x13,
        ReportDTCSnapshotIdentification = 0x03,
        ReportDTCSnapshotRecordByDTCNumber = 0x04,
        ReportDTCStoredDataByRecordNumber = 0x05,
        ReportDTCExtDataRecordByDTCNumber = 0x06,
        #[cfg(any(feature = "std2006", feature = "std2013"))]
        ReportMirrorMemoryDTCExtDataRecordByDTCNumber = 0x10,
        ReportNumberOfDTCBySeverityMaskRecord = 0x07,  // (((statusOfDTC & DTCStatusMask) != 0) && ((severity & DTCSeverityMask) != 0)) == TRUE
        ReportDTCBySeverityMaskRecord = 0x08,  // (((statusOfDTC & DTCStatusMask) !=0) && ((severity & DTCSeverityMask) != 0)) == TRUE
        ReportSeverityInformationOfDTC = 0x09,
        ReportSupportedDTC = 0x0A,
        ReportFirstTestFailedDTC = 0x0B,
        ReportFirstConfirmedDTC = 0x0C,
        ReportMostRecentTestFailedDTC = 0x0D,
        ReportMostRecentConfirmedDTC = 0x0E,
        ReportDTCFaultDetectionCounter = 0x14,
        ReportDTCWithPermanentStatus = 0x15,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        ReportDTCExtDataRecordByRecordNumber = 0x16,    // DTCExtDataRecordNumber 00 to EF
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        ReportUserDefMemoryDTCByStatusMask = 0x17,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        ReportUserDefMemoryDTCSnapshotRecordByDTCNumber = 0x18,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        ReportUserDefMemoryDTCExtDataRecordByDTCNumber = 0x19,
        #[cfg(any(feature = "std2020"))]
        ReportSupportedDTCExtDataRecord = 0x1A,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        ReportWWHOBDDTCByMaskRecord = 0x42,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        ReportWWHOBDDTCWithPermanentStatus = 0x55,
        #[cfg(any(feature = "std2020"))]
        ReportDTCInformationByDTCReadinessGroupIdentifier = 0x56,
    }, u8, Error, InvalidParam
);


