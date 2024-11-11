use std::fmt::Display;
use std::hash::Hash;
use iso15765_2::{can::frame::Frame as CanFrame, device::SyncDevice};

pub struct SyncServer<D, Device, C, F>
where
    D: SyncDevice<Device = Device, Channel = C, Id = u32, Frame = F>,
    C: Clone + Eq,
    F: CanFrame<Channel = C>,
{
    device: D,
}

impl<D, Device, C, F> SyncServer<D, Device, C, F>
where
    D: SyncDevice<Device = Device, Channel = C, Id = u32, Frame = F>,
    C: Display + Clone + Hash + Eq + 'static,
    F: CanFrame<Channel = C> + Clone + 'static
{




}
