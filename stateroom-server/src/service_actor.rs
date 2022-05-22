use crate::messages::{MessageFromClient, MessageFromServer};
use actix::{Actor, Context, Handler, Recipient};
use async_trait::async_trait;
use stateroom::{MessageRecipient, MessageToRoom, Stateroom, StateroomContext};
use std::marker::PhantomData;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

pub struct ServiceActor<J: Stateroom> {
    _handle: JoinHandle<()>,
    _ph: PhantomData<J>,
    message_sender: Sender<MessageToRoom>,
}

/// A [StateroomContext] implementation for [StateroomService]s hosted in the
/// context of a [ServiceActor].
pub struct ServiceActorContext {
    send_message_recipient: Recipient<MessageFromServer>,
    message_receiver: Receiver<MessageToRoom>,
}

impl ServiceActorContext {
    fn try_send(&self, message: MessageFromServer) {
        self.send_message_recipient.do_send(message);
    }
}

#[async_trait]
impl StateroomContext for ServiceActorContext {
    async fn next_message(&mut self) -> MessageToRoom {
        self.message_receiver
            .recv()
            .await
            .expect("Channel unexpectedly closed.")
    }

    fn send(&self, recipient: impl Into<MessageRecipient>, message: &str) {
        self.try_send(MessageFromServer::new(
            recipient.into(),
            message.to_string(),
        ));
    }

    fn send_binary(&self, recipient: impl Into<MessageRecipient>, message: &[u8]) {
        self.try_send(MessageFromServer::new_binary(
            recipient.into(),
            message.to_vec(),
        ));
    }
}

impl<J: Stateroom> ServiceActor<J> {
    #[must_use]
    pub fn new(stateroom: J, recipient: Recipient<MessageFromServer>) -> Option<Self> {
        let (sender, receiver) = channel(256);

        let host_context = ServiceActorContext {
            send_message_recipient: recipient,
            message_receiver: receiver,
        };

        let handle = actix::spawn(async { stateroom.go(host_context).await });

        Some(ServiceActor {
            _handle: handle,
            message_sender: sender,
            _ph: PhantomData::default(),
        })
    }
}

impl<J: Stateroom> Actor for ServiceActor<J> {
    type Context = Context<Self>;

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        tracing::info!("Shutting down service");
        actix::Running::Stop
    }
}

impl<J: Stateroom> Handler<MessageFromClient> for ServiceActor<J> {
    type Result = ();

    fn handle(&mut self, msg: MessageFromClient, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            MessageFromClient::Connect(client, _) => {
                self.message_sender
                    .try_send(MessageToRoom::Connect { client })
                    .expect("Buffer reached.");
            }
            MessageFromClient::Disconnect(client) => {
                self.message_sender
                    .try_send(MessageToRoom::Disconnect { client })
                    .expect("Buffer reached.");
            }
            MessageFromClient::Message { data, client } => {
                self.message_sender
                    .try_send(MessageToRoom::Message {
                        client,
                        message: data,
                    })
                    .expect("Buffer reached.");
            }
        }
    }
}
