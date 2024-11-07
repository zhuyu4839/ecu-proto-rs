mod listener;

use std::sync::{Arc, mpsc::Sender, Mutex};
use tokio::time::sleep;
use std::time::{Duration, Instant};
use crate::{FlowControlContext, FlowControlState, IsoTpEvent, IsoTpEventListener, IsoTpFrame, IsoTpState, can::{Address, CanIsoTpFrame, isotp::context::IsoTpContext, frame::Frame}};
use crate::{P2_ISO14229, P2_STAR_ISO14229, TIMEOUT_AS_ISO15765_2, TIMEOUT_BR_ISO15765_2, TIMEOUT_CR_ISO15765_2};
use crate::Error;

#[derive(Clone)]
pub struct AsyncCanIsoTp<C, F> {
    pub(crate) channel: C,
    pub(crate) address: Arc<Mutex<Address>>,
    pub(crate) sender: Sender<F>,
    pub(crate) context: Arc<Mutex<IsoTpContext>>,
    pub(crate) state: Arc<Mutex<IsoTpState>>,
    pub(crate) listener: Arc<Mutex<Box<dyn IsoTpEventListener>>>,
}

unsafe impl<C, F> Send for AsyncCanIsoTp<C, F> {}

impl<C: Clone, F: Frame<Channel = C>> AsyncCanIsoTp<C, F> {

    pub fn new(channel: C,
               address: Address,
               sender: Sender<F>,
               listener: Box<dyn IsoTpEventListener>
    ) -> Self {
        Self {
            channel,
            address: Arc::new(Mutex::new(address)),
            sender,
            context: Default::default(),
            state: Default::default(),
            listener: Arc::new(Mutex::new(listener)),
        }
    }

    #[inline]
    pub fn update_address(&self, address: Address) {
        if let Ok(mut addr) = self.address.lock() {
            *addr = address;
        }
    }

    pub async fn write(&self, functional: bool, data: Vec<u8>) -> Result<(), Error> {
        self.state_append(IsoTpState::Idle);
        self.context_reset();
        log::debug!("ISO-TP(CAN async) - Sending: {}", hex::encode(&data));

        let frames = CanIsoTpFrame::from_data(data)?;
        let frame_len = frames.len();

        let can_id = match self.address.lock() {
            Ok(address) => if functional { Ok(address.fid) } else { Ok(address.tx_id) },
            Err(_) => Err(Error::ContextError("can't get address context".into())),
        }?;
        let mut need_flow_ctrl = frame_len > 1;
        let mut index = 0;
        for frame in frames {
            let mut frame = F::from_iso_tp(can_id, frame, None)
                .ok_or(Error::ConvertError {
                    src: "iso-tp frame",
                    target: "can-frame",
                })?;
            frame.set_channel(self.channel.clone());

            if need_flow_ctrl {
                need_flow_ctrl = false;
                self.state_append(IsoTpState::Sending | IsoTpState::WaitFlowCtrl);
            }
            else {
                self.write_waiting(&mut index)?;
                self.state_append(IsoTpState::Sending);
            }
            self.sender.send(frame)
                .map_err(|e| {
                    log::warn!("ISO-TP(CAN async) - transmit failed: {:?}", e);
                    Error::DeviceError
                })?;
        }

        Ok(())
    }

    #[inline]
    pub(crate) fn on_single_frame(&self, data: Vec<u8>) {
        self.iso_tp_event(IsoTpEvent::DataReceived(data));
    }

    #[inline]
    pub(crate) fn on_first_frame(&self, tx_id: u32, length: u32, data: Vec<u8>) {
        self.update_consecutive(length, data);

        let iso_tp_frame = CanIsoTpFrame::default_flow_ctrl_frame();
        match F::from_iso_tp(tx_id, iso_tp_frame, None) {
            Some(mut frame) => {
                frame.set_channel(self.channel.clone());

                self.state_append(IsoTpState::Sending);
                match self.sender.send(frame) {
                    Ok(_) => {
                        self.iso_tp_event(IsoTpEvent::FirstFrameReceived);
                    },
                    Err(e) => {
                        log::warn!("ISO-TP(CAN async) - transmit failed: {:?}", e);
                        self.state_append(IsoTpState::Error);

                        self.iso_tp_event(IsoTpEvent::ErrorOccurred(Error::DeviceError));
                    },
                }
            },
            None => log::error!("ISO-TP(CAN async): convert `iso-tp frame` to `can-frame` error"),
        }
    }

    #[inline]
    pub(crate) fn on_consecutive_frame(&self, sequence: u8, data: Vec<u8>) {
        match self.append_consecutive(sequence, data) {
            Ok(event) => self.iso_tp_event(event),
            Err(e) => {
                self.state_append(IsoTpState::Error);
                self.iso_tp_event(IsoTpEvent::ErrorOccurred(e));
            }
        }
    }

    #[inline]
    pub(crate) fn on_flow_ctrl_frame(&self, ctx: FlowControlContext) {
        match ctx.state() {
            FlowControlState::Continues => {
                self.state_remove(IsoTpState::WaitBusy | IsoTpState::WaitFlowCtrl);
            },
            FlowControlState::Wait => {
                self.state_append(IsoTpState::WaitBusy);
                self.iso_tp_event(IsoTpEvent::Wait);
                return;
            }
            FlowControlState::Overload => {
                self.state_append(IsoTpState::Error);
                self.iso_tp_event(IsoTpEvent::ErrorOccurred(Error::OverloadFlow));
                return;
            }
        }

        if let Ok(mut context) = self.context.lock() {
            context.update_flow_ctrl(ctx);
        };
    }

    fn iso_tp_event(&self, event: IsoTpEvent) {
        match self.listener.lock() {
            Ok(mut listener) => {
                // println!("ISO-TP(CAN async): Sending iso-tp event: {:?}", event);
                match &event {
                    IsoTpEvent::DataReceived(data) => {
                        log::debug!("ISO-TP - Received: {}", hex::encode(data));
                    },
                    IsoTpEvent::ErrorOccurred(_) =>
                        log::warn!("ISO-TP(CAN sync): Sending iso-tp event: {:?}", event),
                    _ => log::trace!("ISO-TP(CAN sync): Sending iso-tp event: {:?}", event),
                }
                listener.on_iso_tp_event(event);
            },
            Err(_) => log::warn!("ISO-TP(CAN async): Sending event failed"),
        }
    }

    async fn write_waiting(&self, index: &mut usize) -> Result<(), Error> {
        match self.context.lock() {
            Ok(ctx) => {
                if let Some(ctx) = &ctx.flow_ctrl {
                    if ctx.block_size != 0 {
                        if (*index + 1) == ctx.block_size as usize {
                            *index = 0;
                            self.state_append(IsoTpState::WaitFlowCtrl);
                        }
                        else {
                            *index += 1;
                        }
                    }
                    sleep(Duration::from_micros(ctx.st_min as u64)).await;
                }

                Ok(())
            },
            Err(_) => Err(Error::ContextError("can't get `context`".into()))
        }?;

        let start = Instant::now();
        loop {
            if self.state_contains(IsoTpState::Error) {
                return Err(Error::DeviceError);
            }

            if self.state_contains(IsoTpState::Sending) {
                if start.elapsed() > Duration::from_millis(TIMEOUT_AS_ISO15765_2 as u64) {
                    return Err(Error::Timeout { value: TIMEOUT_AS_ISO15765_2 as u64, unit: "ms" });
                }
            }
            else if self.state_contains(IsoTpState::WaitBusy) {
                if start.elapsed() > Duration::from_millis(P2_STAR_ISO14229 as u64) {
                    return Err(Error::Timeout { value: P2_STAR_ISO14229 as u64, unit: "ms" });
                }
            }
            else if self.state_contains(IsoTpState::WaitFlowCtrl) {
                if start.elapsed() > Duration::from_millis(TIMEOUT_CR_ISO15765_2 as u64) {
                    return Err(Error::Timeout { value: TIMEOUT_CR_ISO15765_2 as u64, unit: "ms" });
                }
            }
            else {
                break;
            }
        }

        Ok(())
    }

    fn append_consecutive(&self, sequence: u8, data: Vec<u8>) -> Result<IsoTpEvent, Error> {
        match self.context.lock() {
            Ok(mut context) => {
                context.append_consecutive(sequence, data)
            },
            Err(_) => Err(Error::ContextError("can't get `context`".into()))
        }
    }

    fn update_consecutive(&self, length: u32, data: Vec<u8>) {
        if let Ok(mut context) = self.context.lock() {
            context.update_consecutive(length, data);
        }
    }

    fn context_reset(&self) {
        if let Ok(mut context) = self.context.lock() {
            context.reset();
        };
    }

    #[inline]
    fn state_contains(&self, flags: IsoTpState) -> bool {
        match self.state.lock() {
            Ok(v) => {
                // log::debug!("ISO-TP(CAN sync): current state(state contains): {} contains: {}", *v, flags);
                *v & flags != IsoTpState::Idle
            },
            Err(_) => {
                log::warn!("ISO-TP(CAN async): state mutex is poisoned");
                false
            },
        }
    }

    #[inline]
    fn state_append(&self, flags: IsoTpState) {
        match self.state.lock() {
            Ok(mut v) => {
                if flags == IsoTpState::Idle {
                    *v = IsoTpState::Idle;
                } else if flags.contains(IsoTpState::Error) {
                    *v = IsoTpState::Error;
                }
                else {
                    *v |= flags;
                }

                log::debug!("ISO-TP(CAN sync): current state(state append): {}", *v);
            }
            Err(_) => log::warn!("ISO-TP(CAN async): state mutex is poisoned"),
        }
    }

    #[inline]
    fn state_remove(&self, flags: IsoTpState) {
        match self.state.lock() {
            Ok(mut v) => {
                v.remove(flags);
                log::debug!("ISO-TP(CAN sync): current state(state remove): {}", *v);
            },
            Err(_) => log::warn!("ISO-TP(CAN async): state mutex is poisoned"),
        }
    }
}
