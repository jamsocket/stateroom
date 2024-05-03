use stateroom_wasm::*;

#[stateroom_wasm]
#[derive(Default)]
struct SharedCounterServer(i32);

impl StateroomService for SharedCounterServer {
    fn message(&mut self, _: ClientId, message: MessagePayload, ctx: &impl StateroomContext) {
        let Some(message) = message.text() else {
            return;
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
