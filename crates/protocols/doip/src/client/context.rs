use derive_getters::Getters;
use iso13400_2::{Eid, FurtherAction, Gid, LogicAddress, SyncStatus, Version};

#[derive(Debug, Clone, Getters)]
pub struct Context {
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
