use std::collections::HashMap;
use std::fmt::Display;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use crate::can::driver::{listener_names, receive_callback, register_listener, transmit_callback, unregister_all, unregister_listener};
use crate::can::frame::Frame;
use crate::device::{Driver, Listener};

#[derive(Clone)]
pub struct SyncCan<D, C, F> {
    device: D,
    sender: Sender<F>,
    receiver: Arc<Mutex<Receiver<F>>>,
    listeners: Arc<Mutex<HashMap<String, Box<dyn Listener<C, u32, F>>>>>,
    stop_tx: Sender<()>,
    stop_rx: Arc<Mutex<Receiver<()>>>,
    send_task: Weak<JoinHandle<()>>,
    receive_task: Weak<JoinHandle<()>>,
    interval: Option<u64>,
}

impl<D, C, F> SyncCan<D, C, F>
where
    D: Driver<C = C, F = F> + Clone + 'static,
    C: Clone + Display + 'static,
    F: Frame<Channel = C> + Clone + Send + Display + 'static,
{
    pub fn new(device: D) -> Self {
        let (tx, rx) = channel();
        let (stop_tx, stop_rx) = channel();
        Self {
            device,
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
            listeners: Arc::new(Mutex::new(HashMap::new())),
            stop_tx,
            stop_rx: Arc::new(Mutex::new(stop_rx)),
            send_task: Default::default(),
            receive_task: Default::default(),
            interval: Default::default(),
        }
    }

    #[inline]
    pub fn sender(&self) -> Sender<F> {
        self.sender.clone()
    }

    #[inline]
    pub fn register_listener(
        &self,
        name: String,
        listener: Box<dyn Listener<C, u32, F>>,
    ) -> bool {
        log::debug!("ISO-TP(CAN sync) - register listener {}", name);
        register_listener(&self.listeners, name, listener)
    }

    #[inline]
    pub fn unregister_listener(&self, name: String) -> bool {
        unregister_listener(&self.listeners, name)
    }

    #[inline]
    pub fn unregister_all(&self) -> bool {
        unregister_all(&self.listeners)
    }

    #[inline]
    pub fn listener_names(&self) -> Vec<String> {
        listener_names(&self.listeners)
    }

    pub fn listener_callback(&self, name: &str, callback: impl FnOnce(&Box<dyn Listener<C, u32, F>>)) {
        if let Ok(listeners) = self.listeners.lock() {
            if let Some(listener) = listeners.get(name) {
                callback(listener);
            }
        }
    }

    pub fn sync_transmit(device: MutexGuard<Self>, interval_us: u64, stopper: Arc<Mutex<Receiver<()>>>) {
        sync_util(device, interval_us, stopper, |device| {
            transmit_callback(&device.receiver, &device.device, &device.listeners, None);
        });
    }

    pub fn sync_receive(device: MutexGuard<Self>, interval_us: u64, stopper: Arc<Mutex<Receiver<()>>>) {
        sync_util(device, interval_us, stopper, |device| {
            receive_callback(&device.device, &device.listeners, None);
        });
    }

    pub fn sync_start(&mut self, interval_us: u64) {
        self.interval = Some(interval_us);

        let self_arc = Arc::new(Mutex::new(self.clone()));
        let stop_rx = Arc::clone(&self.stop_rx);
        let tx_task = spawn(move || {
            if let Ok(self_clone) = self_arc.lock() {
                Self::sync_transmit(self_clone, interval_us, Arc::clone(&stop_rx));
            }
        });

        let self_arc = Arc::new(Mutex::new(self.clone()));
        let stop_rx = Arc::clone(&self.stop_rx);
        let rx_task = spawn(move || {
            if let Ok(self_clone) = self_arc.lock() {
                Self::sync_receive(self_clone, interval_us, Arc::clone(&stop_rx));
            }
        });

        self.send_task = Arc::downgrade(&Arc::new(tx_task));
        self.receive_task = Arc::downgrade(&Arc::new(rx_task));
    }

    pub fn stop(&mut self) {
        log::info!("SyncCAN - closing(sync)");

        if let Err(e) = self.stop_tx.send(()) {
            log::warn!("SyncCAN - error: {} when sending stop signal", e);
        }

        sleep(Duration::from_micros(2 * self.interval.unwrap_or(50 * 1000)));

        if let Some(task) = self.send_task.upgrade() {
            if !task.is_finished() {
                log::warn!("SyncCAN - send task is running after stop signal");
            }
        }

        if let Some(task) = self.receive_task.upgrade() {
            if !task.is_finished() {
                log::warn!("SyncCAN - receive task is running after stop signal");
            }
        }

        self.device.shutdown();
    }
}

#[inline]
fn sync_util<D, C, F>(
    device: MutexGuard<SyncCan<D, C, F>>,
    interval: u64,
    stopper: Arc<Mutex<Receiver<()>>>,
    callback: fn(&MutexGuard<SyncCan<D, C, F>>)
)
where D: Driver<C = C, F = F> + Clone + 'static,
      C: Clone + Display + 'static,
      F: Frame<Channel = C> + Clone + Send + Display + 'static,
{
    loop {
        if !device.device.is_closed() {
            callback(&device);
        }
        else {
            log::info!("SyncCAN - exit sync receive.");
            break;
        }

        if let Ok(stopper) = stopper.lock() {
            if let Ok(()) = stopper.try_recv() {
                log::info!("SyncCAN - stop sync receive.");
                break;
            }
        }

        sleep(Duration::from_micros(interval));
    }
}


