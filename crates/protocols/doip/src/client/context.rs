use derive_getters::Getters;
use lazy_static::lazy_static;
use iso13400_2::{Eid, FurtherAction, Gid, LogicAddress, PayloadType, SyncStatus, Version};

#[derive(Debug, Clone, Getters)]
pub struct GatewayInfo {
    #[getter(copy)]
    pub(crate) version: Version,
    #[getter(copy)]
    pub(crate) address: LogicAddress,
    #[getter(copy)]
    pub(crate) eid: Eid,
    #[getter(copy)]
    pub(crate) gid: Gid,
    #[getter(copy)]
    pub(crate) further_act: FurtherAction,
    #[getter(copy)]
    pub(crate) sync_status: Option<SyncStatus>,
}

#[derive(Debug)]
pub struct ExpectedPayloadType {
    pub(crate) vid_payload_types: Vec<PayloadType>,
    pub(crate) es_payload_types: Vec<PayloadType>,
    pub(crate) dpm_payload_types: Vec<PayloadType>,
    pub(crate) ra_payload_types: Vec<PayloadType>,
    pub(crate) ac_payload_types: Vec<PayloadType>,
    pub(crate) diag_payload_types: Vec<PayloadType>,
    pub(crate) diag_data_payload_types: Vec<PayloadType>,
}

impl Default for ExpectedPayloadType {
    fn default() -> Self {
        Self {
            vid_payload_types: vec![PayloadType::RespHeaderNegative, PayloadType::RespVehicleId, ],
            es_payload_types: vec![PayloadType::RespHeaderNegative, PayloadType::RespEntityStatus, ],
            dpm_payload_types: vec![PayloadType::RespHeaderNegative, PayloadType::RespDiagPowerMode, ],
            ra_payload_types: vec![PayloadType::RespHeaderNegative, PayloadType::RespRoutingActive, ],
            ac_payload_types: vec![PayloadType::RespHeaderNegative, PayloadType::RespAliveCheck, ],
            diag_payload_types: vec![PayloadType::RespHeaderNegative, PayloadType::RespDiagPositive, PayloadType::RespDiagNegative],
            diag_data_payload_types: vec![PayloadType::Diagnostic, ],
        }
    }
}

lazy_static!(
    pub(crate) static ref PL_TYPES: ExpectedPayloadType = Default::default();
);
