use stateroom_wasm::prelude::*;

#[stateroom_wasm]
#[derive(Default)]
struct ClockServer(u32);

impl StateroomService for ClockServer {
    fn init(&mut self, ctx: &impl StateroomContext) {
        ctx.set_timer(4000);
    }

    fn timer(&mut self, ctx: &impl StateroomContext) {
        ctx.send_message(MessageRecipient::Broadcast, &format!("Timer @ {}", self.0));
        self.0 += 1;
        ctx.set_timer(4000);
    }
}
