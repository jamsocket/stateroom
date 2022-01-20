use stateroom_wasm::prelude::*;

#[stateroom_wasm]
struct ClockServer(String, u32);

impl SimpleStateroomService for ClockServer {
    fn new(room_id: &str, ctx: &impl StateroomContext) -> Self {
        ctx.set_timer(4000);
        ClockServer(room_id.to_string(), 0)
    }

    fn timer(&mut self, ctx: &impl StateroomContext) {
        ctx.send_message(MessageRecipient::Broadcast, &format!("Here in room {} from timer @ {}", self.0, self.1));
        self.1 += 1;
        ctx.set_timer(4000);
    }
}
