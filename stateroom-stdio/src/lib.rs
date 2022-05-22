use async_trait::async_trait;
use interactive_process::InteractiveProcess;
use stateroom::{MessageFromRoom, MessagePayload, Stateroom, StateroomContext};
use std::process::Command;

pub struct StdioProcessService {
    command: String,
}

impl StdioProcessService {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
        }
    }
}

#[async_trait]
impl Stateroom for StdioProcessService {
    async fn run<C: StateroomContext>(mut self, mut ctx: C) -> () {
        let (send, mut recv) = tokio::sync::mpsc::channel::<MessageFromRoom>(256);

        let mut process = {
            InteractiveProcess::new(Command::new(&self.command), move |line| {
                let line = line.expect("Error reading line from stdin.");
                let message: MessageFromRoom =
                    serde_json::from_str(&line).expect("Couldn't parse message from process.");
                send.try_send(message).unwrap();
            })
            .unwrap()
        };

        loop {
            tokio::select! {
                message = ctx.next_message() => {
                    process
                        .send(&serde_json::to_string(&message).expect("Could not jsonify message."))
                        .expect("Could not send message to process.");
                },
                message = recv.recv() => {
                    let MessageFromRoom::Message { recipient, message } = message.unwrap();
                    match message {
                        MessagePayload::Bytes(b) => ctx.send_binary(recipient, &b),
                        MessagePayload::Text(t) => ctx.send(recipient, &t),
                    }
                }
            }
        }
    }
}
