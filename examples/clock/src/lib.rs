use jamsocket_wasm::prelude::*;

#[jamsocket_wasm]
#[derive(Default)]
struct ClockServer {
    i: u32,
}

impl SimpleJamsocketService for ClockServer {
    fn initialize(&mut self, _: &str, ctx: &impl JamsocketContext) {
        ctx.set_timer(4000);
    }

    fn timer(&mut self, ctx: &impl JamsocketContext) {
        ctx.send_message(MessageRecipient::Broadcast, &format!("Here from timer @ {}", self.i));
        self.i += 1;
        ctx.set_timer(4000);
    }
}
