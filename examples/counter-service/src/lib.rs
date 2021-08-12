use jamsocket_wasm::prelude::*;

#[jamsocket_wasm]
struct SharedCounterServer(i32);

impl SimpleJamsocketService for SharedCounterServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        SharedCounterServer(0)
    }

    fn message(&mut self, _: ClientId, message: &str, ctx: &impl JamsocketContext) {
        match message {
            "increment" => self.0 += 1,
            "decrement" => self.0 -= 1,
            _ => (),
        }

        ctx.send_message(MessageRecipient::Broadcast, &format!("new value: {}", self.0));
    }
}