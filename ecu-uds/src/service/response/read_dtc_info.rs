//! response of Service 19


use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{enum_to_vec, utils};
use crate::error::Error;
use crate::service::{DataIdentifier, DTCReportType, ResponseData, Configuration, ALGORITHM_INDICATOR_LENGTH};
use crate::service::response::Code;
use crate::utils::U24;

lazy_static!(
    pub static ref READ_DTC_INFO_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestOutOfRange,
    ]);
);

enum_to_vec! (
    #[allow(non_camel_case_types)]
    pub enum DTCFormatIdentifier {
        SAE_J2012_DA_DTCFormat_00 = 0x00,
        ISO_14229_1_DTCFormat = 0x01,
        SAE_J1939_73_DTCFormat = 0x02,
        ISO_11992_4_DTCFormat = 0x03,
        SAE_J2012_DA_DTCFormat_04 = 0x04
    }, u8, Error, InvalidParam
);

#[derive(Debug, Clone)]
pub struct DTCAndStatusRecord { // 0x02 0x0A 0x0B 0x0C 0x0D 0x0E 0x15 0x17 0x1A 0x55 0x56
    pub dtc: U24,
    pub status: u8,     // DTCStatusMask
}

#[derive(Debug, Clone, )]
pub struct DTCSnapshotIdentification {  // 0x03
    pub dtc: U24,
    pub number: u8,
}

#[derive(Debug, Clone, )]
pub struct DTCSnapshotRecordByDTCNumber {   // 0x04
    pub number: u8,     // the echo of client request
    pub number_of_identifier: u8,
    pub records: Vec<(DataIdentifier, Vec<u8>)>,
}

#[derive(Debug, Clone, )]
pub struct DTCStoredDataByRecordNumber {    // 0x05
    pub status_record: DTCAndStatusRecord,
    pub number_of_identifier: u8,
    pub records: Vec<(DataIdentifier, Vec<u8>)>,
}

#[derive(Debug, Clone, )]
pub struct DTCExtDataRecordByDTCNumber {    // 0x06 0x10
    pub number: u8,     // 0x00~0xFD
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, )]
pub struct DTCExtDataRecordByRecordNumber { // 0x16
    pub status_record: DTCAndStatusRecord,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, )]
pub struct DTCAndSeverityRecord {   // 0x42
    pub severity: u8,
    pub dtc: U24,
    pub status: u8,
}

#[derive(Debug, Clone, )]
pub struct DTCAndSeverityRecord1 {  // 0x08 0x09
    pub severity: u8,
    pub func_unit: u8,
    pub dtc: U24,
    pub status: u8,
}

#[derive(Debug, Clone)]
pub struct DTCFaultDetectionCounterRecord { // 0x14
    pub dtc: U24,
    pub counter: u8,    // less than 0x7F
}

#[derive(Debug, Clone, )]
pub struct UserDefDTCSnapshotRecord {   // 0x18
    pub number: u8,     // the echo from client request
    // pub status_record: DTCAndStatusRecord,
    pub number_of_identifier: u8,
    pub records: Vec<(DataIdentifier, Vec<u8>)>,
}

// #[derive(Debug, Clone, )]
// pub struct UserDefMemoryDTCExtDataRecord {  // 0x19
//     pub number: u8,     // 00 to FE
//     pub record: Vec<u8>,
// }

#[derive(Debug, Clone, )]
pub enum DTCInfo {
    ReportNumberOfDTCByStatusMask {     // 0x01
        avl_mask: u8,
        fid: DTCFormatIdentifier,
        count: u16,
    },
    ReportDTCByStatusMask {             // 0x02
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportMirrorMemoryDTCByStatusMask { // 0x0F
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportNumberOfMirrorMemoryDTCByStatusMask { // 0x11
        avl_mask: u8,
        fid: DTCFormatIdentifier,
        count: u16,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportNumberOfEmissionsOBDDTCByStatusMask { // 0x12
        avl_mask: u8,
        fid: DTCFormatIdentifier,
        count: u16,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportEmissionsOBDDTCByStatusMask {     // 0x13
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    ReportDTCSnapshotIdentification {       // 0x03
        records: Vec<DTCSnapshotIdentification>
    },
    ReportDTCSnapshotRecordByDTCNumber {    // 0x04
        status_record: DTCAndStatusRecord,
        records: Vec<DTCSnapshotRecordByDTCNumber>,
    },
    ReportDTCStoredDataByRecordNumber {     // 0x05
        number: u8,
        status_records: Vec<DTCStoredDataByRecordNumber>,
    },
    ReportDTCExtDataRecordByDTCNumber {     // 0x06
        status_record: DTCAndStatusRecord,
        records: Vec<DTCExtDataRecordByDTCNumber>,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportMirrorMemoryDTCExtDataRecordByDTCNumber {    // 0x10
        status_record: DTCAndStatusRecord,
        records: Vec<DTCExtDataRecordByDTCNumber>,
    },
    ReportNumberOfDTCBySeverityMaskRecord { // 0x07
        avl_mask: u8,
        fid: DTCFormatIdentifier,
        count: u16,
    },
    ReportDTCBySeverityMaskRecord {         // 0x08
        avl_mask: u8,
        record: DTCAndSeverityRecord1,
        others: Vec<DTCAndSeverityRecord1>,
    },
    ReportSeverityInformationOfDTC {        // 0x09
        avl_mask: u8,
        records: Vec<DTCAndSeverityRecord1>,
    },
    ReportSupportedDTC {                    // 0x0A
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    ReportFirstTestFailedDTC {              // 0x0B
        avl_mask: u8,
        record: Option<DTCAndStatusRecord>,
    },
    ReportFirstConfirmedDTC {               // 0x0C
        avl_mask: u8,
        record: Option<DTCAndStatusRecord>,
    },
    ReportMostRecentTestFailedDTC {         // 0x0D
        avl_mask: u8,
        record: Option<DTCAndStatusRecord>,
    },
    ReportMostRecentConfirmedDTC {          // 0x0E
        avl_mask: u8,
        record: Option<DTCAndStatusRecord>,
    },
    ReportDTCFaultDetectionCounter {        // 0x14
        record: DTCFaultDetectionCounterRecord,
        others: Vec<DTCFaultDetectionCounterRecord>,
    },
    ReportDTCWithPermanentStatus {          // 0x15
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportDTCExtDataRecordByRecordNumber {  // 0x16
        ext_number: u8,
        records: Vec<DTCExtDataRecordByRecordNumber>,       // length of .1 = ext_number
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCByStatusMask {    // 0x17
        mem_selection: u8,
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {   // 0x18
        mem_selection: u8,
        status_record: DTCAndStatusRecord,
        records: Vec<UserDefDTCSnapshotRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCExtDataRecordByDTCNumber {    // 0x19
        mem_selection: u8,
        status_record: DTCAndStatusRecord,
        number: Option<u8>,     // 0x00~0xFE
        records: Vec<Vec<u8>>,
    } ,
    #[cfg(any(feature = "std2020"))]
    ReportSupportedDTCExtDataRecord {       // 0x1A
        avl_mask: u8,
        number: u8, // 00 to FD
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportWWHOBDDTCByMaskRecord {           // 0x42
        func_gid: u8,    // 00 to FE
        status_avl_mask: u8,
        severity_avl_mask: u8,
        /// Only supported [`DTCFormatIdentifier::SAE_J1939_73_DTCFormat`] and
        /// [`DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04`]
        fid: DTCFormatIdentifier,
        records: Vec<DTCAndSeverityRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportWWHOBDDTCWithPermanentStatus {    // 0x55
        func_gid: u8,    // 00 to FE
        status_avl_mask: u8,
        /// Only supported [`DTCFormatIdentifier::SAE_J1939_73_DTCFormat`] and
        /// [`DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04`]
        fid: DTCFormatIdentifier,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2020"))]
    ReportDTCInformationByDTCReadinessGroupIdentifier { // 0x56
        func_gid: u8,    // 00 to FE
        status_avl_mask: u8,
        format_identifier: u8,
        readiness_gid: u8,     // 00 to FE
        records: Vec<DTCAndStatusRecord>,
    },
}

impl ResponseData for DTCInfo {
    type SubFunc = DTCReportType;
    fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, cfg: &Configuration) -> Result<Self, Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 1, false)?;
        let mut offset = 0;

        match sub_func {
            Some(v) => match v {
                DTCReportType::ReportNumberOfDTCByStatusMask => {
                    utils::data_length_check(data_len, offset + 4, false)?;

                    let avl_mask = data[offset];
                    offset += 1;
                    let fid = DTCFormatIdentifier::try_from(data[offset])?;
                    offset += 1;
                    match fid {
                        DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_00 => {}
                        DTCFormatIdentifier::ISO_14229_1_DTCFormat => {}
                        DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                        DTCFormatIdentifier::ISO_11992_4_DTCFormat => {}
                        DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 =>
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data))),
                    }
                    let count = u16::from_be_bytes([data[offset], data[offset + 1]]);

                    Ok(Self::ReportNumberOfDTCByStatusMask {
                        avl_mask,
                        fid,
                        count
                    })
                },
                DTCReportType::ReportDTCByStatusMask => {
                    utils::data_length_check(data_len, offset + 5, false)?;
                    if (data_len - offset - 1) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let avl_mask = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportDTCByStatusMask {
                        avl_mask,
                        records,
                    })
                },
                #[cfg(any(feature = "std2006", feature = "std2013"))]
                DTCReportType::ReportMirrorMemoryDTCByStatusMask => {
                    utils::data_length_check(data_len, offset + 5, false)?;
                    if (data_len - offset - 1) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let avl_mask = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportMirrorMemoryDTCByStatusMask {
                        avl_mask,
                        records,
                    })
                },
                #[cfg(any(feature = "std2006", feature = "std2013"))]
                DTCReportType::ReportNumberOfMirrorMemoryDTCByStatusMask => {
                    utils::data_length_check(data_len, offset + 4, false)?;

                    let avl_mask = data[offset];
                    offset += 1;
                    let fid = DTCFormatIdentifier::try_from(data[offset])?;
                    offset += 1;
                    match fid {
                        DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_00 => {}
                        DTCFormatIdentifier::ISO_14229_1_DTCFormat => {}
                        DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                        DTCFormatIdentifier::ISO_11992_4_DTCFormat => {}
                        DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 =>
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data))),
                    }
                    let count = u16::from_be_bytes([data[offset], data[offset + 1]]);

                    Ok(Self::ReportNumberOfMirrorMemoryDTCByStatusMask {
                        avl_mask,
                        fid,
                        count
                    })
                },
                #[cfg(any(feature = "std2006", feature = "std2013"))]
                DTCReportType::ReportNumberOfEmissionsOBDDTCByStatusMask => {
                    utils::data_length_check(data_len, offset + 4, false)?;

                    let avl_mask = data[offset];
                    offset += 1;
                    let fid = DTCFormatIdentifier::try_from(data[offset])?;
                    offset += 1;
                    match fid {
                        DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_00 => {}
                        DTCFormatIdentifier::ISO_14229_1_DTCFormat => {}
                        DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                        DTCFormatIdentifier::ISO_11992_4_DTCFormat => {}
                        DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 =>
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data))),
                    }
                    let count = u16::from_be_bytes([data[offset], data[offset + 1]]);

                    Ok(Self::ReportNumberOfEmissionsOBDDTCByStatusMask {
                        avl_mask,
                        fid,
                        count
                    })
                },
                #[cfg(any(feature = "std2006", feature = "std2013"))]
                DTCReportType::ReportEmissionsOBDDTCByStatusMask => {
                    utils::data_length_check(data_len, offset + 5, false)?;
                    if (data_len - offset - 1) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let avl_mask = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportEmissionsOBDDTCByStatusMask {
                        avl_mask,
                        records,
                    })
                },
                DTCReportType::ReportDTCSnapshotIdentification => {
                    if (data_len % 4) != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let number = data[offset];
                        offset += 1;

                        records.push(DTCSnapshotIdentification { dtc, number });

                    }

                    Ok(Self::ReportDTCSnapshotIdentification {
                        records
                    })
                },
                DTCReportType::ReportDTCSnapshotRecordByDTCNumber => {
                    utils::data_length_check(data_len, offset + 4, false)?;

                    let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        utils::data_length_check(data_len, offset + 2, false)?;

                        let number = data[offset];
                        offset += 1;
                        let number_of_identifier = data[offset];
                        offset += 1;
                        let mut sub_records = Vec::new();
                        while sub_records.len() < number_of_identifier as usize {
                            utils::data_length_check(data_len, offset + 2, false)?;

                            let did = DataIdentifier::from(
                                u16::from_be_bytes([data[offset], data[offset + 1]])
                            );
                            offset += 2;
                            let did_data_len = *cfg.did_cfg.get(&did)
                                .ok_or(Error::DidNotSupported(did))?;

                            utils::data_length_check(data_len, offset + did_data_len, false)?;

                            sub_records.push((did, data[offset..offset + did_data_len].to_vec()));
                            offset += did_data_len;

                        }

                        records.push(DTCSnapshotRecordByDTCNumber {
                            number, number_of_identifier, records: sub_records,
                        });
                    }

                    Ok(Self::ReportDTCSnapshotRecordByDTCNumber {
                        status_record: DTCAndStatusRecord { dtc, status },
                        records
                    })
                },
                DTCReportType::ReportDTCStoredDataByRecordNumber => {
                    utils::data_length_check(data_len, offset + 1, false)?;

                    let number = data[offset];
                    offset += 1;

                    let mut status_records = Vec::new();

                    while data_len > offset {
                        utils::data_length_check(data_len, offset + 5, false)?;

                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;
                        let number_of_identifier = data[offset];
                        offset += 1;

                        let mut sub_records = Vec::new();
                        while sub_records.len() < number_of_identifier as usize {
                            utils::data_length_check(data_len, offset + 2, false)?;

                            let did = DataIdentifier::from(
                                u16::from_be_bytes([data[offset], data[offset + 1]])
                            );
                            offset += 2;
                            let did_data_len = *cfg.did_cfg.get(&did)
                                .ok_or(Error::DidNotSupported(did))?;

                            utils::data_length_check(data_len, offset + did_data_len, false)?;

                            sub_records.push((did, data[offset..offset + did_data_len].to_vec()));
                            offset += did_data_len;
                        }

                        status_records.push(DTCStoredDataByRecordNumber {
                            status_record: DTCAndStatusRecord { dtc, status },
                            number_of_identifier,
                            records: sub_records,
                        });
                    }

                    Ok(Self::ReportDTCStoredDataByRecordNumber {
                        number,
                        status_records,
                    })
                },
                DTCReportType::ReportDTCExtDataRecordByDTCNumber => {
                    utils::data_length_check(data_len, offset + 4, false)?;

                    let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let number = data[offset];
                        offset += 1;
                        utils::data_length_check(data_len, offset + number as usize, false)?;

                        records.push(DTCExtDataRecordByDTCNumber {
                            number,
                            data: data[offset..offset + number as usize].to_vec(),
                        });
                        offset += number as usize;
                    }

                    Ok(Self::ReportDTCExtDataRecordByDTCNumber {
                        status_record: DTCAndStatusRecord { dtc, status },
                        records,
                    })
                },
                #[cfg(any(feature = "std2006", feature = "std2013"))]
                DTCReportType::ReportMirrorMemoryDTCExtDataRecordByDTCNumber => {
                    utils::data_length_check(data_len, offset + 4, false)?;

                    let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let number = data[offset];
                        offset += 1;
                        utils::data_length_check(data_len, offset + number as usize, false)?;

                        records.push(DTCExtDataRecordByDTCNumber {
                            number,
                            data: data[offset..offset + number as usize].to_vec(),
                        });
                        offset += number as usize;
                    }

                    Ok(Self::ReportDTCExtDataRecordByDTCNumber {
                        status_record: DTCAndStatusRecord { dtc, status },
                        records,
                    })
                },
                DTCReportType::ReportNumberOfDTCBySeverityMaskRecord => {
                    utils::data_length_check(data_len, offset + 4, false)?;

                    let avl_mask = data[offset];
                    offset += 1;
                    let fid = DTCFormatIdentifier::try_from(data[offset])?;
                    offset += 1;
                    let count = u16::from_be_bytes([data[offset], data[offset + 1]]);

                    Ok(Self::ReportNumberOfDTCBySeverityMaskRecord {
                        avl_mask,
                        fid,
                        count
                    })
                },
                DTCReportType::ReportDTCBySeverityMaskRecord => {
                    utils::data_length_check(data_len, offset + 7, false)?;

                    let avl_mask = data[offset];
                    offset += 1;
                    if (data_len - offset) % 6 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let severity = data[offset];
                    offset += 1;
                    let func_unit = data[offset];
                    offset += 1;
                    let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    let mut others = Vec::new();
                    while data_len > offset {
                        let severity = data[offset];
                        offset += 1;
                        let func_unit = data[offset];
                        offset += 1;
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        others.push(DTCAndSeverityRecord1 {
                            severity, func_unit, dtc, status
                        })
                    }

                    Ok(Self::ReportDTCBySeverityMaskRecord {
                        avl_mask,
                        record: DTCAndSeverityRecord1 {
                            severity, func_unit, dtc, status
                        },
                        others,
                    })
                },
                DTCReportType::ReportSeverityInformationOfDTC => {
                    let avl_mask = data[offset];
                    offset += 1;
                    if (data_len - offset) % 6 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let mut records = Vec::new();
                    while data_len > offset {
                        let severity = data[offset];
                        offset += 1;
                        let func_unit = data[offset];
                        offset += 1;
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndSeverityRecord1 {
                            severity, func_unit, dtc, status
                        })
                    }

                    Ok(Self::ReportSeverityInformationOfDTC {
                        avl_mask, records
                    })
                },
                DTCReportType::ReportSupportedDTC => {
                    utils::data_length_check(data_len, offset + 5, false)?;
                    if (data_len - offset - 1) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let avl_mask = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportSupportedDTC {
                        avl_mask,
                        records,
                    })
                },
                DTCReportType::ReportFirstTestFailedDTC => {
                    utils::data_length_check(data_len, offset + 5, false)?;

                    let avl_mask = data[offset];
                    offset += 1;
                    if (data_len - offset) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let mut record = None;
                    if data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];

                        record = Some(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportMostRecentConfirmedDTC {
                        avl_mask,
                        record,
                    })
                },
                DTCReportType::ReportFirstConfirmedDTC => {
                    utils::data_length_check(data_len, offset + 5, false)?;

                    let avl_mask = data[offset];
                    offset += 1;
                    if (data_len - offset) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let mut record = None;
                    if data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];

                        record = Some(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportMostRecentConfirmedDTC {
                        avl_mask,
                        record,
                    })
                },
                DTCReportType::ReportMostRecentTestFailedDTC => {
                    utils::data_length_check(data_len, offset + 5, false)?;

                    let avl_mask = data[offset];
                    offset += 1;
                    if (data_len - offset) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let mut record = None;
                    if data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];

                        record = Some(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportMostRecentConfirmedDTC {
                        avl_mask,
                        record,
                    })
                },
                DTCReportType::ReportMostRecentConfirmedDTC => {
                    utils::data_length_check(data_len, offset + 5, false)?;
                    if (data_len - offset - 1) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let avl_mask = data[offset];
                    offset += 1;

                    let mut record = None;
                    if data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];

                        record = Some(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportMostRecentConfirmedDTC {
                        avl_mask,
                        record,
                    })
                },
                DTCReportType::ReportDTCFaultDetectionCounter => {
                    utils::data_length_check(data_len, 4, false)?;
                    if (data_len - offset) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                    offset += 3;
                    let counter = data[offset];
                    offset += 1;

                    let mut others = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let counter = data[offset];
                        offset += 1;

                        others.push(DTCFaultDetectionCounterRecord {
                            dtc, counter
                        });
                    }

                    Ok(Self::ReportDTCFaultDetectionCounter {
                        record: DTCFaultDetectionCounterRecord {
                            dtc, counter
                        },
                        others,
                    })
                },
                DTCReportType::ReportDTCWithPermanentStatus => {
                    utils::data_length_check(data_len, offset + 5, false)?;
                    if (data_len - offset - 1) % 4 != 0{
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let avl_mask = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportDTCWithPermanentStatus {
                        avl_mask,
                        records,
                    })
                },
                #[cfg(any(feature = "std2013", feature = "std2020"))]
                DTCReportType::ReportDTCExtDataRecordByRecordNumber => {
                    Err(Error::OtherError("This library does not yet support".to_string()))
                },
                #[cfg(any(feature = "std2013", feature = "std2020"))]
                DTCReportType::ReportUserDefMemoryDTCByStatusMask => {
                    utils::data_length_check(data_len, offset + 2, false)?;
                    if (data_len - offset - 2) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let mem_selection = data[offset];
                    offset += 1;
                    let avl_mask = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status })
                    }

                    Ok(Self::ReportUserDefMemoryDTCByStatusMask {
                        mem_selection, avl_mask, records,
                    })
                },
                #[cfg(any(feature = "std2013", feature = "std2020"))]
                DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber => {
                    utils::data_length_check(data_len, offset + 5, false)?;

                    let mem_selection = data[offset];
                    offset += 1;
                    let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let number = data[offset];
                        offset += 1;
                        let number_of_identifier = data[offset];
                        offset += 1;

                        let mut sub_records = Vec::new();
                        while sub_records.len() < number_of_identifier as usize {
                            utils::data_length_check(data_len, offset + 2, false)?;

                            let did = DataIdentifier::from(
                                u16::from_be_bytes([data[offset], data[offset + 1]])
                            );
                            offset += 2;
                            let did_data_len = *cfg.did_cfg.get(&did)
                                .ok_or(Error::DidNotSupported(did))?;

                            utils::data_length_check(data_len, offset + did_data_len, false)?;

                            sub_records.push((did, data[offset..offset + did_data_len].to_vec()));
                            offset += did_data_len;
                        }

                        records.push(UserDefDTCSnapshotRecord {
                            number,
                            number_of_identifier,
                            records: sub_records,
                        });
                    }

                    Ok(Self::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                        mem_selection,
                        status_record: DTCAndStatusRecord { dtc, status },
                        records,
                    })
                },
                #[cfg(any(feature = "std2013", feature = "std2020"))]
                DTCReportType::ReportUserDefMemoryDTCExtDataRecordByDTCNumber => {
                    utils::data_length_check(data_len, offset + 5, false)?;

                    let mem_selection = data[offset];
                    offset += 1;
                    let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    let mut number = None;
                    let mut records = Vec::new();
                    if data_len > offset {
                        let value = data[offset];
                        offset += 1;
                        if value > 0xFE {
                            return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                        }

                        number = Some(value);

                        while data_len > offset {
                            // TODO add to configuration
                            return Err(Error::OtherError("This library does not yet support".to_string()))
                        }
                    }

                    Ok(Self::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                        mem_selection,
                        status_record: DTCAndStatusRecord { dtc, status },
                        number,
                        records,
                    })
                },
                #[cfg(any(feature = "std2020"))]
                DTCReportType::ReportSupportedDTCExtDataRecord => {
                    utils::data_length_check(data_len, offset + 1, false)?;

                    let avl_mask = data[offset];
                    offset += 1;

                    let number = if data_len > offset {
                        let result = data[offset];
                        offset += 1;
                        result
                    }
                    else {
                        0
                    };

                    if number > 0xFD {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }
                    utils::data_length_check(data_len, offset + 4 * number as usize, false)?;

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status })
                    }

                    Ok(Self::ReportSupportedDTCExtDataRecord {
                        avl_mask, number, records,
                    })
                },
                #[cfg(any(feature = "std2013", feature = "std2020"))]
                DTCReportType::ReportWWHOBDDTCByMaskRecord => {
                    utils::data_length_check(data_len, offset + 4, false)?;
                    if (data_len - offset - 4) % 5 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let func_gid = data[offset];
                    offset += 1;
                    if func_gid > 0xFE {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let status_avl_mask = data[offset];
                    offset += 1;
                    let severity_avl_mask = data[offset];
                    offset += 1;
                    let fid = DTCFormatIdentifier::try_from(data[offset])?;
                    offset += 1;
                    match fid {
                        DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                        DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 => {}
                        _ => return Err(Error::InvalidData(utils::hex_slice_to_string(data))),
                    }

                    let mut records = Vec::new();
                    while data_len > offset {
                        let severity = data[offset];
                        offset += 1;
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndSeverityRecord { severity, dtc, status });
                    }

                    Ok(Self::ReportWWHOBDDTCByMaskRecord {
                        func_gid, status_avl_mask, severity_avl_mask, fid, records,
                    })
                },
                #[cfg(any(feature = "std2013", feature = "std2020"))]
                DTCReportType::ReportWWHOBDDTCWithPermanentStatus => {
                    utils::data_length_check(data_len, offset + 3, false)?;
                    if (data_len - offset - 3) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let func_gid = data[offset];
                    offset += 1;
                    if func_gid > 0xFE {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let status_avl_mask = data[offset];
                    offset += 1;
                    let fid = DTCFormatIdentifier::try_from(data[offset])?;
                    offset += 1;
                    match fid {
                        DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                        DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 => {}
                        _ => return Err(Error::InvalidData(utils::hex_slice_to_string(data))),
                    }

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportWWHOBDDTCWithPermanentStatus {
                        func_gid, status_avl_mask, fid, records,
                    })
                },
                #[cfg(any(feature = "std2020"))]
                DTCReportType::ReportDTCInformationByDTCReadinessGroupIdentifier => {
                    utils::data_length_check(data_len, offset + 4, false)?;
                    if (data_len - offset - 4) % 4 != 0 {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let func_gid = data[offset];
                    offset += 1;
                    if func_gid > 0xFE {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let status_avl_mask = data[offset];
                    offset += 1;
                    let format_identifier = data[offset];
                    offset += 1;

                    let readiness_gid = data[offset];
                    offset += 1;
                    if readiness_gid > 0xFE {
                        return Err(Error::InvalidData(utils::hex_slice_to_string(data)));
                    }

                    let mut records = Vec::new();
                    while data_len > offset {
                        let dtc = U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
                        offset += 3;
                        let status = data[offset];
                        offset += 1;

                        records.push(DTCAndStatusRecord { dtc, status });
                    }

                    Ok(Self::ReportDTCInformationByDTCReadinessGroupIdentifier {
                        func_gid, status_avl_mask, format_identifier, readiness_gid, records,
                    })
                },
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
            Self::ReportNumberOfDTCByStatusMask { avl_mask, fid, count } => {
                result.push(avl_mask);
                result.push(fid.into());
                result.extend(count.to_be_bytes());
            },
            Self::ReportDTCByStatusMask { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportMirrorMemoryDTCByStatusMask { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportNumberOfMirrorMemoryDTCByStatusMask { avl_mask, fid, count } => {
                result.push(avl_mask);
                result.push(fid.into());
                result.extend(count.to_be_bytes());
            },
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportNumberOfEmissionsOBDDTCByStatusMask { avl_mask, fid, count } => {
                result.push(avl_mask);
                result.push(fid.into());
                result.extend(count.to_be_bytes());
            },
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportEmissionsOBDDTCByStatusMask { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            Self::ReportDTCSnapshotIdentification { records } => {
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.number);
                    });
            },
            Self::ReportDTCSnapshotRecordByDTCNumber { status_record, records } => {
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                records.into_iter()
                    .for_each(|v| {
                        result.push(v.number);
                        result.push(v.number_of_identifier);
                        v.records.into_iter()
                            .for_each(|(did, mut data)| {
                                let did: u16 = did.into();
                                result.extend(did.to_be_bytes());
                                result.append(&mut data);
                            })
                    });
            },
            Self::ReportDTCStoredDataByRecordNumber { number, status_records } => {
                result.push(number);
                status_records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.status_record.dtc.into());
                        result.push(v.status_record.status);
                        result.push(v.number_of_identifier);
                        v.records.into_iter()
                            .for_each(|(did, mut data)| {
                                let did: u16 = did.into();
                                result.extend(did.to_be_bytes());
                                result.append(&mut data);
                            })
                    })
            },
            Self::ReportDTCExtDataRecordByDTCNumber { status_record, records } => {
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                records.into_iter()
                    .for_each(|mut v| {
                        result.push(v.number);
                        result.append(&mut v.data);
                    });
            },
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Self::ReportMirrorMemoryDTCExtDataRecordByDTCNumber { status_record, records } => {
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                records.into_iter()
                    .for_each(|mut v| {
                        result.push(v.number);
                        result.append(&mut v.data);
                    });
            },
            Self::ReportNumberOfDTCBySeverityMaskRecord { avl_mask, fid, count } => {
                result.push(avl_mask);
                result.push(fid.into());
                result.extend(count.to_be_bytes());
            },
            Self::ReportDTCBySeverityMaskRecord { avl_mask, record, others } => {
                result.push(avl_mask);
                result.push(record.severity);
                result.push(record.func_unit);
                result.append(&mut record.dtc.into());
                result.push(record.status);

                others.into_iter()
                    .for_each(|v| {
                        result.push(v.severity);
                        result.push(v.func_unit);
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            Self::ReportSeverityInformationOfDTC { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter()
                    .for_each(|v| {
                        result.push(v.severity);
                        result.push(v.func_unit);
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            Self::ReportSupportedDTC { avl_mask, records, } => {
                result.push(avl_mask);
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            Self::ReportFirstTestFailedDTC { avl_mask, record } |
            Self::ReportFirstConfirmedDTC { avl_mask, record } |
            Self::ReportMostRecentTestFailedDTC { avl_mask, record } |
            Self::ReportMostRecentConfirmedDTC { avl_mask, record } => {
                result.push(avl_mask);
                if let Some(v) = record {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                }
            },
            Self::ReportDTCFaultDetectionCounter { record, others } => {
                result.append(&mut record.dtc.into());
                result.push(record.counter);
                others.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.counter);
                    });
            },
            Self::ReportDTCWithPermanentStatus { avl_mask, records, } => {
                result.push(avl_mask);
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportDTCExtDataRecordByRecordNumber { ext_number, records } => {
                result.push(ext_number);
                records.into_iter()
                    .for_each(|mut v| {
                        result.append(&mut v.status_record.dtc.into());
                        result.push(v.status_record.status);
                        result.append(&mut v.data);
                    })
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportUserDefMemoryDTCByStatusMask { mem_selection, avl_mask, records } => {
                result.push(mem_selection);
                result.push(avl_mask);
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber { mem_selection, status_record, records } => {
                result.push(mem_selection);
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                records.into_iter()
                    .for_each(|v| {
                        result.push(v.number);
                        result.push(v.number_of_identifier);
                        v.records.into_iter()
                            .for_each(|(did, mut data)| {
                                let did: u16 = did.into();
                                result.extend(did.to_be_bytes());
                                result.append(&mut data);
                            });
                    });
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportUserDefMemoryDTCExtDataRecordByDTCNumber { mem_selection, status_record, number, records } => {
                result.push(mem_selection);
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                if let Some(v) = number {
                    result.push(v);
                }
                records.into_iter()
                    .for_each(|mut v| result.append(&mut v));
            },
            #[cfg(any(feature = "std2020"))]
            Self::ReportSupportedDTCExtDataRecord { avl_mask, number, records } => {
                result.push(avl_mask);
                result.push(number);
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportWWHOBDDTCByMaskRecord {
                func_gid,    // 00 to FE
                status_avl_mask,
                severity_avl_mask,
                fid,
                records,
            } => {
                result.push(func_gid);
                result.push(status_avl_mask);
                result.push(severity_avl_mask);
                result.push(fid.into());
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Self::ReportWWHOBDDTCWithPermanentStatus {
                func_gid,    // 00 to FE
                status_avl_mask,
                fid,
                records,
            } => {
                result.push(func_gid);
                result.push(status_avl_mask);
                result.push(fid.into());
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
            #[cfg(any(feature = "std2020"))]
            Self::ReportDTCInformationByDTCReadinessGroupIdentifier {
                func_gid,    // 00 to FE
                status_avl_mask,
                format_identifier,
                readiness_gid,     // 00 to FE
                records,
            } => {
                result.push(func_gid);
                result.push(status_avl_mask);
                result.push(format_identifier);
                result.push(readiness_gid);
                records.into_iter()
                    .for_each(|v| {
                        result.append(&mut v.dtc.into());
                        result.push(v.status);
                    });
            },
        }

        result
    }
}

