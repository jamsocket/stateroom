use stateroom_wasm::*;

#[stateroom_wasm]
#[derive(Default)]
struct BinaryEcho;

impl StateroomService for BinaryEcho {
    fn message(&mut self, _: ClientId, message: MessagePayload, ctx: &impl StateroomContext) {
        if let Some(message) = message.text() {
            ctx.send_message(MessageRecipient::Broadcast, message);
        }
    }
}
