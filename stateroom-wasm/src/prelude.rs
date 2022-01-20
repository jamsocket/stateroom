/// Re-exports useful items from `stateroom` and `stateroom_wasm_macro`.
pub use stateroom::{
    ClientId, StateroomContext, StateroomService, StateroomServiceFactory, MessageRecipient,
    SimpleStateroomService, WrappedStateroomService,
};
pub use stateroom_wasm_macro::stateroom_wasm;
