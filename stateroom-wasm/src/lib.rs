use stateroom::{
    MessageRecipient, RoomEvent, Stateroom, StateroomContext,
};
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Poll},
};

#[cfg(not(test))]
pub mod prelude;

static mut MESSAGE_QUEUE: Option<VecDeque<RoomEvent>> = None;
pub static mut ROOM_FUTURE: Option<Pin<Box<dyn Future<Output = ()>>>> = None;

struct NextMessageFuture;

impl Future for NextMessageFuture {
    type Output = RoomEvent;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let next_message = unsafe { MESSAGE_QUEUE.as_mut().unwrap().pop_front() };
        if let Some(message) = next_message {
            Poll::Ready(message)
        } else {
            Poll::Pending
        }
    }
}

#[allow(unused)]
mod dummy_waker {
    use std::{
        ptr,
        task::{RawWaker, RawWakerVTable, Waker},
    };

    type WakerData = *const ();

    unsafe fn clone(_: WakerData) -> RawWaker {
        raw_waker()
    }
    unsafe fn wake(_: WakerData) {
        panic!("Should never wake dummy waker!")
    }
    unsafe fn wake_by_ref(_: WakerData) {
        panic!("Should never wake dummy waker!")
    }
    unsafe fn drop(_: WakerData) {}

    static MY_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    fn raw_waker() -> RawWaker {
        RawWaker::new(ptr::null(), &MY_VTABLE)
    }

    pub fn waker() -> Waker {
        unsafe { Waker::from_raw(raw_waker()) }
    }
}

pub fn initialize_room<T: Stateroom + Default>() {
    let room = T::default();
    let future = room.run(GlobalStateroomContext);

    unsafe {
        MESSAGE_QUEUE.replace(VecDeque::new());
        ROOM_FUTURE.replace(future);
    }
}

pub fn receive_message(message: RoomEvent) {
    unsafe {
        MESSAGE_QUEUE
            .as_mut()
            .expect("Queue not available.")
            .push_back(message);
    }
}

// Functions implemented by the host.
mod ffi {
    extern "C" {
        pub fn send_message(client: u32, message: u32, message_len: u32);

        pub fn send_binary(client: u32, message: u32, message_len: u32);
    }
}

pub struct GlobalStateroomContext;

impl StateroomContext for GlobalStateroomContext {
    fn send(&self, recipient: impl Into<MessageRecipient>, message: &str) {
        unsafe {
            ffi::send_message(
                recipient.into().encode_u32(),
                &message.as_bytes()[0] as *const u8 as u32,
                message.len() as u32,
            );
        }
    }

    fn send_binary(&self, recipient: impl Into<MessageRecipient>, message: &[u8]) {
        unsafe {
            ffi::send_binary(
                recipient.into().encode_u32(),
                &message[0] as *const u8 as u32,
                message.len() as u32,
            );
        }
    }

    fn next_event<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> Pin<Box<dyn Future<Output = RoomEvent> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(NextMessageFuture)
    }
}