// mod config;
// pub use config::*;

use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use isotp_rs::{IsoTpEvent, IsoTpEventListener};
use isotp_rs::can::isotp::SyncCanIsoTp;
use isotp_rs::error::Error as IsoTpError;
use crate::{P2Context, utils};
use crate::service::Configuration;

#[derive(Debug, Default, Clone)]
pub struct IsoTpBuffer {
    inner: Arc<RwLock<Option<IsoTpEvent>>>,
}

impl IsoTpBuffer {
    fn clear(&self) {
        match self.inner.try_write() {
            Ok(mut buffer) => *buffer = None,
            Err(_) => {
                log::warn!("UDS - failed to acquire write lock for `IsoTpBuffer::clear`");
            },
        }
    }

    fn set(&self, event: IsoTpEvent) {
        match self.inner.try_write() {
            Ok(mut buffer) => *buffer = Some(event),
            Err(_) => {
                log::warn!("UDS - failed to acquire write lock for `IsoTpBuffer::set`");
            },
        }
    }

    fn get(&self) -> Option<IsoTpEvent> {
        match self.inner.try_read() {
            Ok(buffer) => buffer.clone(),
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
        let mut start = SystemTime::now();

        loop {
            tokio::time::sleep(Duration::from_millis(5)).await;

            match SystemTime::now().duration_since(start) {
                Ok(elapsed) => if elapsed > timeout {
                    self.buffer.clear();
                    return Err(IsoTpError::Timeout { value: tov, unit: "ms" })
                },
                Err(e) => {
                    self.buffer.clear();
                    return Err(IsoTpError::ContextError(e.to_string()));
                },
            }

            match self.buffer.get() {
                Some(event) => match event {
                    IsoTpEvent::Wait | IsoTpEvent::FirstFrameReceived => {
                        start = SystemTime::now();
                    },
                    IsoTpEvent::DataReceived(data) => {
                        self.buffer.clear();
                        return Ok(data);
                    },
                    IsoTpEvent::ErrorOccurred(e) => return Err(e.clone()),
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
        let mut start = SystemTime::now();

        loop {
            std::thread::sleep(Duration::from_millis(5));

            match SystemTime::now().duration_since(start) {
                Ok(elapsed) => if elapsed > timeout {
                    self.buffer.clear();
                    return Err(IsoTpError::Timeout { value: tov, unit: "ms" })
                },
                Err(e) => {
                    self.buffer.clear();
                    return Err(IsoTpError::ContextError(e.to_string()));
                },
            }

            match self.buffer.get() {
                Some(event) => match event {
                    IsoTpEvent::Wait | IsoTpEvent::FirstFrameReceived => {
                        start = SystemTime::now();
                    },
                    IsoTpEvent::DataReceived(data) => {
                        log::trace!("UDS - data received: {}", utils::hex_slice_to_string(data.as_slice()));
                        self.buffer.clear();
                        return Ok(data);
                    },
                    IsoTpEvent::ErrorOccurred(e) => return Err(e.clone()),
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
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

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



