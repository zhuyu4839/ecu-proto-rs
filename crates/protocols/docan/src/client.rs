mod context;

#[cfg(not(feature = "async"))]
mod synchronous;
#[cfg(not(feature = "async"))]
pub use synchronous::*;

use iso14229_1::{request, response, *};
use rs_can::isotp::{Address, AddressType};
use rs_can::ResultWrapper;
use crate::{SecurityAlgo};

pub trait Client {
    type Channel;
    // type Device;
    type Error;

    fn update_address(&mut self,
                      channel: Self::Channel,
                      address: Address
    ) -> ResultWrapper<(), Self::Error>;
    fn update_security_algo(&mut self,
                            channel: Self::Channel,
                            algo: SecurityAlgo
    ) -> ResultWrapper<(), Self::Error>;
    fn add_data_identifier(&mut self,
                           channel: Self::Channel,
                           did: DataIdentifier,
                           length: usize
    ) -> ResultWrapper<(), Self::Error>;
    fn remove_data_identifier(&mut self,
                              channel: Self::Channel,
                              did: DataIdentifier
    ) -> ResultWrapper<(), Self::Error>;
    fn set_address_of_byte_order(&mut self,
                                 channel: Self::Channel,
                                 bo: ByteOrder
    ) -> ResultWrapper<(), Self::Error>;
    fn set_memory_size_of_byte_order(&mut self,
                                     channel: Self::Channel,
                                     bo: ByteOrder
    ) -> ResultWrapper<(), Self::Error>;
    /** - Diagnostic and communication management functional unit - **/
    fn session_ctrl(&mut self,
                    channel: Self::Channel,
                    r#type: SessionType,
                    suppress_positive: bool,
                    addr_type: AddressType,
    ) -> ResultWrapper<(), Self::Error>;
    fn ecu_reset(&mut self,
                 channel: Self::Channel,
                 r#type: ECUResetType,
                 suppress_positive: bool,
                 addr_type: AddressType,
    ) -> ResultWrapper<(), Self::Error>;
    fn security_access(&mut self,
                       channel: Self::Channel,
                       level: u8,
                       params: Vec<u8>,
    ) -> ResultWrapper<Vec<u8>, Self::Error>;
    fn unlock_security_access(&mut self,
                              channel: Self::Channel,
                              level: u8,
                              params: Vec<u8>,
                              salt: Vec<u8>,
    ) -> ResultWrapper<(), Self::Error>;
    fn communication_control(&mut self,
                             channel: Self::Channel,
                             ctrl_type:CommunicationCtrlType,
                             comm_type: CommunicationType,
                             node_id: Option<request::NodeId>,
                             suppress_positive: bool,
                             addr_type: AddressType,
    ) -> ResultWrapper<(), Self::Error>;
    #[cfg(feature = "std2020")]
    fn authentication(&mut self,
                      channel: Self::Channel,
                      auth_task: AuthenticationTask,
                      data: request::Authentication,
    ) -> ResultWrapper<response::Authentication, Self::Error>;
    fn tester_present(&mut self,
                      channel: Self::Channel,
                      r#type: TesterPresentType,
                      suppress_positive: bool,
                      addr_type: AddressType,
    ) -> ResultWrapper<(), Self::Error>;
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    fn access_timing_parameter(&mut self,
                               channel: Self::Channel,
                               r#type: TimingParameterAccessType,
                               parameter: Vec<u8>,
                               suppress_positive: bool,
    ) -> ResultWrapper<Option<response::AccessTimingParameter>, Self::Error>;
    fn secured_data_transmit(&mut self,
                             channel: Self::Channel,
                             apar: AdministrativeParameter,
                             signature: SignatureEncryptionCalculation,
                             anti_replay_cnt: u16,
                             service: u8,
                             service_data: Vec<u8>,
                             signature_data: Vec<u8>,
    ) -> ResultWrapper<response::SecuredDataTrans, Self::Error>;
    fn control_dtc_setting(&mut self,
                           channel: Self::Channel,
                           r#type: DTCSettingType,
                           parameter: Vec<u8>,
                           suppress_positive: bool,
    ) -> ResultWrapper<(), Self::Error>;
    fn response_on_event(&mut self,
                         channel: Self::Channel,
    ) -> ResultWrapper<(), Self::Error>;
    fn link_control(&mut self,
                    channel: Self::Channel,
                    r#type: LinkCtrlType,
                    data: request::LinkCtrl,
                    suppress_positive: bool,
    ) -> ResultWrapper<(), Self::Error>;
    fn read_data_by_identifier(&mut self,
                               channel: Self::Channel,
                               did: DataIdentifier,
                               others: Vec<DataIdentifier>,
    ) -> ResultWrapper<response::ReadDID, Self::Error>;
    fn read_memory_by_address(&mut self,
                              channel: Self::Channel,
                              mem_loc: MemoryLocation,
    ) -> ResultWrapper<Vec<u8>, Self::Error>;
    fn read_scaling_data_by_identifier(&mut self,
                                       channel: Self::Channel,
                                       did: DataIdentifier,
    ) -> ResultWrapper<response::ReadScalingDID, Self::Error>;
    /** - Data transmission functional unit - **/
    fn read_data_by_period_identifier(&mut self,
                                      channel: Self::Channel,
                                      mode: request::TransmissionMode,
                                      did: Vec<u8>,
    ) -> ResultWrapper<response::ReadDataByPeriodId, Self::Error>;
    fn dynamically_define_data_by_identifier(&mut self,
                                             channel: Self::Channel,
                                             r#type: DefinitionType,
                                             data: request::DynamicallyDefineDID,
                                             suppress_positive: bool,
    ) -> ResultWrapper<Option<response::DynamicallyDefineDID>, Self::Error>;
    fn write_data_by_identifier(&mut self,
                                channel: Self::Channel,
                                did: DataIdentifier,
                                data: Vec<u8>,
    ) -> ResultWrapper<(), Self::Error>;
    fn write_memory_by_address(&mut self,
                               channel: Self::Channel,
                               alfi: AddressAndLengthFormatIdentifier,
                               mem_addr: u128,
                               mem_size: u128,
                               record: Vec<u8>,
    ) -> ResultWrapper<response::WriteMemByAddr, Self::Error>;
    /** Stored data transmission functional unit - **/
    fn clear_dtc_info(&mut self,
                      channel: Self::Channel,
                      group: utils::U24,
                      #[cfg(any(feature = "std2020"))]
                      mem_sel: Option<u8>,
                      addr_type: AddressType,
    ) -> ResultWrapper<(), Self::Error>;
    fn read_dtc_info(&mut self,
                     channel: Self::Channel,
                     r#type: DTCReportType,
                     data: request::DTCInfo,
    ) -> ResultWrapper<response::DTCInfo, Self::Error>;
    /** - InputOutput control functional unit - **/
    fn io_control(&mut self,
                  channel: Self::Channel,
                  did: DataIdentifier,
                  param: IOCtrlParameter,
                  state: Vec<u8>,
                  mask: Vec<u8>,
    ) -> ResultWrapper<response::IOCtrl, Self::Error>;
    /** - Remote activation of routine functional unit - **/
    fn routine_control(&mut self,
                       channel: Self::Channel,
                       r#type: RoutineCtrlType,
                       routine_id: u16,
                       option_record: Vec<u8>,
    ) -> ResultWrapper<response::RoutineCtrl, Self::Error>;
    /** - Upload download functional unit - **/
    fn request_download(&mut self,
                        channel: Self::Channel,
                        alfi: AddressAndLengthFormatIdentifier,
                        mem_addr: u128,
                        mem_size: u128,
                        dfi: Option<DataFormatIdentifier>,
    ) -> ResultWrapper<response::RequestDownload, Self::Error>;
    fn request_upload(&mut self,
                      channel: Self::Channel,
                      alfi: AddressAndLengthFormatIdentifier,
                      mem_addr: u128,
                      mem_size: u128,
                      dfi: Option<DataFormatIdentifier>,
    ) -> ResultWrapper<response::RequestUpload, Self::Error>;
    fn transfer_data(&mut self,
                     channel: Self::Channel,
                     sequence: u8,
                     data: Vec<u8>,
    ) -> ResultWrapper<response::TransferData, Self::Error>;
    fn request_transfer_exit(&mut self,
                             channel: Self::Channel,
                             parameter: Vec<u8>,
    ) -> ResultWrapper<Vec<u8>, Self::Error>;
    fn request_file_transfer(&mut self,
                             channel: Self::Channel,
                             operation: ModeOfOperation,
                             data: request::RequestFileTransfer,
    ) -> ResultWrapper<response::RequestFileTransfer, Self::Error>;
}
