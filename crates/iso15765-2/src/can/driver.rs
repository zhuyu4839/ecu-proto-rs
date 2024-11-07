//! CAN device driver impl.

mod synchronous;
pub use synchronous::SyncCan;

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use crate::can::frame::Frame;
use crate::device::{Driver, Listener};

pub(crate) type ListenerType<C, F> = Box<dyn Listener<C, u32, F>>;

#[inline]
pub(crate) fn register_listener<C, F>(
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
    name: String,
    listener: ListenerType<C, F>,
) -> bool {
    match listeners.lock() {
        Ok(mut v) => {
            v.insert(name, listener);
            true
        },
        Err(e) => {
            log::warn!("SyncCAN - mutex error: {:?} when inserting listener", e);
            false
        },
    }
}

#[inline]
pub(crate) fn unregister_listener<C, F>(
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
    name: String,
) -> bool {
    match listeners.lock() {
        Ok(mut v) => {
            v.remove(&name).is_some()
        },
        Err(e) => {
            log::warn!("SyncCAN - mutex error: {:?} when removing listener", e);
            false
        },
    }
}

#[inline]
pub(crate) fn unregister_all<C, F>(
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
) -> bool {
    match listeners.lock() {
        Ok(mut v) => {
            v.clear();
            true
        },
        Err(e) => {
            log::warn!("SyncCAN - mutex error: {:?} when removing all listeners", e);
            false
        },
    }
}

#[inline]
pub(crate) fn listener_names<C, F>(
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
) -> Vec<String> {
    match listeners.lock() {
        Ok(v) => {
            v.keys()
                .into_iter()
                .map(|f| f.clone())
                .collect()
        },
        Err(e) => {
            log::warn!("SyncCAN - mutex error: {:?} when removing all listeners", e);
            vec![]
        },
    }
}

#[inline]
fn on_messages_util<C, F>(
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
    messages: &Vec<F>,
    channel: C
)
where
    F: 'static,
    C: Clone + 'static
{
    match listeners.lock() {
        Ok(mut v) => v.values_mut()
            .for_each(|o| {
                o.on_frame_received(channel.clone(), messages);
            }),
        Err(e) =>
            log::error!("SyncCAN - mutex error: {e:?} `on_messages`"),
    }
}

#[inline]
fn on_transmitting_util<C, F>(
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
    channel: C,
    frame: &F
)
where
    F: 'static,
    C: Clone + 'static
{
    match listeners.lock() {
        Ok(mut v) => v.values_mut()
            .for_each(|o| {
                o.on_frame_transmitting(channel.clone(), frame);
            }),
        Err(e) =>
            log::error!("SyncCAN - mutex error: {e:?} `on_transmit`"),
    }
}

#[inline]
fn on_transmitted_util<C, F>(
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
    id: u32,
    channel: C
)
where
    F: 'static,
    C: Clone + 'static
{
    match listeners.lock() {
        Ok(mut v) => v.values_mut()
            .for_each(|o| {
                o.on_frame_transmitted(channel.clone(), id);
            }),
        Err(e) =>
            log::error!("SyncCAN - mutex error: {e:?} `on_transmit`"),
    }
}

#[inline]
pub(crate) fn transmit_callback<D, C, F>(
    receiver: &Arc<Mutex<Receiver<F>>>,
    device: &D,
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
    timeout: Option<u32>,
)
where
    D: Driver<F = F>,
    C: Clone + Display + 'static,
    F: Frame<Channel = C> + Display + 'static,
{
    if let Ok(receiver) = receiver.lock() {
        if let Ok(msg) = receiver.try_recv() {
            log::debug!("SyncCAN - transmit: {}", msg);
            let id = msg.id();
            on_transmitting_util(listeners, msg.channel(), &msg);
            let channel = msg.channel();
            if let Ok(_) = device.transmit(msg, timeout) {
                on_transmitted_util(listeners, id.into_bits(), channel);
            }
        }
    }
}

#[inline]
pub(crate) fn receive_callback<D, C, F>(
    device: &D,
    listeners: &Arc<Mutex<HashMap<String, ListenerType<C, F>>>>,
    timeout: Option<u32>,
)
where
    F: 'static,
    D: Driver<C = C, F = F>,
    C: Clone + 'static,
{
    let channels = device.opened_channels();
    channels.into_iter()
        .for_each(|c| {
            if let Ok(messages) = device.receive(c.clone(), timeout) {
                if !messages.is_empty() {
                    on_messages_util(listeners, &messages, c.clone());
                }
            }
        });
}
