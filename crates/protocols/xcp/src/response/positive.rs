use getset::Getters;
use crate::{AddressGranularity, Command, DAQPackedMode, DAQPackedModeData, DTOCTRPropertyMode, IntoWith, PayloadFormat, ResourceStatus, SegmentMode, SyncState, TimestampUnit, XcpError};
use crate::response::{BuildChecksum, ChecksumType, CmdModeOpt, CommonMode, Connect, DAQEventProperty, DAQKeyByte, DAQListModeGet, DAQListProperty, DAQProperty, DTOCTRProperty, GetCalPage, GetCmdModeInfo, GetDAQClock, GetDAQEventInfo, GetDAQListInfo, GetDAQListMode, GetDAQPackedMode, GetDTOCTRProperty, GetDaqProcessorInfo, GetDaqResolutionInfo, GetId, GetIdMode, GetPageInfo, GetPageProcessorInfo, GetProgramProcessorInfo, GetProgramSectorInfo, GetSeed, GetSegmentInfo, GetSegmentMode, GetStatus, GetVersion, PageProperty, ProcessorInfoProperty, ProgramProperty, ProgramStart, ProgrammingMode, ReadDAQ, SessionStatus, StartStopDaqList, TimestampMode, Unlock, UnlockStatus, Upload, Version};

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct Positive {
    pub(crate) data: Vec<u8>,
}

impl Positive {
    pub fn common(command: Command) -> Option<Self> {
        match command {
            Command::C0
            | Command::DAQGetClock
            | Command::DAQGetEventInfo
            | Command::DAQGetListInfo
            | Command::DAQGetListMode
            | Command::DAQGetProcessorInfo
            | Command::DAQGetResolutionInfo
            | Command::DAQDTOCTRProperty
            | Command::DAQRead
            | Command::DAQStartStopList
            | Command::PAGGetCalPage
            | Command::PAGGetPageInfo
            | Command::PAGGetPageProcessorInfo
            | Command::PAGGetSegmentInfo
            | Command::PAGGetSegmentMode
            | Command::PGMGetProcessorInfo
            | Command::PGMGetSectorInfo
            | Command::PGMPrgStart
            | Command::BuildChecksum
            | Command::Connect
            | Command::GetCommandModeInfo
            | Command::GetId
            | Command::GetSeed
            | Command::GetStatus
            | Command::Unlock
            | Command::Upload => None,
            _ => Some(Self { data: vec![] })
        }
    }

    pub fn get_daq_packed_mode(
        mode: DAQPackedMode,
        data: Option<DAQPackedModeData>
    ) -> Result<Self, XcpError> {
        let response = GetDAQPackedMode::new(mode, data)?;
        Ok(Self { data: response.into() })
    }

    pub fn get_version(protocol_version: Version, transport_version: Version) -> Self {
        let response = GetVersion::new(protocol_version, transport_version);
        Self { data: response.into() }
    }

    pub fn get_daq_clock(
        trigger_info: u8,
        payload_fmt: PayloadFormat,
        timestamp: u64,
        dedicated_timestamp: Option<u64>,
        ecu_timestamp: Option<u64>,
        synch_state: Option<SyncState>,
    ) -> Result<Self, XcpError> {
        let response = GetDAQClock::new(
            trigger_info,
            payload_fmt,
            timestamp,
            dedicated_timestamp,
            ecu_timestamp,
            synch_state,
        )?;
        Ok(Self { data: response.into() })
    }

    pub fn get_daq_event_info(
        property: DAQEventProperty,
        max_daq_list: u8,
        channel_name_length: u8,
        channel_time_cycle: u8,
        channel_time_unit: TimestampUnit,
        channel_priority: u8,
    ) -> Self {
        let response = GetDAQEventInfo::new(
            property,
            max_daq_list,
            channel_name_length,
            channel_time_cycle,
            channel_time_unit,
            channel_priority
        );
        Self { data: response.into() }
    }

    pub fn get_daq_list_info(
        property: DAQListProperty,
        max_odt: u8,
        max_odt_entry: u8,
        fixed_event: u16,
    ) -> Self {
        let response = GetDAQListInfo::new(
            property,
            max_odt,
            max_odt_entry,
            fixed_event
        );
        Self { data: response.into() }
    }

    pub fn get_daq_list_mode(
        mode: DAQListModeGet,
        event_channel_number: u16,
        prescaler: u8,
        daq_list_priority: u8,
    ) -> Self {
        let response = GetDAQListMode::new(
            mode,
            event_channel_number,
            prescaler,
            daq_list_priority,
        );
        Self { data: response.into() }
    }

    pub fn get_daq_processor_info(
        property: DAQProperty,
        available_daq_list: u16,
        available_event_channel: u16,
        predefine_daq_list: u8,
        daq_key_byte: DAQKeyByte,
    ) -> Self {
        let response = GetDaqProcessorInfo::new(
            property,
            available_daq_list,
            available_event_channel,
            predefine_daq_list,
            daq_key_byte
        );
        Self { data: response.into() }
    }

    pub fn get_daq_resolution_info(
        daq_odt_entry_size: u8,
        daq_odt_max_size: u8,
        stim_odt_entry_size: u8,
        stim_odt_max_size: u8,
        timestamp_mode: TimestampMode,
        timestamp_ticks: u16,
    ) -> Self {
        let response = GetDaqResolutionInfo::new(
            daq_odt_entry_size,
            daq_odt_max_size,
            stim_odt_entry_size,
            stim_odt_max_size,
            timestamp_mode,
            timestamp_ticks,
        );
        Self { data: response.into() }
    }

    pub fn get_dto_ctr_property(
        property: DTOCTRProperty,
        related_event_channel_number: u16,
        mode: DTOCTRPropertyMode,
    ) -> Self {
        let response = GetDTOCTRProperty::new(
            property,
            related_event_channel_number,
            mode
        );
        Self { data: response.into() }
    }

    pub fn read_daq(bit_offset: u8, size: u8, address_extension: u8, address: u32) -> Self {
        let response = ReadDAQ::new(bit_offset, size, address_extension, address);
        Self { data: response.into() }
    }

    pub fn start_stop_daq_list(first_pid: u8) -> Self {
        let response = StartStopDaqList::new(first_pid);
        Self { data: response.into() }
    }

    pub fn get_cal_page(page_number: u8) -> Self {
        let response = GetCalPage::new(page_number);
        Self { data: response.into() }
    }

    pub fn get_page_info(property: PageProperty, init_segment: u8) -> Self {
        let response = GetPageInfo::new(property, init_segment);
        Self { data: response.into() }
    }

    pub fn get_page_processor_info(segments: u8, property: ProcessorInfoProperty) -> Self {
        let response = GetPageProcessorInfo::new(segments, property);
        Self { data: response.into() }
    }

    pub fn get_segment_info(info: GetSegmentInfo) -> Self {
        Self { data: info.into() }
    }

    pub fn get_segment_mode(mode: SegmentMode) -> Self {
        let response = GetSegmentMode::new(mode);
        Self { data: response.into() }
    }

    pub fn get_program_processor_info(property: ProgramProperty, max_sector: u8) -> Self {
        let response = GetProgramProcessorInfo::new(property, max_sector);
        Self { data: response.into() }
    }

    pub fn get_program_sector_info(info: GetProgramSectorInfo) -> Self {
        Self { data: info.into() }
    }

    pub fn program_start(
        mode: ProgrammingMode,
        max_cto: u8,
        max_bs: u8,
        st_min: u8,
        queue_size: u8,
    ) -> Self {
        let response = ProgramStart::new(mode, max_cto, max_bs, st_min, queue_size);
        Self { data: response.into() }
    }

    pub fn build_checksum(r#type: ChecksumType, checksum: u32) -> Self {
        let response = BuildChecksum::new(r#type, checksum);
        Self { data: response.into() }
    }

    pub fn connect(
        resource_status: ResourceStatus,
        comm_mode: CommonMode,
        max_cto: u8,
        max_dto: u16,
        protocol_version: u8,
        transport_version: u8,
    ) -> Self {
        let response = Connect::new(
            resource_status,
            comm_mode,
            max_cto,
            max_dto,
            protocol_version,
            transport_version,
        );
        Self { data: response.into() }
    }

    pub fn get_cmd_mode_info(
        cmd_mode_opt: CmdModeOpt,
        max_bs: u8,
        min_st: u8,
        q_size: u8,
        drv_ver: u8
    ) -> Self {
        let response = GetCmdModeInfo::new(
            cmd_mode_opt,
            max_bs,
            min_st,
            q_size,
            drv_ver,
        );
        Self { data: response.into() }
    }

    pub fn get_id(mode: GetIdMode, data: Vec<u8>) -> Self {
        let response = GetId::new(mode, data);
        Self { data: response.into() }
    }

    pub fn get_seed(remain_length: u8, seed: Vec<u8>) -> Self {
        let response = GetSeed::new(remain_length, seed);
        Self { data: response.into() }
    }

    pub fn get_status(
        session_status: SessionStatus,
        resource_status: ResourceStatus,
        state_number: u8,
        session_cfg_id: u16,
    ) -> Self {
        let response = GetStatus::new(
            session_status,
            resource_status,
            state_number,
            session_cfg_id,
        );
        Self { data: response.into() }
    }

    pub fn unlock(status: UnlockStatus) -> Self {
        let response = Unlock::new(status);
        Self { data: response.into() }
    }

    pub fn upload(elements: Vec<u8>, ag: AddressGranularity) -> Self {
        let response = Upload::new(elements);
        Self { data: response.into_with(ag) }
    }

    #[inline]
    pub fn origin_data<T>(&self) -> Result<T, XcpError>
    where
        T: for<'a> TryFrom<&'a [u8], Error = XcpError>
    {
        T::try_from(self.data.as_slice())
    }
}

impl Into<Vec<u8>> for Positive {
    #[inline]
    fn into(mut self) -> Vec<u8> {
        let mut result = vec![0xFF, ];
        result.append(&mut self.data);

        result
    }
}

impl From<&[u8]> for Positive {
    #[inline]
    fn from(data: &[u8]) -> Self {
        Self { data: data.to_vec() }
    }
}
