use jamsocket_wasm::prelude::*;

#[jamsocket_wasm]
#[derive(Default)]
struct BinaryEcho;

impl SimpleJamsocketService for BinaryEcho {
    fn message(&mut self, _: u32, message: &str, ctx: &impl JamsocketContext) {
        ctx.send_binary(
            MessageRecipient::Broadcast,
            message.as_bytes(),
        );
    }

    fn binary(&mut self, _: u32, message: &[u8], ctx: &impl JamsocketContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("Received binary data: {:?}", &message),
        );
    }
}
