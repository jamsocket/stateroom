extern crate alloc;

/// Re-exports useful items from `stateroom` and `stateroom_wasm_macro`.
pub use stateroom::{
    ClientId, MessagePayload, MessageRecipient, RoomEvent, Stateroom, StateroomContext,
};
pub use stateroom_wasm_macro::stateroom_wasm;
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

static mut MESSAGE_QUEUE: Option<VecDeque<RoomEvent>> = None;
pub static mut ROOM_FUTURE: Option<Pin<Box<dyn Future<Output = ()>>>> = None;

// Functions implemented by the host.
mod ffi {
    extern "C" {
        pub fn send_message(client: u32, message: u32, message_len: u32);

        pub fn send_binary(client: u32, message: u32, message_len: u32);
    }
}

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

#[cfg(not(test))]
mod ffi {

#[no_mangle]
pub static STATEROOM_API_VERSION: i32 = 1;

#[no_mangle]
pub static STATEROOM_API_PROTOCOL: i32 = 1;

#[no_mangle]
extern "C" fn message(ptr: *const u8, len: usize) {
    unsafe {
        let bytes = std::slice::from_raw_parts(ptr, len).to_vec();

        let message: RoomEvent = bincode::deserialize(&bytes).expect("Error deserializing.");

        receive_message(message);
    }
}

#[no_mangle]
extern "C" fn poll() {
    let fut = unsafe { ROOM_FUTURE.as_mut().expect("Room future expected.") };
    let waker = dummy_waker::waker();
    let mut cx = Context::from_waker(&waker);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(_) => todo!("Task completed (unhandled!)"),
        Poll::Pending => (),
    }
}

#[no_mangle]
pub unsafe extern "C" fn stateroom_malloc(size: u32) -> *mut u8 {
    let layout = core::alloc::Layout::from_size_align_unchecked(size as usize, 0);
    alloc::alloc::alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn stateroom_free(ptr: *mut u8, size: u32) {
    let layout = core::alloc::Layout::from_size_align_unchecked(size as usize, 0);
    alloc::alloc::dealloc(ptr, layout);
}
}
