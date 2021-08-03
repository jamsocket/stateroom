use jamsocket_wasm::prelude::*;

#[jamsocket_wasm]
struct EchoServer;

impl SimpleJamsocketService for EchoServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        EchoServer
    }

    fn connect(&mut self, user: u32, ctx: &impl JamsocketContext) {
        ctx.send_message(user, &format!("User {} connected.", user));
    }

    fn message(&mut self, user: u32, message: &str, ctx: &impl JamsocketContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("User {} sent '{}'", user, message),
        );
    }

    fn disconnect(&mut self, user: u32, ctx: &impl JamsocketContext) {
        ctx.send_message(MessageRecipient::Broadcast, &format!("User {} left.", user));
    }
}