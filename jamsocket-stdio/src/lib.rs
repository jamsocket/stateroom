use std::process::Command;

use interactive_process::InteractiveProcess;
use jamsocket::{ClientId, JamsocketContext, JamsocketService, JamsocketServiceFactory, MessageRecipient};
use serde::{Deserialize, Serialize};

pub struct StdioProcessServiceFactory {
    command: String,
}

impl StdioProcessServiceFactory {
    pub fn new(command: &str) -> Self {
        StdioProcessServiceFactory {
            command: command.to_string(),
        }
    }
}

impl<T: JamsocketContext> JamsocketServiceFactory<T> for StdioProcessServiceFactory {
    type Service = StdioProcessService;

    fn build(&self, _room_id: &str, context: T) -> Option<Self::Service> {
        let process = InteractiveProcess::new(Command::new(&self.command), move |line| {
            let line = line.unwrap();
            let message: MessageFromProcess = serde_json::from_str(&line).expect("Couldn't parse message from process.");

            match message {
                MessageFromProcess::Message {recipient, message: Message::Bytes(message)} => {
                    context.send_binary(recipient, &message)
                },
                MessageFromProcess::Message {recipient, message: Message::Text(message)} => {
                    context.send_message(recipient, &message)
                },
            }

        }).ok()?;

        Some(StdioProcessService {
            process,
        })
    }
}

#[derive(Serialize, Deserialize)]
enum Message {
    Bytes(Vec<u8>),
    Text(String),
}

#[derive(Serialize, Deserialize)]
#[serde(tag="type")]
enum MessageToProcess {
    Connect {client: ClientId},
    Disconnect {client: ClientId},
    Message {client: ClientId, message: Message},
    Timer,
}

#[derive(Serialize, Deserialize)]
#[serde(tag="type")]
enum MessageFromProcess {
    Message {recipient: MessageRecipient, message: Message},
}

pub struct StdioProcessService {
    process: InteractiveProcess,

}

impl StdioProcessService {
    fn send_to_process(&mut self, message: MessageToProcess) {
        self.process
            .send(&serde_json::to_string(&message).expect("Could not jsonify message."))
            .expect("Could not send message to process.");
    }
}

impl JamsocketService for StdioProcessService {
    fn connect(&mut self, client: jamsocket::ClientId) {
        self.send_to_process(MessageToProcess::Connect {client});
    }

    fn disconnect(&mut self, client: jamsocket::ClientId) {
        self.send_to_process(MessageToProcess::Disconnect {client});
    }

    fn message(&mut self, sender: jamsocket::ClientId, message: &str) {
        self.send_to_process(MessageToProcess::Message {
            client: sender,
            message: Message::Text(message.to_string()),
        });
    }

    fn binary(&mut self, sender: jamsocket::ClientId, message: &[u8]) {
        self.send_to_process(MessageToProcess::Message {
            client: sender,
            message: Message::Bytes(message.to_vec()),
        });
    }

    fn timer(&mut self) {
        self.send_to_process(MessageToProcess::Timer);
    }
}
