use std::{collections::VecDeque, sync::{Arc, Mutex}};
use iso15765_2::IsoTpEvent;

#[derive(Debug, Default, Clone)]
pub struct IsoTpBuffer {
    inner: Arc<Mutex<VecDeque<IsoTpEvent>>>,
}

impl IsoTpBuffer {
    #[inline]
    pub fn clear(&self) {
        match self.inner.lock() {
            Ok(mut buffer) => buffer.clear(),
            Err(_) => {
                log::warn!("DoCAN - failed to acquire write lock for `IsoTpBuffer::clear`");
            },
        }
    }

    #[inline]
    pub fn set(&self, event: IsoTpEvent) {
        match self.inner.lock() {
            Ok(mut buffer) => buffer.push_back(event),
            Err(_) => {
                log::warn!("DoCAN - failed to acquire write lock for `IsoTpBuffer::set`");
            },
        }
    }

    #[inline]
    pub fn get(&self) -> Option<IsoTpEvent> {
        match self.inner.lock() {
            Ok(mut buffer) => buffer.pop_front(),
            Err(_) => {
                log::warn!("DoCAN - failed to acquire write lock for `IsoTpBuffer::get`");
                None
            },
        }
    }
}
