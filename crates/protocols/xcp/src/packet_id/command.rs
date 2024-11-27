use crate::XcpError;

/// XCP command define
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Command {
    /* start of Standard */
    Connect = 0xFF,                 // ✅ page 110
    Disconnect = 0xFE,              // ✅ page 115
    GetStatus = 0xFD,               // ✅ page 116
    Synch = 0xFC,                   // ✅ page 120
    // optional
    GetCommandModeInfo = 0xFB,      // ✅ page 121
    GetId = 0xFA,                   // ✅ page 123
    SetRequest = 0xF9,              // ✅ page 125
    GetSeed = 0xF8,                 // ✅ page 128
    Unlock = 0xF7,                  // ✅ page 130
    SetMeta = 0xF6,                 // ✅ page 133
    Upload = 0xF5,                  // ✅ page 134
    ShortUpload = 0xF4,             // ✅ page 136
    BuildChecksum = 0xF3,           // ✅ page 137 nonstandard negative
    TransportLayer = 0xF2,          // ✅ page 140
    UserCmd = 0xF1,                 // ✅ page 141
    /* end of Standard */

    /* start of calibration */
    CALDownload = 0xF0,             // ✅ page 142
    // optional
    CALDownloadNext = 0xEF,         // ✅ page 144
    CALDownloadMax = 0xEE,          // ✅ page 146
    CALShortDownload = 0xED,        // ✅ page 147
    CALModifyBits = 0xEC,           // ✅ page 148
    /* end of calibration */

    /* start of page switch */
    // optional
    PAGSetCalPage = 0xEB,           // ✅ page 149
    PAGGetCalPage = 0xEA,           // ✅ page 150
    PAGGetPageProcessorInfo = 0xE9, // ✅ page 151
    PAGGetSegmentInfo = 0xE8,       // ✅ page 152
    PAGGetPageInfo = 0xE7,          // ✅ page 155
    PAGSetSegmentMode = 0xE6,       // ✅ page 159
    PAGGetSegmentMode = 0xE5,       // ✅ page 160
    PAGCopyCalPage = 0xE4,          // ✅ page 161
    /* end of page switch */

    /* start of DAQ & STIM */
    DAQSetPtr = 0xE2,               // ✅ page 162
    DAQWrite = 0xE1,                // ✅ page 163
    DAQSetListMode = 0xE0,          // ✅ page 164
    DAQStartStopList = 0xDE,        // ✅ page 167
    DAQStartStopSynch = 0xDD,       // ✅ page 169
    // optional
    DAQWriteMultiple = 0xC7,        // ✅ page 170
    DAQRead = 0xDB,                 // ✅ page 175
    DAQGetClock = 0xDC,             // ✅ page 176
    DAQGetProcessorInfo = 0xDA,     // ✅ page 178
    DAQGetResolutionInfo = 0xD9,    // ✅ page 184
    DAQGetListMode = 0xDF,          // ✅ page 187
    DAQGetEventInfo = 0xD7,         // ✅ page 189
    DAQDTOCTRProperty = 0xC5,       // ✅ page 193

    // static configuration
    DAQClearList = 0xE3,            // ✅ page 198
    // optional
    DAQGetListInfo = 0xD8,          // ✅ page 199

    // Dynamic configuration
    DAQFree = 0xD6,                 // ✅ page 201
    DAQAlloc = 0xD5,                // ✅ page 202
    DAQAllocODT = 0xD4,             // ✅ page 203
    DAQAllocODTEntry = 0xD3,        // ✅ page 204
    /* end of DAQ & STIM */

    /* start of nonvolatile memory program */
    PGMPrgStart = 0xD2,             // ✅ page 205
    PGMPrgClear = 0xD1,             // ✅ page 207
    PGMPrg = 0xD0,                  // ✅ page 209
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

    TimeCorrelationProperty = 0xC6, // ✅ page 223

    /// get version page 114
    /// set daq package mode page 172
    /// get daq package mode page 174
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
