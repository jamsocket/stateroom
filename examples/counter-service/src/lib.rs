use stateroom_wasm::{
    stateroom_wasm, ClientId, MessageRecipient, StateroomContext, StateroomService, MessagePayload
};

#[stateroom_wasm]
#[derive(Default)]
struct SharedCounterServer(i32);

impl StateroomService for SharedCounterServer {
    fn message(&mut self, _: ClientId, message: MessagePayload, ctx: &impl StateroomContext) {
        let message = match message {
            MessagePayload::Text(s) => s,
            MessagePayload::Bytes(_) => return,
        };

        match &message[..] {
            "increment" => self.0 += 1,
            "decrement" => self.0 -= 1,
            _ => (),
        }

        ctx.send_message(
            MessageRecipient::Broadcast,
            format!("new value: {}", self.0),
        );
    }
}
