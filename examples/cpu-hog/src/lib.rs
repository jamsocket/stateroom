use stateroom_wasm::{
    stateroom_wasm, ClientId, MessageRecipient, StateroomContext, StateroomService,
};

// Seconds per nanosecond. (`wasi::clock_time_get` uses nanos.)
const SECONDS: u64 = 1_000_000_000;

#[stateroom_wasm]
#[derive(Default)]
struct CpuHog;

fn get_time() -> u64 {
    unsafe { wasi::clock_time_get(wasi::CLOCKID_REALTIME, 0).unwrap() }
}

impl StateroomService for CpuHog {
    fn connect(&mut self, _: ClientId, ctx: &impl StateroomContext) {
        ctx.send_message(MessageRecipient::Broadcast, &format!("Connected."));

        let init_time = get_time();
        loop {
            let cur_time = get_time();
            if init_time + 10 * SECONDS < cur_time {
                break;
            }
        }

        ctx.send_message(MessageRecipient::Broadcast, &format!("Finished."));
    }
}
