use stateroom_wasm::{
    stateroom_wasm, ClientId, MessageRecipient, StateroomContext, StateroomService, MessagePayload
};

#[stateroom_wasm]
#[derive(Default)]
struct BinaryEcho;

impl StateroomService for BinaryEcho {
    fn message(&mut self, _: ClientId, message: MessagePayload, ctx: &impl StateroomContext) {
        let message = match message {
            MessagePayload::Text(s) => MessagePayload::Bytes(s.as_bytes().to_vec()),
            MessagePayload::Bytes(b) => MessagePayload::Text(format!("{:?}", b)),
        };
        ctx.send_message(MessageRecipient::Broadcast, message);
    }
}
