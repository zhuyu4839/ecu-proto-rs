//! start from page 49

mod cal;
pub use cal::*;
mod daq;
pub use daq::*;
mod page;
pub use page::*;
mod pgm;
pub use pgm::*;
mod standard;
pub use standard::*;
mod c0;
pub use c0::*;
mod time_correlation;
pub use time_correlation::*;

use crate::{AddressGranularity, CalPageMode, Command, SegmentInfoMode, SegmentMode, IntoWith, XcpError};

#[derive(Debug, Clone)]
pub struct Request {
    data: Vec<u8>,
}

impl Request {
    pub fn new(data: Vec<u8>) -> Result<Self, XcpError> {
        if data.is_empty() {
            return Err(XcpError::InvalidDataLength { expected: 1, actual: data.len() });
        }

        Ok(Self { data })
    }

    pub fn connect(mode: ConnectMode) -> Self {
        let mut result = vec![Command::Connect.into(), ];
        let request = Connect::new(mode);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn disconnect() -> Self {
        Self { data: vec![Command::Disconnect.into(), ] }
    }

    pub fn get_status() -> Self {
        Self { data: vec![Command::GetStatus.into(), ] }
    }

    pub fn synch() -> Self {
        Self { data: vec![Command::Synch.into(), ] }
    }

    pub fn get_command_mode_info() -> Self {
        Self { data: vec![Command::GetCommandModeInfo.into(), ] }
    }

    pub fn get_id(id_type: IdType) -> Self {
        let mut result = vec![Command::GetId.into(), ];
        let request = GetId::new(id_type);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn set_request(mode: RequestMode, session_cfg_id: u16) -> Self {
        let mut result = vec![Command::SetRequest.into(), ];
        let request = SetRequest::new(mode, session_cfg_id);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_seed(mode: SeedMode, resource: u8) -> Self {
        let mut result = vec![Command::GetSeed.into(), ];
        let request = GetSeed::new(mode, resource);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn unlock(remain_length: u8, key: Vec<u8>) -> Self {
        let mut result = vec![Command::Unlock.into(), ];
        let request = Unlock::new(remain_length, key);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn set_meta(address_extension: u8, address: u32) -> Self {
        let mut result = vec![Command::SetMeta.into(), ];
        let request = SetMeta::new(address_extension, address);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn upload(size: u8) -> Self {
        let mut result = vec![Command::Upload.into(), ];
        let request = Upload::new(size);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn short_upload(size: u8, address_extension: u8, address: u32) -> Self {
        let mut result = vec![Command::ShortUpload.into(), ];
        let request = ShortUpload::new(size, address_extension, address);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn build_checksum(block_size: u32) -> Self {
        let mut result = vec![Command::BuildChecksum.into(), ];
        let request = BuildChecksum::new(block_size);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn transport_layer(sub_cmd: u8, param: Vec<u8>) -> Self {
        let mut result = vec![Command::TransportLayer.into(), ];
        let request = TransportLayer::new(sub_cmd, param);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_version() -> Self {      // 0xC0
        let result = vec![Command::C0.into(), CmdCode::GetVersion.into()];

        Self { data: result }
    }

    pub fn user_command(sub_cmd: u8, param: Vec<u8>) -> Self {
        let mut result = vec![Command::UserCmd.into(), ];
        let request = UserDefine::new(sub_cmd, param);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn cal_download(
        remain_size: u8,
        elements: Vec<u8>,
        ag: AddressGranularity
    ) -> Result<Self, XcpError> {
        let mut result = vec![Command::CALDownload.into(), ];
        let request = Download::new(remain_size, elements);
        result.append(&mut request.into_with(ag));

        Ok(Self { data: result })
    }

    pub fn cal_download_next(
        remain_size: u8,
        elements: Vec<u8>,
        ag: AddressGranularity
    ) -> Result<Self, XcpError> {
        let mut result = vec![Command::CALDownloadNext.into(), ];
        let request = DownloadNext::new(remain_size, elements);
        result.append(&mut request.into_with(ag));

        Ok(Self { data: result })
    }

    pub fn cal_download_max(elements: Vec<u8>, ag: AddressGranularity) -> Result<Self, XcpError> {
        let mut result = vec![Command::CALDownloadMax.into(), ];
        let request = DownloadMax::new(elements);
        result.append(&mut request.into_with(ag));

        Ok(Self { data: result })
    }

    pub fn cal_short_download(address_extension: u8, address: u32, elements: Vec<u8>) -> Self {
        let mut result = vec![Command::CALShortDownload.into(), ];
        let request = ShortDownload::new(address_extension, address, elements);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn cal_modify_bits(shift_value: u8, and_mask: u16, xor_mask: u16) -> Self {
        let mut result = vec![Command::CALModifyBits.into(), ];
        let request = ModifyBits::new(shift_value, and_mask, xor_mask);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn page_set_cal_page(mode: CalPageMode, segment: u8, page: u8) -> Self {
        let mut result = vec![Command::PAGSetCalPage.into(), ];
        let request = SetCalPage::new(mode, segment, page);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn page_get_cal_page(mode: CalPageMode, segment: u8) -> Result<Self, XcpError> {
        let mut result = vec![Command::PAGGetCalPage.into()];
        let request = GetCalPage::new(mode, segment)?;
        result.append(&mut request.into());

        Ok(Self { data: result })
    }

    pub fn page_get_page_processor_info() -> Self {
        Self { data: vec![Command::PAGGetPageProcessorInfo.into(), ] }
    }

    pub fn page_get_segment_info(mode: SegmentInfoMode, size: u8, info: u8, mapping_index: u8) -> Self {
        let mut result = vec![Command::PAGGetSegmentInfo.into(), ];
        let request = GetSegmentInfo::new(mode, size, info, mapping_index);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn page_get_page_info(segment_number: u8, page_number: u8) -> Self {
        let mut result = vec![Command::PAGGetPageInfo.into(), ];
        let request = GetPageInfo::new(segment_number, page_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn page_set_segment_mode(mode: SegmentMode, number: u8) -> Self {
        let mut result = vec![Command::PAGSetSegmentMode.into(), ];
        let request = SetSegmentMode::new(mode, number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn page_get_segment_mode(number: u8) -> Self {
        let mut result = vec![Command::PAGGetSegmentMode.into(), ];
        let request = GetSegmentMode::new(number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn page_copy_cal_page(
        src_segment_number: u8,
        src_page_number: u8,
        dist_segment_number: u8,
        dist_page_number: u8
    ) -> Self {
        let mut result = vec![Command::PAGCopyCalPage.into(), ];
        let request = CopyCalPage::new(
            src_segment_number,
            src_page_number,
            dist_segment_number,
            dist_page_number
        );
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn daq_set_ptr() -> Self {
        todo!()
    }

    pub fn daq_write() -> Self {
        todo!()
    }

    pub fn daq_set_list_mode() -> Self {
        todo!()
    }

    pub fn daq_set_start_stop_list() -> Self {
        todo!()
    }

    pub fn daq_set_start_stop_synch() -> Self {
        todo!()
    }

    pub fn daq_write_multiple() -> Self {
        todo!()
    }

    pub fn daq_read() -> Self {
        todo!()
    }

    pub fn daq_get_clock() -> Self {
        todo!()
    }

    pub fn daq_get_processor_info() -> Self {
        todo!()
    }

    pub fn daq_get_resolution_info() -> Self {
        todo!()
    }

    pub fn daq_get_list_mode() -> Self {
        todo!()
    }

    pub fn daq_get_event_info() -> Self {
        todo!()
    }

    pub fn daq_dto_ctr_property() -> Self {
        todo!()
    }

    pub fn daq_clear_list() -> Self {
        todo!()
    }

    pub fn daq_get_list_info() -> Self {
        todo!()
    }

    pub fn daq_free() -> Self {
        todo!()
    }

    pub fn daq_alloc() -> Self {
        todo!()
    }

    pub fn daq_alloc_odt() -> Self {
        todo!()
    }

    pub fn daq_alloc_odt_entry() -> Self {
        todo!()
    }

    pub fn daq_set_packed_mode() -> Self {  // 0xC0
        todo!()
    }

    pub fn daq_get_packed_mode() -> Self {  // 0xC0
        todo!()
    }

    pub fn program_start() -> Self {
        todo!()
    }

    pub fn program_clear() -> Self {
        todo!()
    }

    pub fn program() -> Self {
        todo!()
    }

    pub fn program_reset() -> Self {
        todo!()
    }

    pub fn program_get_processor_info() -> Self {
        todo!()
    }

    pub fn program_get_selector_info() -> Self {
        todo!()
    }

    pub fn program_prepare() -> Self {
        todo!()
    }

    pub fn program_format() -> Self {
        todo!()
    }

    pub fn program_next() -> Self {
        todo!()
    }

    pub fn program_max() -> Self {
        todo!()
    }

    pub fn program_verify() -> Self {
        todo!()
    }

    pub fn time_correlation_set_property() -> Self {
        todo!()
    }

    pub fn debug_over_xcp() -> Self {   // 0xC0
        todo!()
    }

    pub fn pod_bs() -> Self {           // 0xC0
        todo!()
    }

    #[inline(always)]
    pub fn command(&self) -> Result<Command, XcpError> {
        Command::try_from(self.data[0])
    }

    pub fn origin_data<T>(&self) -> Result<T, XcpError> {
        // let offset = 1;
        let pid = self.command()?;
        match pid {
            Command::Connect => {}
            Command::Disconnect
            | Command::GetStatus
            | Command::Synch
            | Command::GetCommandModeInfo
            | Command::PAGGetPageProcessorInfo => {}
            Command::GetId => {}
            Command::SetRequest => {}
            Command::GetSeed => {}
            Command::Unlock => {}
            Command::SetMeta => {}
            Command::Upload => {}
            Command::ShortUpload => {}
            Command::BuildChecksum => {}
            Command::TransportLayer => {}
            Command::UserCmd => {}
            Command::CALDownload => {}
            Command::CALDownloadNext => {}
            Command::CALDownloadMax => {}
            Command::CALShortDownload => {}
            Command::CALModifyBits => {}
            Command::PAGSetCalPage => {}
            Command::PAGGetCalPage => {}
            Command::PAGGetSegmentInfo => {}
            Command::PAGGetPageInfo => {}
            Command::PAGSetSegmentMode => {}
            Command::PAGGetSegmentMode => {}
            Command::PAGCopyCalPage => {}
            Command::DAQSetPtr => {}
            Command::DAQWrite => {}
            Command::DAQSetListMode => {}
            Command::DAQStartStopList => {}
            Command::DAQStartStopSynch => {}
            Command::DAQWriteMultiple => {}
            Command::DAQRead => {}
            Command::DAQGetClock => {}
            Command::DAQGetProcessorInfo => {}
            Command::DAQGetResolutionInfo => {}
            Command::DAQGetListMode => {}
            Command::DAQGetEventInfo => {}
            Command::DAQDTOCTRProperty => {}
            Command::DAQClearList => {}
            Command::DAQGetListInfo => {}
            Command::DAQFree => {}
            Command::DAQAlloc => {}
            Command::DAQAllocODT => {}
            Command::DAQAllocODTEntry => {}
            Command::PGMPrgStart => {}
            Command::PGMPrgClear => {}
            Command::PGMPrg => {}
            Command::PGMPrgReset => {}
            Command::PGMGetProcessorInfo => {}
            Command::PGMGetSectorInfo => {}
            Command::PGMPrgPrepare => {}
            Command::PGMPrgFormat => {}
            Command::PGMPrgNext => {}
            Command::PGMPrgMAX => {}
            Command::PGMPrgVerify => {}
            Command::TimeCorrelationProperty => {}
            Command::C0 => {}
        }

        todo!()
    }
}

impl Into<Vec<u8>> for Request {
    #[inline]
    fn into(self) -> Vec<u8> {
        self.data
    }
}
