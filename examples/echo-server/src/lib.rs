use stateroom_wasm::prelude::*;

#[stateroom_wasm]
#[derive(Default)]
struct EchoServer;

impl StateroomService for EchoServer {
    fn connect(&mut self, client_id: ClientId, ctx: &impl StateroomContext) {
        ctx.send_message(client_id, &format!("User {:?} connected.", client_id));
    }

    fn message(&mut self, client_id: ClientId, message: &str, ctx: &impl StateroomContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("User {:?} sent '{}'", client_id, message),
        );
    }

    fn disconnect(&mut self, client_id: ClientId, ctx: &impl StateroomContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("User {:?} left.", client_id),
        );
    }
}
