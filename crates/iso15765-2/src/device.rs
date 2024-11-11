//! Uniform Device Driver trait

use std::any::Any;


pub trait Listener<Channel, Id, Frame>: Any + Send {
    fn as_any(&self) -> &dyn Any;
    /// Callback when frame transmitting.
    fn on_frame_transmitting(&mut self, channel: Channel, frame: &Frame);
    /// Callback when frame transmit success.
    fn on_frame_transmitted(&mut self, channel: Channel, id: Id);
    /// Callback when frames received.
    fn on_frame_received(&mut self, channel: Channel, frames: &[Frame]);
}

pub trait Driver: Send {
    type Error;
    type C;
    type F;

    /// get all channels that has opened
    fn opened_channels(&self) -> Vec<Self::C>;

    /// closed flag.
    fn is_closed(&self) -> bool;

    /// Transmit a CAN or CAN-FD Frame.
    #[cfg(not(feature = "async"))]
    fn transmit(
        &self,
        msg: Self::F,
        timeout: Option<u32>,
    ) -> Result<(), Self::Error>;
    #[cfg(feature = "async")]
    fn transmit(
        &self,
        msg: Self::F,
        timeout: Option<u32>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>>;
    /// Receive CAN and CAN-FD Frames.
    #[cfg(not(feature = "async"))]
    fn receive(
        &self,
        channel: Self::C,
        timeout: Option<u32>,
    ) -> Result<Vec<Self::F>, Self::Error>;
    #[cfg(feature = "async")]
    fn receive(
        &self,
        channel: Self::C,
        timeout: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<Self::F>, Self::Error>>;
    /// Close CAN device.
    #[cfg(not(feature = "async"))]
    fn shutdown(&mut self);
    #[cfg(feature = "async")]
    fn shutdown(&mut self) -> impl std::future::Future<Output = ()>;

}
