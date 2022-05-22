use stateroom_wasm::prelude::*;
use async_trait::async_trait;

#[stateroom_wasm]
#[derive(Default)]
struct EchoServer;

#[async_trait]
impl Stateroom for EchoServer {
    async fn go<C: StateroomContext>(self, mut ctx: C) -> () {
        loop {
            let message = ctx.next_message().await;

            match message {
                MessageToRoom::Connect { .. } => {
                    ctx.send(MessageRecipient::Broadcast, "Hello.");
                },
                MessageToRoom::Message { client, message: MessagePayload::Text(text) } => {
                    ctx.send(client, &format!("Got message: {}", text));
                },
                _ => ()
            }
        }
    }
}
