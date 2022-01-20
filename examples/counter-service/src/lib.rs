use stateroom_wasm::prelude::*;

#[stateroom_wasm]
struct SharedCounterServer(i32);

impl SimpleStateroomService for SharedCounterServer {
    fn new(_: &str, _: &impl StateroomContext) -> Self {
        SharedCounterServer(0)
    }

    fn message(&mut self, _: ClientId, message: &str, ctx: &impl StateroomContext) {
        match message {
            "increment" => self.0 += 1,
            "decrement" => self.0 -= 1,
            _ => (),
        }

        ctx.send_message(MessageRecipient::Broadcast, &format!("new value: {}", self.0));
    }
}