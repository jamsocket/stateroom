use jamsocket_wasm::prelude::*;

#[jamsocket_wasm]
struct ClockServer(String, u32);

impl SimpleJamsocketService for ClockServer {
    fn new(room_id: &str, ctx: &impl JamsocketContext) -> Self {
        ctx.set_timer(4000);
        ClockServer(room_id.to_string(), 0)
    }

    fn timer(&mut self, ctx: &impl JamsocketContext) {
        ctx.send_message(MessageRecipient::Broadcast, &format!("Here in room {} from timer @ {}", self.0, self.1));
        self.1 += 1;
        ctx.set_timer(4000);
    }
}
