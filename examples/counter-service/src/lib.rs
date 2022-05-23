use stateroom_wasm::prelude::*;

#[stateroom_wasm]
async fn run<C: StateroomContext>(mut ctx: C) {
    let mut c = 0;

    loop {
        let message = ctx.next_event().await;
        if let RoomEvent::Message {
            message: MessagePayload::Text(message),
            ..
        } = message
        {
            match message.trim().as_ref() {
                "increment" => c += 1,
                "decrement" => c -= 1,
                _ => (),
            }

            ctx.send(MessageRecipient::Broadcast, &format!("new value: {}", c));
        }
    }
}
