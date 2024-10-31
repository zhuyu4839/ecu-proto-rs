// mod config;
// pub use config::*;

use std::{sync::{Arc, Mutex}, time::{Duration, Instant}};
use std::collections::VecDeque;
use isotp_rs::{can::isotp::SyncCanIsoTp, error::Error as IsoTpError, IsoTpEvent, IsoTpEventListener};
use iso14229_1::Configuration;
use crate::P2Context;

#[derive(Debug, Default, Clone)]
pub struct IsoTpBuffer {
    inner: Arc<Mutex<VecDeque<IsoTpEvent>>>,
}

impl IsoTpBuffer {
    #[inline]
    fn clear(&self) {
        match self.inner.lock() {
            Ok(mut buffer) => buffer.clear(),
            Err(_) => {
                log::warn!("UDS - failed to acquire write lock for `IsoTpBuffer::clear`");
            },
        }
    }

    #[inline]
    fn set(&self, event: IsoTpEvent) {
        match self.inner.lock() {
            Ok(mut buffer) => buffer.push_back(event),
            Err(_) => {
                log::warn!("UDS - failed to acquire write lock for `IsoTpBuffer::set`");
            },
        }
    }

    #[inline]
    fn get(&self) -> Option<IsoTpEvent> {
        match self.inner.lock() {
            Ok(mut buffer) => buffer.pop_front(),
            Err(_) => {
                log::warn!("UDS - failed to acquire write lock for `IsoTpBuffer::get`");
                None
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct IsoTpListener {
    pub(crate) buffer: IsoTpBuffer,
    pub(crate) p2_ctx: P2Context,
}

impl IsoTpListener {
    pub fn new(p2_ctx: P2Context) -> Self {
        Self {
            buffer: Default::default(),
            p2_ctx,
        }
    }
}

impl IsoTpListener {
    #[cfg(feature = "tokio")]
    pub async fn async_timer(&mut self, response_pending: bool) -> Result<Vec<u8>, IsoTpError> {
        let tov = if response_pending {
            self.p2_ctx.p2_star as u64
        }
        else {
            (self.p2_ctx.p2 + self.p2_ctx.p2_offset) as u64
        };

        let timeout = Duration::from_millis(tov);
        let mut start = Instant::now();

        loop {
            tokio::time::sleep(Duration::from_millis(1)).await;

            if start.elapsed() > timeout {
                self.clear_buffer();
                return Err(IsoTpError::Timeout { value: tov, unit: "ms" })
            }

            match self.from_buffer() {
                Some(event) => match event {
                    IsoTpEvent::Wait | IsoTpEvent::FirstFrameReceived => {
                        start = Instant::now();
                    },
                    IsoTpEvent::DataReceived(data) => {
                        log::trace!("UDS - data received: {}", hex::encode(&data));
                        return Ok(data);
                    },
                    IsoTpEvent::ErrorOccurred(e) => {
                        self.clear_buffer();
                        return Err(e.clone());
                    },
                },
                None => {
                    continue
                },
            }
        }
    }

    pub fn sync_timer(&mut self, response_pending: bool) -> Result<Vec<u8>, IsoTpError> {
        let tov = if response_pending {
            self.p2_ctx.p2_star as u64
        }
        else {
            (self.p2_ctx.p2 + self.p2_ctx.p2_offset) as u64
        };

        let timeout = Duration::from_millis(tov);
        let mut start = Instant::now();

        loop {
            std::thread::sleep(Duration::from_millis(5));

            if start.elapsed() > timeout {
                self.clear_buffer();
                return Err(IsoTpError::Timeout { value: tov, unit: "ms" });
            }

            match self.from_buffer() {
                Some(event) => match event {
                    IsoTpEvent::Wait | IsoTpEvent::FirstFrameReceived => {
                        start = Instant::now();
                    },
                    IsoTpEvent::DataReceived(data) => {
                        log::trace!("UDS - data received: {}", hex::encode(&data));
                        return Ok(data);
                    },
                    IsoTpEvent::ErrorOccurred(e) => {
                        self.clear_buffer();
                        return Err(e.clone());
                    },
                },
                None => {
                    continue
                },
            }
        }
    }

    #[inline]
    pub fn update_p2_ctx(&mut self, p2: u16, p2_star: u32) {
        self.p2_ctx.p2 = p2;
        self.p2_ctx.p2_star = p2_star;
    }
}

impl IsoTpEventListener for IsoTpListener {
    #[inline]
    fn from_buffer(&mut self) -> Option<IsoTpEvent> {
        self.buffer.get()
    }
    #[inline]
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
    #[inline]
    fn on_iso_tp_event(&mut self, event: IsoTpEvent) {
        self.buffer.set(event)
    }
}

#[derive(Clone)]
pub struct Context<C: Clone + Eq, F> {
    pub(crate) iso_tp: SyncCanIsoTp<C, F>,
    pub(crate) listener: IsoTpListener,
    pub(crate) config: Configuration,
    // pub(crate) config: Configuration,
}



