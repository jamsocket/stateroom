use stateroom::MessageFromProcess;
pub use stateroom::{ClientId, MessageRecipient, StateroomContext, StateroomService};
pub use stateroom::{MessagePayload, MessageToProcess};
pub use stateroom_wasm_macro::stateroom_wasm;

type Callback = unsafe extern "C" fn(*const u8, u32);

pub struct WrappedStateroomService<S: StateroomService> {
    state: S,
    context: WasmStateroomContext,
}

impl<S: StateroomService> WrappedStateroomService<S> {
    pub fn new(state: S, callback: Callback) -> Self {
        Self {
            state,
            context: WasmStateroomContext { callback },
        }
    }

    pub fn recv(&mut self, message_ptr: *const u8, message_len: u32) {
        let message = unsafe { std::slice::from_raw_parts(message_ptr, message_len as usize) };
        let message: MessageToProcess = bincode::deserialize(message).unwrap();

        match message {
            MessageToProcess::Init => {
                self.state.init(&self.context);
            }
            MessageToProcess::Connect { client } => {
                self.state.connect(client.into(), &self.context);
            }
            MessageToProcess::Disconnect { client } => {
                self.state.disconnect(client.into(), &self.context);
            }
            MessageToProcess::Message { sender, message } => {
                self.state.message(sender, message, &self.context);
            }
            MessageToProcess::Timer => {
                self.state.timer(&self.context);
            }
        }
    }
}

#[derive(Clone)]
struct WasmStateroomContext {
    callback: Callback,
}

impl WasmStateroomContext {
    pub fn send(&self, message: &MessageFromProcess) {
        let message = bincode::serialize(message).unwrap();
        unsafe {
            (self.callback)(message.as_ptr(), message.len() as u32);
        }
    }
}

impl StateroomContext for WasmStateroomContext {
    fn send_message(
        &self,
        recipient: impl Into<MessageRecipient>,
        message: impl Into<MessagePayload>,
    ) {
        let message: MessagePayload = message.into();
        let recipient: MessageRecipient = recipient.into();

        self.send(&MessageFromProcess::Message { recipient, message });
    }

    fn set_timer(&self, ms_delay: u32) {
        self.send(&MessageFromProcess::SetTimer { ms_delay });
    }
}
