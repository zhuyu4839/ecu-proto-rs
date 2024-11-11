use std::any::Any;
use std::fmt::Display;
use crate::{IsoTpEvent, IsoTpFrame, IsoTpState, can::CanIsoTpFrame};
use crate::can::{isotp::SyncCanIsoTp, frame::Frame};
use crate::device::Listener;

impl<C, F> Listener<C, u32, F> for SyncCanIsoTp<C, F>
where
    C: Clone + Eq + Display + 'static,
    F: Frame<Channel = C> + Clone + Display + 'static {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn on_frame_transmitting(&mut self, _: C, _: &F) {

    }

    fn on_frame_transmitted(&mut self, channel: C, id: u32) {
        log::trace!("ISO-TP(CAN sync) transmitted: {:04X} from {}", id, channel);
        if channel != self.channel {
            return;
        }

        if let Ok(address) = self.address.lock() {
            if id == address.tx_id ||
                id == address.fid {
                self.state_remove(IsoTpState::Sending);
            }
        }
    }

    fn on_frame_received(&mut self, channel: C, frames: &[F]) {
        if channel != self.channel
            || self.state_contains(IsoTpState::Error) {
            return;
        }

        let address_id = if let Ok(address) = self.address.lock() {
            Some((address.tx_id, address.rx_id))
        }
        else {
            None
        };

        if let Some(address) = address_id {
            for frame in frames {
                if frame.id().into_bits() == address.1 {
                    log::debug!("ISO-TP(CAN sync) received: {}", frame);

                    match CanIsoTpFrame::decode(frame.data()) {
                        Ok(frame) => match frame {
                            CanIsoTpFrame::SingleFrame { data } => {
                                self.on_single_frame(data);
                            }
                            CanIsoTpFrame::FirstFrame { length, data } => {
                                self.on_first_frame(address.0, length, data);
                            }
                            CanIsoTpFrame::ConsecutiveFrame { sequence, data } => {
                                self.on_consecutive_frame(sequence, data);
                            },
                            CanIsoTpFrame::FlowControlFrame(ctx) => {
                                self.on_flow_ctrl_frame(ctx);
                            },
                        },
                        Err(e) => {
                            log::warn!("ISO-TP(CAN sync) - data convert to frame failed: {}", e);
                            self.state_append(IsoTpState::Error);
                            self.iso_tp_event(IsoTpEvent::ErrorOccurred(e));

                            break;
                        }
                    }
                }
            }
        }
    }
}