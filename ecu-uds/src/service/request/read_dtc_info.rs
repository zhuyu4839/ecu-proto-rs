use crate::error::Error;
use crate::service::{Configuration, DTCReportType, Placeholder, RequestData, Service};
use crate::utils;

#[derive(Debug, Clone)]
pub struct DTCExtDataRecord {
    pub number: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum DTCInfo {
    ReportNumberOfDTCByStatusMask(u8),      // 0x01
    ReportDTCByStatusMask(u8),              // 0x02
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportMirrorMemoryDTCByStatusMask(u8),          // 0x0F
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportNumberOfMirrorMemoryDTCByStatusMask(u8),  // 0x11
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportNumberOfEmissionsOBDDTCByStatusMask(u8),  // 0x12
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportEmissionsOBDDTCByStatusMask(u8),          // 0x13
    ReportDTCSnapshotIdentification,       // 0x03
    ReportDTCSnapshotRecordByDTCNumber {    // 0x04
        mask_record: utils::U24,
        record_num: u8,
    },
    ReportDTCStoredDataByRecordNumber {     // 0x05
        stored_num: u8,
    },
    #[cfg(any(feature = "std2006", feature = "std2020"))]
    ReportDTCExtDataRecordByDTCNumber {     // 0x06
        mask_record: utils::U24,
        extra_num: u8,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportMirrorMemoryDTCExtDataRecordByDTCNumber { // 0x10
        mask_record: utils::U24,
        extra_num: u8,
    },
    ReportNumberOfDTCBySeverityMaskRecord { // 0x07
        severity_mask: u8,
        status_mask: u8,
    },
    ReportDTCBySeverityMaskRecord {         // 0x08
        severity_mask: u8,
        status_mask: u8,
    },
    ReportSeverityInformationOfDTC {        // 0x09
        mask_record: utils::U24,
    },
    ReportSupportedDTC,                     // 0x0A
    ReportFirstTestFailedDTC,               // 0x0B
    ReportFirstConfirmedDTC,                // 0x0C
    ReportMostRecentTestFailedDTC,          // 0x0D
    ReportMostRecentConfirmedDTC,           // 0x0E
    ReportDTCFaultDetectionCounter,         // 0x14
    ReportDTCWithPermanentStatus,           // 0x15
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportDTCExtDataRecordByRecordNumber {  // 0x16
        extra_num: u8,  // 0x00~0xEF
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCByStatusMask {    // 0x17
        status_mask: u8,
        mem_selection: u8,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {   // 0x18
        mask_record: utils::U24,
        record_num: u8,
        mem_selection: u8,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCExtDataRecordByDTCNumber {    // 0x19
        mask_record: utils::U24,
        extra_num: u8,
        mem_selection: u8,
    },
    #[cfg(any(feature = "std2020"))]
    ReportSupportedDTCExtDataRecord {       // 0x1A
        extra_num: u8,  // 0x01~0xFD
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportWWHOBDDTCByMaskRecord {           // 0x42
        func_gid: u8, // 0x00~0xFE
        status_mask: u8,
        severity_mask: u8,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportWWHOBDDTCWithPermanentStatus {    // 0x55
        func_gid: u8, // 0x00~0xFE
    },
    #[cfg(any(feature = "std2020"))]
    ReportDTCInformationByDTCReadinessGroupIdentifier { // 0x56
        func_gid: u8, // 0x00~0xFE
        readiness_gid: u8, // 0x00~0xFE
    },
}

impl RequestData for DTCInfo {
    type SubFunc = DTCReportType;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, Error> {
        match sub_func {
            Some(v) => {
                let data_len = data.len();
                let mut offset = 0;

                match v {
                    DTCReportType::ReportNumberOfDTCByStatusMask => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        Ok(Self::ReportNumberOfDTCByStatusMask(data[offset]))
                    },
                    DTCReportType::ReportDTCByStatusMask => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        Ok(Self::ReportDTCByStatusMask(data[offset]))
                    },
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportMirrorMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        Ok(Self::ReportMirrorMemoryDTCByStatusMask(data[offset]))
                    },
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportNumberOfMirrorMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        Ok(Self::ReportNumberOfMirrorMemoryDTCByStatusMask(data[offset]))
                    },
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportNumberOfEmissionsOBDDTCByStatusMask => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        Ok(Self::ReportNumberOfEmissionsOBDDTCByStatusMask(data[offset]))
                    },
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportEmissionsOBDDTCByStatusMask => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        Ok(Self::ReportEmissionsOBDDTCByStatusMask(data[offset]))
                    },
                    DTCReportType::ReportDTCSnapshotIdentification =>
                        Ok(Self::ReportDTCSnapshotIdentification),
                    DTCReportType::ReportDTCSnapshotRecordByDTCNumber => {
                        utils::data_length_check(data_len, offset + 4, true)?;

                        let mask_record = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let record_num = data[offset];

                        Ok(Self::ReportDTCSnapshotRecordByDTCNumber {
                            mask_record,
                            record_num,
                        })
                    }
                    DTCReportType::ReportDTCStoredDataByRecordNumber => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        Ok(Self::ReportDTCStoredDataByRecordNumber {
                            stored_num: data[offset],
                        })
                    },
                    #[cfg(any(feature = "std2006", feature = "std2020"))]
                    DTCReportType::ReportDTCExtDataRecordByDTCNumber => {
                        utils::data_length_check(data_len, offset + 4, true)?;

                        let mask_record = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let extra_num = data[offset];

                        Ok(Self::ReportDTCExtDataRecordByDTCNumber {
                            mask_record,
                            extra_num,
                        })
                    },
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportMirrorMemoryDTCExtDataRecordByDTCNumber => {
                        utils::data_length_check(data_len, offset + 4, true)?;

                        let mask_record = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let extra_num = data[offset];

                        Ok(Self::ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
                            mask_record,
                            extra_num,
                        })
                    },
                    DTCReportType::ReportNumberOfDTCBySeverityMaskRecord => {
                        utils::data_length_check(data_len, offset + 2, true)?;

                        let severity_mask = data[offset];
                        offset += 1;
                        let status_mask = data[offset];

                        Ok(Self::ReportNumberOfDTCBySeverityMaskRecord {
                            severity_mask,
                            status_mask,
                        })
                    },
                    DTCReportType::ReportDTCBySeverityMaskRecord => {
                        utils::data_length_check(data_len, offset + 2, true)?;

                        let severity_mask = data[offset];
                        offset += 1;
                        let status_mask = data[offset];

                        Ok(Self::ReportDTCBySeverityMaskRecord {
                            severity_mask,
                            status_mask,
                        })
                    },
                    DTCReportType::ReportSeverityInformationOfDTC => {
                        utils::data_length_check(data_len, offset + 3, true)?;

                        let mask_record = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);

                        Ok(Self::ReportSeverityInformationOfDTC {
                            mask_record,
                        })
                    },
                    DTCReportType::ReportSupportedDTC =>
                        Ok(Self::ReportSupportedDTC),
                    DTCReportType::ReportFirstTestFailedDTC =>
                        Ok(Self::ReportFirstTestFailedDTC),
                    DTCReportType::ReportFirstConfirmedDTC =>
                        Ok(Self::ReportFirstConfirmedDTC),
                    DTCReportType::ReportMostRecentTestFailedDTC =>
                        Ok(Self::ReportMostRecentTestFailedDTC),
                    DTCReportType::ReportMostRecentConfirmedDTC =>
                        Ok(Self::ReportMostRecentConfirmedDTC),
                    DTCReportType::ReportDTCFaultDetectionCounter =>
                        Ok(Self::ReportDTCFaultDetectionCounter),
                    DTCReportType::ReportDTCWithPermanentStatus =>
                        Ok(Self::ReportDTCWithPermanentStatus),
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportDTCExtDataRecordByRecordNumber => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        let extra_num = data[offset];
                        if extra_num > 0xEF {
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                        }

                        Ok(Self::ReportDTCExtDataRecordByRecordNumber {
                            extra_num,
                        })
                    },
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, offset + 2, true)?;

                        Ok(Self::ReportUserDefMemoryDTCByStatusMask {
                            status_mask: data[offset],
                            mem_selection: data[offset + 1],
                        })
                    },
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber => {
                        utils::data_length_check(data_len, offset + 5, true)?;

                        let mask_record = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;

                        Ok(Self::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                            mask_record,
                            record_num: data[offset],
                            mem_selection: data[offset + 1],
                        })
                    },
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCExtDataRecordByDTCNumber => {
                        utils::data_length_check(data_len, offset + 5, true)?;

                        let mask_record = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;

                        Ok(Self::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                            mask_record,
                            extra_num: data[offset],
                            mem_selection: data[offset + 1],
                        })
                    },
                    #[cfg(any(feature = "std2020"))]
                    DTCReportType::ReportSupportedDTCExtDataRecord => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        let extra_num = data[offset];
                        if extra_num < 1 || extra_num > 0xFD {
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                        }

                        Ok(Self::ReportSupportedDTCExtDataRecord {
                            extra_num,
                        })
                    },
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportWWHOBDDTCByMaskRecord => {
                        utils::data_length_check(data_len, offset + 3, true)?;

                        let func_gid = data[offset];
                        offset += 1;
                        if func_gid > 0xFE {
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                        }

                        Ok(Self::ReportWWHOBDDTCByMaskRecord {
                            func_gid,
                            status_mask: data[offset],
                            severity_mask: data[offset + 1],
                        })
                    },
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportWWHOBDDTCWithPermanentStatus => {
                        utils::data_length_check(data_len, offset + 1, true)?;

                        let func_gid = data[offset];
                        if func_gid > 0xFE {
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                        }

                        Ok(Self::ReportWWHOBDDTCWithPermanentStatus {
                            func_gid,
                        })
                    },
                    #[cfg(any(feature = "std2020"))]
                    DTCReportType::ReportDTCInformationByDTCReadinessGroupIdentifier => {
                        utils::data_length_check(data_len, offset + 2, true)?;

                        let func_gid = data[offset];
                        offset += 1;
                        if func_gid > 0xFE {
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                        }

                        let readiness_gid = data[offset];
                        if readiness_gid > 0xFE {
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                        }

                        Ok(Self::ReportDTCInformationByDTCReadinessGroupIdentifier {
                            func_gid,
                            readiness_gid,
                        })
                    },
                }
            },
            None => panic!("Sub-function required"),
        }
    }
    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}

impl Into<Vec<u8>> for DTCInfo {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();

        match self {
            Self::ReportNumberOfDTCByStatusMask(v) => result.push(v),
            Self::ReportDTCByStatusMask(v) => result.push(v),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportMirrorMemoryDTCByStatusMask(v) => result.push(v),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportNumberOfMirrorMemoryDTCByStatusMask(v) => result.push(v),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportNumberOfEmissionsOBDDTCByStatusMask(v) => result.push(v),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportEmissionsOBDDTCByStatusMask(v) => result.push(v),
            Self::ReportDTCSnapshotIdentification => {},
            Self::ReportDTCSnapshotRecordByDTCNumber {
                mask_record,
                record_num,
            } => {
                result.append(&mut mask_record.into());
                result.push(record_num);
            },
            Self::ReportDTCStoredDataByRecordNumber { stored_num } => result.push(stored_num),
            Self::ReportDTCExtDataRecordByDTCNumber {
                mask_record,
                extra_num,
            } => {
                result.append(&mut mask_record.into());
                result.push(extra_num);
            },
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
                mask_record,
                extra_num,
            } => {
                result.append(&mut mask_record.into());
                result.push(extra_num);
            },
            Self::ReportNumberOfDTCBySeverityMaskRecord {
                severity_mask,
                status_mask,
            } => {
                result.push(severity_mask);
                result.push(status_mask);
            },
            Self::ReportDTCBySeverityMaskRecord {
                severity_mask,
                status_mask,
            } => {
                result.push(severity_mask);
                result.push(status_mask);
            },
            Self::ReportSeverityInformationOfDTC { mask_record } => result.append(&mut mask_record.into()),
            Self::ReportSupportedDTC => {},
            Self::ReportFirstTestFailedDTC => {},
            Self::ReportFirstConfirmedDTC => {},
            Self::ReportMostRecentTestFailedDTC => {},
            Self::ReportMostRecentConfirmedDTC => {},
            Self::ReportDTCFaultDetectionCounter => {},
            Self::ReportDTCWithPermanentStatus => {},
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportDTCExtDataRecordByRecordNumber { extra_num } => result.push(extra_num),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportUserDefMemoryDTCByStatusMask {
                status_mask,
                mem_selection } => {
                result.push(status_mask);
                result.push(mem_selection);
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                mask_record,
                record_num,
                mem_selection
            } => {
                result.append(&mut mask_record.into());
                result.push(record_num);
                result.push(mem_selection);
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                mask_record,
                extra_num,
                mem_selection
            } => {
                result.append(&mut mask_record.into());
                result.push(extra_num);
                result.push(mem_selection);
            },
            #[cfg(any(feature = "std2020"))]
            Self::ReportSupportedDTCExtDataRecord { extra_num } => result.push(extra_num),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportWWHOBDDTCByMaskRecord { func_gid, status_mask, severity_mask } => {
                result.push(func_gid);
                result.push(status_mask);
                result.push(severity_mask);
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportWWHOBDDTCWithPermanentStatus { func_gid } => result.push(func_gid),
            #[cfg(any(feature = "std2020"))]
            Self::ReportDTCInformationByDTCReadinessGroupIdentifier { func_gid, readiness_gid } => {
                result.push(func_gid);
                result.push(readiness_gid);
            },
        }

        result
    }
}

