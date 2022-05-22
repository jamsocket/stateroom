use bytemuck::cast;
use stateroom_wasm::prelude::*;

#[stateroom_wasm]
#[derive(Default)]
struct RandomServer;

#[async_trait]
impl Stateroom for RandomServer {
    async fn go<C: StateroomContext>(self, mut ctx: C) -> () {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        unsafe {
            wasi::random_get(&mut buf[0] as *mut u8, 4).unwrap();
        }

        let num: [u32; 1] = cast(buf);

        loop {
            match ctx.next_message().await {
                MessageToRoom::Connect { client } => {
                    ctx.send(
                        MessageRecipient::Broadcast,
                        &format!("User {:?} connected. Random number: {}", client, num[0]),
                    );
                }
                MessageToRoom::Disconnect { client } => {
                    ctx.send(
                        MessageRecipient::Broadcast,
                        &format!("User {:?} left.", client),
                    );
                }
                MessageToRoom::Message { client, message } => {
                    if let MessagePayload::Text(t) = message {
                        ctx.send(
                            MessageRecipient::Broadcast,
                            &format!("User {:?} sent: {}.", client, t),
                        );
                    }
                }
            }
        }
    }
}
