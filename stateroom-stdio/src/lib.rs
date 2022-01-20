use std::process::Command;

use interactive_process::InteractiveProcess;
use stateroom::{
    StateroomContext, StateroomService, StateroomServiceFactory, MessageFromProcess,
    MessagePayload, MessageToProcess,
};

pub struct StdioProcessServiceFactory {
    command: String,
}

impl StdioProcessServiceFactory {
    #[must_use]
    pub fn new(command: &str) -> Self {
        StdioProcessServiceFactory {
            command: command.to_string(),
        }
    }
}

impl<T: StateroomContext> StateroomServiceFactory<T> for StdioProcessServiceFactory {
    type Service = StdioProcessService;
    type Error = std::io::Error;

    fn build(&self, _room_id: &str, context: T) -> Result<Self::Service, Self::Error> {
        let process = InteractiveProcess::new(Command::new(&self.command), move |line| {
            let line = line.expect("Error reading line from stdin.");
            let message: MessageFromProcess =
                serde_json::from_str(&line).expect("Couldn't parse message from process.");

            match message {
                MessageFromProcess::Message {
                    recipient,
                    message: MessagePayload::Bytes(message),
                } => {
                    context.send_binary(recipient, &message);
                }
                MessageFromProcess::Message {
                    recipient,
                    message: MessagePayload::Text(message),
                } => {
                    context.send_message(recipient, &message);
                }
            }
        })?;

        Ok(StdioProcessService { process })
    }
}

pub struct StdioProcessService {
    process: InteractiveProcess,
}

impl StdioProcessService {
    fn send_to_process(&mut self, message: &MessageToProcess) {
        self.process
            .send(&serde_json::to_string(&message).expect("Could not jsonify message."))
            .expect("Could not send message to process.");
    }
}

impl StateroomService for StdioProcessService {
    fn connect(&mut self, client: stateroom::ClientId) {
        self.send_to_process(&MessageToProcess::Connect { client });
    }

    fn disconnect(&mut self, client: stateroom::ClientId) {
        self.send_to_process(&MessageToProcess::Disconnect { client });
    }

    fn message(&mut self, sender: stateroom::ClientId, message: &str) {
        self.send_to_process(&MessageToProcess::Message {
            client: sender,
            message: MessagePayload::Text(message.to_string()),
        });
    }

    fn binary(&mut self, sender: stateroom::ClientId, message: &[u8]) {
        self.send_to_process(&MessageToProcess::Message {
            client: sender,
            message: MessagePayload::Bytes(message.to_vec()),
        });
    }

    fn timer(&mut self) {
        self.send_to_process(&MessageToProcess::Timer);
    }
}
