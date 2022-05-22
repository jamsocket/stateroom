use stateroom_wasm::prelude::*;

#[stateroom_wasm]
#[derive(Default)]
struct SharedCounterServer;

#[async_trait]
impl Stateroom for SharedCounterServer {
    async fn go<C: StateroomContext>(self, mut ctx: C) -> () {
        let mut c = 0;

        loop {
            let message = ctx.next_message().await;
            if let MessageToRoom::Message {
                message: MessagePayload::Text(message),
                ..
            } = message
            {
                match message.trim().as_ref() {
                    "increment" => c += 1,
                    "decrement" => c -= 1,
                    _ => (),
                }

                ctx.send(
                    MessageRecipient::Broadcast,
                    &format!("new value: {}", c),
                );
            }
        }
    }
}
