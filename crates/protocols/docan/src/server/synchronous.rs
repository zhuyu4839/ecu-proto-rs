use std::fmt::Display;
use std::hash::Hash;
use rs_can::{CanDriver, Frame, Listener};
use rs_can::isotp::{Address, IsoTpAdapter};

pub struct SyncServer<D, C, F> {
    adapter: IsoTpAdapter<D, C, F>,
}

impl<D, C, F> SyncServer<D, C, F>
where
    D: CanDriver<Channel = C, Frame = F> + Clone + Send + 'static,
    C: Display + Clone + Hash + Eq + 'static,
    F: Frame<Channel = C> + Clone + 'static
{




}
