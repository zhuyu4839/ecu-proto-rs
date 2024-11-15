use std::time::{Duration, Instant};
use iso14229_1::Configuration;
use iso15765_2::{Iso15765Error, IsoTpEvent, IsoTpEventListener};
use rs_can::isotp::{CanIsoTp, P2Context};
use crate::{SecurityAlgo, buffer::IsoTpBuffer};

#[derive(Debug, Default, Clone)]
pub struct IsoTpListener {
    pub(crate) buffer: IsoTpBuffer,
    pub(crate) p2_ctx: P2Context,
    pub(crate) p2_offset: u64,
}

impl IsoTpListener {
    pub fn new(p2_ctx: P2Context, p2_offset: u64) -> Self {
        Self {
            buffer: Default::default(),
            p2_ctx,
            p2_offset,
        }
    }
}

impl IsoTpListener {
    #[cfg(feature = "async")]
    pub async fn async_timer(&mut self, response_pending: bool) -> Result<Vec<u8>, Iso15765Error> {
        let tov = if response_pending {
            self.p2_ctx.p2_star_ms()
        }
        else {
            self.p2_ctx.p2_ms() + self.p2_offset
        };

        let timeout = Duration::from_millis(tov);
        let mut start = Instant::now();

        loop {
            tokio::time::sleep(Duration::from_millis(1)).await;

            if start.elapsed() > timeout {
                self.clear_buffer();
                return Err(Iso15765Error::Timeout { value: tov, unit: "ms" })
            }

            match self.from_buffer() {
                Some(event) => match event {
                    IsoTpEvent::Wait | IsoTpEvent::FirstFrameReceived => {
                        start = Instant::now();
                    },
                    IsoTpEvent::DataReceived(data) => {
                        log::trace!("DoCAN - data received: {}", hex::encode(&data));
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

    pub fn sync_timer(&mut self, response_pending: bool) -> Result<Vec<u8>, Iso15765Error> {
        let tov = if response_pending {
            self.p2_ctx.p2_star_ms()
        }
        else {
            self.p2_ctx.p2_ms() + self.p2_offset
        };

        let timeout = Duration::from_millis(tov);
        let mut start = Instant::now();

        loop {
            std::thread::sleep(Duration::from_millis(5));

            if start.elapsed() > timeout {
                self.clear_buffer();
                return Err(Iso15765Error::Timeout { value: tov, unit: "ms" });
            }

            match self.buffer_data() {
                Some(event) => match event {
                    IsoTpEvent::Wait | IsoTpEvent::FirstFrameReceived => {
                        start = Instant::now();
                    },
                    IsoTpEvent::DataReceived(data) => {
                        if data.is_empty() {
                            continue;
                        }
                        log::trace!("DoCANClient - data received: {}", hex::encode(&data));
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
    pub fn update_p2_ctx(&mut self, p2: u16, p2_star: u16) {
        self.p2_ctx.update(p2, p2_star)
    }
}

impl IsoTpEventListener for IsoTpListener {
    #[inline]
    fn buffer_data(&mut self) -> Option<IsoTpEvent> {
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
    pub(crate) iso_tp: CanIsoTp<C, F>,
    pub(crate) listener: IsoTpListener,
    pub(crate) config: Configuration,
    pub(crate) security_algo: Option<SecurityAlgo>,
}
