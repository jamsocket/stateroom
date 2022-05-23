extern crate alloc;

/// Re-exports useful items from `stateroom` and `stateroom_wasm_macro`.
pub use stateroom::{
    ClientId, MessagePayload, MessageRecipient, RoomEvent, Stateroom, StateroomContext,
};
pub use stateroom_wasm_macro::stateroom_wasm;
use std::{
    task::{Context, Poll},
};
pub use crate::{GlobalStateroomContext, ROOM_FUTURE, initialize_room};

#[no_mangle]
pub static STATEROOM_API_VERSION: i32 = 1;

#[no_mangle]
pub static STATEROOM_API_PROTOCOL: i32 = 1;

#[no_mangle]
extern "C" fn message(ptr: *const u8, len: usize) {
    unsafe {
        let bytes = std::slice::from_raw_parts(ptr, len).to_vec();

        let message: RoomEvent = bincode::deserialize(&bytes).expect("Error deserializing.");

        crate::receive_message(message);
    }
}

#[no_mangle]
extern "C" fn poll() {
    let fut = unsafe { crate::ROOM_FUTURE.as_mut().expect("Room future expected.") };
    let waker = crate::dummy_waker::waker();
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
