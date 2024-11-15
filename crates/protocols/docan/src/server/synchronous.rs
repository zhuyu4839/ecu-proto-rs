use std::{fmt::Display, hash::Hash};
use rs_can::{CanDriver, Frame, isotp::{Address, IsoTpAdapter}, ResultWrapper};
use rs_can::isotp::CanIsoTp;
use crate::{DoCanError, Server};
use super::context::{Context, IsoTpListener};

#[derive(Clone)]
pub struct DoCanServer<D, C, F> {
    adapter: IsoTpAdapter<D, C, F>,
    context: Context<C, F>,
}

impl<D, C, F> DoCanServer<D, C, F>
where
    D: CanDriver<Channel = C, Frame = F> + Clone + Send + 'static,
    C: Display + Clone + Hash + Eq + 'static,
    F: Frame<Channel = C> + Clone + Send + Display + 'static
{
    pub fn new(adapter: IsoTpAdapter<D, C, F>, channel: C, address: Address) -> Self {
        let listener = IsoTpListener {
            buffer: Default::default(),
        };
        let iso_tp = CanIsoTp::new(
            channel.clone(),
            address,
            adapter.sender(),
            Box::new(listener.clone()),
        );
        adapter.register_listener(
            format!("DoCANServer-{}", channel),
            Box::new(iso_tp.clone()),
        );
        Self {
            adapter,
            context: Context::new(iso_tp, listener),
        }
    }

    #[inline]
    pub fn adapter(&self) -> &IsoTpAdapter<D, C, F> {
        &self.adapter
    }
}

impl<D, C, F> Server for DoCanServer<D, C, F>
where
    C: Display + Clone,
    F: Frame<Channel = C>
{
    type Channel = C;
    type Device = D;
    type Error = DoCanError;

    fn service_forever(&mut self, interval: u64) -> ResultWrapper<(), Self::Error> {
        self.context.server(interval)
    }

    fn service_stop(&mut self) -> ResultWrapper<(), Self::Error> {
        self.context.stop()
    }
}
