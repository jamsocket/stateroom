use jamsocket_wasm::prelude::*;

// Seconds per nanosecond. (`wasi::clock_time_get` uses nanos.)
const SECONDS: u64 = 1_000_000_000;

#[jamsocket_wasm]
struct CpuHog(String);

fn get_time() -> u64 {
    unsafe {
        wasi::clock_time_get(wasi::CLOCKID_REALTIME, 0).unwrap()
    }
}

impl SimpleJamsocketService for CpuHog {
    fn new(room_id: &str, _: &impl JamsocketContext) -> Self {
        CpuHog(room_id.to_string())
    }

    fn connect(&mut self, _: ClientId, ctx: &impl JamsocketContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("Connected to room {}", self.0),
        );
        
        let init_time = get_time();
        loop {
            let cur_time = get_time();
            if init_time + 10 * SECONDS < cur_time {
                break;
            }
        }
    
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("Finished in room {}", self.0),
        );
    }
}
