use stateroom_wasm::prelude::*;

#[stateroom_wasm]
async fn run<C: StateroomContext>(self, mut ctx: C) {
    loop {
        let message = ctx.next_message().await;

        match message {
            MessageToRoom::Connect { .. } => {
                ctx.send(MessageRecipient::Broadcast, "Hello.");
            }
            MessageToRoom::Message {
                client,
                message: MessagePayload::Text(text),
            } => {
                ctx.send(client, &format!("Got message: {}", text));
            }
            _ => (),
        }
    }
}
