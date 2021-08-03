use bytemuck::cast;
use jamsocket_wasm::prelude::*;

#[jamsocket_wasm]
struct RandomServer;

impl SimpleJamsocketService for RandomServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        RandomServer
    }

    fn connect(&mut self, user: u32, ctx: &impl JamsocketContext) {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        unsafe {
            wasi::random_get(&mut buf[0] as *mut u8, 4).unwrap();
        }

        let num: [u32; 1] = cast(buf);

        ctx.send_message(
            user,
            &format!("User {} connected. Random number: {}", user, num[0]),
        );
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
