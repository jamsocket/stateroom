use bytemuck::cast;
use jamsocket_wasm::prelude::*;

#[jamsocket_wasm]
struct RandomServer;

impl SimpleJamsocketService for RandomServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        RandomServer
    }

    fn connect(&mut self, client_id: ClientId, ctx: &impl JamsocketContext) {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        unsafe {
            wasi::random_get(&mut buf[0] as *mut u8, 4).unwrap();
        }

        let num: [u32; 1] = cast(buf);

        ctx.send_message(
            client_id,
            &format!("User {:?} connected. Random number: {}", client_id, num[0]),
        );
    }

    fn message(&mut self, client_id: ClientId, message: &str, ctx: &impl JamsocketContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("User {:?} sent '{}'", client_id, message),
        );
    }

    fn disconnect(&mut self, client_id: ClientId, ctx: &impl JamsocketContext) {
        ctx.send_message(MessageRecipient::Broadcast, &format!("User {:?} left.", client_id));
    }
}
