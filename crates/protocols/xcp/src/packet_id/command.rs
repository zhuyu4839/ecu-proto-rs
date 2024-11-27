use crate::XcpError;

/// XCP command define
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Command {
    /* start of Standard */
    Connect = 0xFF,                 // ✅
    Disconnect = 0xFE,              // ✅
    GetStatus = 0xFD,               // ✅
    Synch = 0xFC,                   // ✅
    // optional
    GetCommandModeInfo = 0xFB,      // ✅
    GetId = 0xFA,                   // ✅
    SetRequest = 0xF9,              // ✅
    GetSeed = 0xF8,                 // ✅
    Unlock = 0xF7,                  // ✅
    SetMeta = 0xF6,                 // ✅
    Upload = 0xF5,                  // ✅
    ShortUpload = 0xF4,             // ✅
    BuildChecksum = 0xF3,           // ✅ nonstandard negative
    TransportLayer = 0xF2,          // ✅
    UserCmd = 0xF1,                 // ✅
    /* end of Standard */

    /* start of calibration */
    CALDownload = 0xF0,             // ✅
    // optional
    CALDownloadNext = 0xEF,         // ✅
    CALDownloadMax = 0xEE,          // ✅
    CALShortDownload = 0xED,        // ✅
    CALModifyBits = 0xEC,           // ✅
    /* end of calibration */

    /* start of page switch */
    // optional
    PAGSetCalPage = 0xEB,           // ✅
    PAGGetCalPage = 0xEA,           // ✅
    PAGGetPageProcessorInfo = 0xE9, // ✅
    PAGGetSegmentInfo = 0xE8,       // ✅
    PAGGetPageInfo = 0xE7,          // ✅
    PAGSetSegmentMode = 0xE6,       // ✅
    PAGGetSegmentMode = 0xE5,       // ✅
    PAGCopyCalPage = 0xE4,          // ✅
    /* end of page switch */

    /* start of DAQ & STIM */
    DAQSetPtr = 0xE2,               // ✅
    DAQWrite = 0xE1,                // ✅
    DAQSetListMode = 0xE0,          // ✅
    DAQStartStopList = 0xDE,        // ✅
    DAQStartStopSynch = 0xDD,       // ✅
    // optional
    DAQWriteMultiple = 0xC7,        // ✅
    DAQRead = 0xDB,                 // ✅
    DAQGetClock = 0xDC,             // ✅
    DAQGetProcessorInfo = 0xDA,     // ✅
    DAQGetResolutionInfo = 0xD9,    // ✅
    DAQGetListMode = 0xDF,          // ✅
    DAQGetEventInfo = 0xD7,         // ✅
    DAQDTOCTRProperty = 0xC5,       // ✅

    // static configuration
    DAQClearList = 0xE3,            // ✅
    // optional
    DAQGetListInfo = 0xD8,          // ✅

    // Dynamic configuration
    DAQFree = 0xD6,                 // ✅
    DAQAlloc = 0xD5,                // ✅
    DAQAllocODT = 0xD4,             // ✅
    DAQAllocODTEntry = 0xD3,        // ✅
    /* end of DAQ & STIM */

    /* start of nonvolatile memory program */
    PGMPrgStart = 0xD2,             // ✅
    PGMPrgClear = 0xD1,             // ✅
    PGMPrg = 0xD0,                  // TODO page 209
    PGMPrgReset = 0xCF,             // ✅ page 211
    // optional
    PGMGetProcessorInfo = 0xCE,     // ✅ page 212
    PGMGetSectorInfo = 0xCD,        // ✅ page 215
    PGMPrgPrepare = 0xCC,           // ✅ page 217
    PGMPrgFormat = 0xCB,            // ✅ page 218
    PGMPrgNext = 0xCA,              // ✅ page 220
    PGMPrgMAX = 0xC9,               // ✅ page 221
    PGMPrgVerify = 0xC8,            // ✅ page 222
    /* end of nonvolatile memory program */

    TimeCorrelationProperty = 0xC6, // page 223

    C0 = 0xC0,
}

impl Into<u8> for Command {
    #[inline]
    fn into(self) -> u8 {
        self as _
    }
}

impl TryFrom<u8> for Command {
    type Error = XcpError;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0xFF => Ok(Self::Connect),
            0xFE => Ok(Self::Disconnect),
            0xFD => Ok(Self::GetStatus),
            0xFC => Ok(Self::Synch),
            0xFB => Ok(Self::GetCommandModeInfo),
            0xFA => Ok(Self::GetId),
            0xF9 => Ok(Self::SetRequest),
            0xF8 => Ok(Self::GetSeed),
            0xF7 => Ok(Self::Unlock),   // page 130
            0xF6 => Ok(Self::SetMeta),
            0xF5 => Ok(Self::Upload),
            0xF4 => Ok(Self::ShortUpload),
            0xF3 => Ok(Self::BuildChecksum),
            0xF2 => Ok(Self::TransportLayer),
            0xF1 => Ok(Self::UserCmd),

            0xF0 => Ok(Self::CALDownload),
            0xEF => Ok(Self::CALDownloadNext),
            0xEE => Ok(Self::CALDownloadMax),
            0xED => Ok(Self::CALShortDownload),
            0xEC => Ok(Self::CALModifyBits),

            0xEB => Ok(Self::PAGSetCalPage),
            0xEA => Ok(Self::PAGGetCalPage),
            0xE9 => Ok(Self::PAGGetPageProcessorInfo),
            0xE8 => Ok(Self::PAGGetSegmentInfo),
            0xE7 => Ok(Self::PAGGetPageInfo),
            0xE6 => Ok(Self::PAGSetSegmentMode),
            0xE5 => Ok(Self::PAGGetSegmentMode),
            0xE4 => Ok(Self::PAGCopyCalPage),

            0xE2 => Ok(Self::DAQSetPtr),
            0xE1 => Ok(Self::DAQWrite),
            0xE0 => Ok(Self::DAQSetListMode),
            0xDE => Ok(Self::DAQStartStopList),
            0xDD => Ok(Self::DAQStartStopSynch),
            0xC7 => Ok(Self::DAQWriteMultiple),
            0xDB => Ok(Self::DAQRead),
            0xDC => Ok(Self::DAQGetClock),
            0xDA => Ok(Self::DAQGetProcessorInfo),
            0xD9 => Ok(Self::DAQGetResolutionInfo),
            0xDF => Ok(Self::DAQGetListMode),
            0xD7 => Ok(Self::DAQGetEventInfo),
            0xC5 => Ok(Self::DAQDTOCTRProperty),

            0xE3 => Ok(Self::DAQClearList),
            0xD8 => Ok(Self::DAQGetListInfo),
            0xD6 => Ok(Self::DAQFree),
            0xD5 => Ok(Self::DAQAlloc),
            0xD4 => Ok(Self::DAQAllocODT),
            0xD3 => Ok(Self::DAQAllocODTEntry),

            0xD2 => Ok(Self::PGMPrgStart),
            0xD1 => Ok(Self::PGMPrgClear),
            0xD0 => Ok(Self::PGMPrg),
            0xCF => Ok(Self::PGMPrgReset),
            0xCE => Ok(Self::PGMGetProcessorInfo),
            0xCD => Ok(Self::PGMGetSectorInfo),
            0xCC => Ok(Self::PGMPrgPrepare),
            0xCB => Ok(Self::PGMPrgFormat),
            0xCA => Ok(Self::PGMPrgNext),
            0xC9 => Ok(Self::PGMPrgMAX),
            0xC8 => Ok(Self::PGMPrgVerify),

            0xC6 => Ok(Self::TimeCorrelationProperty),

            0xC0 => Ok(Self::C0),
            _ => Err(XcpError::UnknownCommand(byte))
        }
    }
}
