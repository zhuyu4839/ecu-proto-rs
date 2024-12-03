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

use crate::{AddressGranularity, CalPageMode, Command, SegmentInfoMode, SegmentMode, IntoWith, XcpError, DTOCTRPropertyMode, DAQPackedMode, DAQPackedModeData, GetSectorInfoMode, ProgrammingMethod};

#[derive(Debug, Clone)]
pub struct Request {
    data: Vec<u8>,
}

impl Request {
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

    pub fn download(
        remain_size: u8,
        elements: Vec<u8>,
        ag: AddressGranularity
    ) -> Result<Self, XcpError> {
        let mut result = vec![Command::CALDownload.into(), ];
        let request = Download::new(remain_size, elements);
        result.append(&mut request.into_with(ag));

        Ok(Self { data: result })
    }

    pub fn download_next(
        remain_size: u8,
        elements: Vec<u8>,
        ag: AddressGranularity
    ) -> Result<Self, XcpError> {
        let mut result = vec![Command::CALDownloadNext.into(), ];
        let request = DownloadNext::new(remain_size, elements);
        result.append(&mut request.into_with(ag));

        Ok(Self { data: result })
    }

    pub fn download_max(elements: Vec<u8>, ag: AddressGranularity) -> Result<Self, XcpError> {
        let mut result = vec![Command::CALDownloadMax.into(), ];
        let request = DownloadMax::new(elements);
        result.append(&mut request.into_with(ag));

        Ok(Self { data: result })
    }

    pub fn short_download(address_extension: u8, address: u32, elements: Vec<u8>) -> Self {
        let mut result = vec![Command::CALShortDownload.into(), ];
        let request = ShortDownload::new(address_extension, address, elements);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn modify_bits(shift_value: u8, and_mask: u16, xor_mask: u16) -> Self {
        let mut result = vec![Command::CALModifyBits.into(), ];
        let request = ModifyBits::new(shift_value, and_mask, xor_mask);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn set_cal_page(mode: CalPageMode, segment: u8, page: u8) -> Self {
        let mut result = vec![Command::PAGSetCalPage.into(), ];
        let request = SetCalPage::new(mode, segment, page);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_cal_page(mode: CalPageMode, segment: u8) -> Result<Self, XcpError> {
        let mut result = vec![Command::PAGGetCalPage.into()];
        let request = GetCalPage::new(mode, segment)?;
        result.append(&mut request.into());

        Ok(Self { data: result })
    }

    pub fn get_page_processor_info() -> Self {
        Self { data: vec![Command::PAGGetPageProcessorInfo.into(), ] }
    }

    pub fn get_segment_info(mode: SegmentInfoMode, size: u8, info: u8, mapping_index: u8) -> Self {
        let mut result = vec![Command::PAGGetSegmentInfo.into(), ];
        let request = GetSegmentInfo::new(mode, size, info, mapping_index);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_page_info(segment_number: u8, page_number: u8) -> Self {
        let mut result = vec![Command::PAGGetPageInfo.into(), ];
        let request = GetPageInfo::new(segment_number, page_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn set_segment_mode(mode: SegmentMode, number: u8) -> Self {
        let mut result = vec![Command::PAGSetSegmentMode.into(), ];
        let request = SetSegmentMode::new(mode, number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_segment_mode(number: u8) -> Self {
        let mut result = vec![Command::PAGGetSegmentMode.into(), ];
        let request = GetSegmentMode::new(number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn copy_cal_page(
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

    pub fn set_daq_ptr(daq_list_number: u16, odt_number: u8, odt_entry_number: u8) -> Self {
        let mut result = vec![Command::DAQSetPtr.into(), ];
        let request = SetDAQPtr::new(daq_list_number, odt_number, odt_entry_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn write_daq(bit_offset: u8, size: u8, address_extension: u8, address: u32) -> Self {
        let mut result = vec![Command::DAQWrite.into(), ];
        let request = WriteDAQ::new(bit_offset, size, address_extension, address);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn set_daq_list_mode(
        mode: DAQListModeSet,
        daq_list_number: u16,
        event_channel_number: u16,
        transmission_rate_prescaler: u8,
        daq_list_priority:  u8,
    ) -> Self {
        let mut result = vec![Command::DAQSetListMode.into(), ];
        let request = SetDAQListMode::new(
            mode,
            daq_list_number,
            event_channel_number,
            transmission_rate_prescaler,
            daq_list_priority,
        );
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn start_stop_list(mode: StartStopDaqListMode, daq_list_number: u16) -> Self {
        let mut result = vec![Command::DAQStartStopList.into(), ];
        let request = StartStopDAQList::new(mode, daq_list_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn start_stop_synch(mode: StartStopSynchMode) -> Self {
        let mut result = vec![Command::DAQStartStopSynch.into(), ];
        let request = StartStopSynch::new(mode);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn write_daq_multiple(elements: Vec<DAQElement>) -> Self {
        let mut result = vec![Command::DAQWriteMultiple.into(), ];
        let request = WriteDAQMultiple::new(elements);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn read_daq() -> Self {
        Self { data: vec![Command::DAQRead.into(), ] }
    }

    pub fn get_daq_clock() -> Self {
        Self { data: vec![Command::DAQGetClock.into(), ] }
    }

    pub fn get_daq_processor_info() -> Self {
        Self { data: vec![Command::DAQGetProcessorInfo.into(), ] }
    }

    pub fn get_daq_resolution_info() -> Self {
        Self { data: vec![Command::DAQGetResolutionInfo.into(), ] }
    }

    pub fn get_daq_list_mode(daq_list_number: u16) -> Self {
        let mut result = vec![Command::DAQGetListMode.into(), ];
        let request = GetDAQListMode::new(daq_list_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_daq_event_info(event_channel_number: u16) -> Self {
        let mut result = vec![Command::DAQGetEventInfo.into(), ];
        let request = GetDAQEventInfo::new(event_channel_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_daq_dto_ctr_property(
        modifier: DTOCTRPropertyModifier,
        event_channel_number: u16,
        related_event_channel_number: u16,
        mode: DTOCTRPropertyMode,
    ) -> Self {
        let mut result = vec![Command::DAQDTOCTRProperty.into(), ];
        let request = GetDTOCTRProperty::new(
            modifier,
            event_channel_number,
            related_event_channel_number,
            mode,
        );
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn clear_daq_list(daq_list_number: u16) -> Self {
        let mut result = vec![Command::DAQClearList.into(), ];
        let request = ClearDAQList::new(daq_list_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_daq_list_info(daq_list_number: u16) -> Self {
        let mut result = vec![Command::DAQGetListInfo.into(), ];
        let request = GetDAQListInfo::new(daq_list_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn free_daq() -> Self {
        Self { data: vec![Command::DAQFree.into(), ] }
    }

    pub fn alloc_daq(daq_count: u16) -> Self {
        let mut result = vec![Command::DAQAlloc.into(), ];
        let request = AllocDAQ::new(daq_count);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn alloc_daq_odt(daq_list_number: u16, odt_count: u8) -> Self {
        let mut result = vec![Command::DAQAllocODT.into(), ];
        let request = AllocODT::new(daq_list_number, odt_count);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn alloc_daq_odt_entry(daq_list_number: u16, odt_number: u8, odt_entry_count: u8) -> Self {
        let mut result = vec![Command::DAQAllocODTEntry.into(), ];
        let request = AllocODTEntry::new(daq_list_number, odt_number, odt_entry_count);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn set_daq_packed_mode(daq_list_number: u16) -> Self {  // 0xC0
        let mut result = vec![Command::C0.into(), CmdCode::GetDAQPackedMode.into()];
        let request = GetDAQPackedMode::new(daq_list_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn get_daq_packed_mode(
        daq_list_number: u16,
        mode: DAQPackedMode,
        content: Option<DAQPackedModeData>,
    ) -> Self {  // 0xC0
        let mut result = vec![Command::C0.into(), CmdCode::SetDAQPackedMode.into()];
        let request = SetDAQPackedMode::new(daq_list_number, mode, content);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn program_start() -> Self {
        Self { data: vec![Command::PGMPrgStart.into(), ] }
    }

    pub fn program_clear(mode: ProgramClearMode, clear_range: u32) -> Self {
        let mut result = vec![Command::PGMPrgClear.into(), ];
        let request = ProgramClear::new(mode, clear_range);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn program(remain_size: u8, elements: Vec<u8>) -> Self {
        let mut result = vec![Command::PGMPrg.into(), ];
        let request = Program::new(remain_size, elements);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn program_reset() -> Self {
        Self { data: vec![Command::PGMPrgReset.into(), ] }
    }

    pub fn get_program_processor_info() -> Self {
        Self { data: vec![Command::PGMGetProcessorInfo.into(), ] }
    }

    pub fn get_program_selector_info(mode: GetSectorInfoMode, sector_number: u8) -> Self {
        let mut result = vec![Command::PGMGetSectorInfo.into(), ];
        let request = GetProgramSectorInfo::new(mode, sector_number);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn program_prepare(code_size: u16) -> Self {
        let mut result = vec![Command::PGMPrgPrepare.into(), ];
        let request = ProgramPrepare::new(code_size);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn program_format(
        compression_method: CompressionMethod,
        encryption_method: EncryptionMethod,
        programming_method: ProgrammingMethod,
        access_mode: AccessMode,
    ) -> Result<Self, XcpError> {
        let mut result = vec![Command::PGMPrgFormat.into(), ];
        let request = ProgramFormat::new(
            compression_method,
            encryption_method,
            programming_method,
            access_mode,
        )?;
        result.append(&mut request.into());

        Ok(Self { data: result })
    }

    pub fn program_next(remain_size: u8, elements: Vec<u8>) -> Self {
        let mut result = vec![Command::PGMPrgNext.into(), ];
        let request = ProgramNext::new(remain_size, elements);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn program_max(elements: Vec<u8>) -> Self {
        let mut result = vec![Command::PGMPrgMAX.into(), ];
        let request = ProgramMax::new(elements);
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn program_verify(
        compression_method: CompressionMethod,
        encryption_method: EncryptionMethod,
        programming_method: ProgrammingMethod,
        access_mode: AccessMode,
    ) -> Result<Self, XcpError> {
        let mut result = vec![Command::PGMPrgVerify.into(), ];
        let request = ProgramFormat::new(
            compression_method,
            encryption_method,
            programming_method,
            access_mode,
        )?;
        result.append(&mut request.into());

        Ok(Self { data: result })
    }

    pub fn time_correlation_set_property(
        set_property: TimeCorrelationSetProperty,
        get_property_request: u8,
        cluster_id: u16,
    ) -> Self {
        let mut result = vec![Command::TimeCorrelationProperty.into(), ];
        let request = TimeCorrelationProperty::new(
            set_property,
            get_property_request,
            cluster_id,
        );
        result.append(&mut request.into());

        Self { data: result }
    }

    pub fn debug_over_xcp() -> Self {   // 0xC0
        Self { data: vec![Command::C0.into(), CmdCode::SwDbgOverXCP.into()] }
    }

    pub fn pod_bs() -> Self {           // 0xC0
        Self { data: vec![Command::C0.into(), CmdCode::PodBS.into()] }
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
