use stateroom_wasm::prelude::*;

#[stateroom_wasm]
struct BinaryEcho;

impl SimpleStateroomService for BinaryEcho {
    fn new(_: &str, _: &impl StateroomContext) -> Self {
        BinaryEcho
    }

    fn message(&mut self, _: ClientId, message: &str, ctx: &impl StateroomContext) {
        ctx.send_binary(
            MessageRecipient::Broadcast,
            message.as_bytes(),
        );
    }

    fn binary(&mut self, _: ClientId, message: &[u8], ctx: &impl StateroomContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("Received binary data: {:?}", &message),
        );
    }
}
