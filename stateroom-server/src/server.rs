use axum::extract::ws::Message;
use dashmap::DashMap;
use stateroom::{
    ClientId, MessageRecipient, StateroomContext, StateroomService, StateroomServiceFactory,
};
use std::{
    sync::{atomic::AtomicU32, Arc},
    time::Duration,
};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};

/// A [StateroomContext] implementation for [StateroomService]s hosted in the
/// context of a [ServiceActor].
#[derive(Clone)]
pub struct ServiceActorContext {
    senders: Arc<DashMap<ClientId, Sender<Message>>>,
    event_sender: Sender<Event>,
}

impl ServiceActorContext {
    pub fn try_send(&self, recipient: MessageRecipient, message: Message) {
        match recipient {
            MessageRecipient::Broadcast => {
                for sender in self.senders.iter() {
                    sender.value().try_send(message.clone()).unwrap();
                }
            }
            MessageRecipient::EveryoneExcept(skip_client_id) => {
                for sender in self.senders.iter() {
                    if sender.key() != &skip_client_id {
                        sender.try_send(message.clone()).unwrap();
                    }
                }
            }
            MessageRecipient::Client(client_id) => {
                if let Some(sender) = self.senders.get(&client_id) {
                    sender.try_send(message).unwrap();
                } else {
                    println!("No sender for client {:?}", client_id);
                }
            }
        }
    }
}

impl StateroomContext for ServiceActorContext {
    fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str) {
        self.try_send(recipient.into(), Message::Text(message.to_string()));
    }

    fn send_binary(&self, recipient: impl Into<MessageRecipient>, message: &[u8]) {
        self.try_send(recipient.into(), Message::Binary(message.to_vec()));
    }

    fn set_timer(&self, ms_delay: u32) {
        // TODO: setting the timer should replace the previous timer?
        let sender = self.event_sender.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(ms_delay as u64)).await;
            sender.send(Event::TimerEvent).await.unwrap();
        });
    }
}

#[derive(Debug)]
pub struct ServerState {
    pub handle: JoinHandle<()>,
    pub inbound_sender: Sender<Event>,
    pub senders: Arc<DashMap<ClientId, Sender<Message>>>,
    pub next_client_id: AtomicU32,
}

#[derive(Debug)]
pub enum Event {
    Message { client: ClientId, message: Message },
    Join { client: ClientId },
    Leave { client: ClientId },
    TimerEvent,
}

impl ServerState {
    pub fn new<T: StateroomService + Send + Sync + 'static>(
        service_factory: impl StateroomServiceFactory<ServiceActorContext, Service = T> + Send + 'static,
    ) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(100);

        let senders = Arc::new(DashMap::new());

        let senders_ = senders.clone();
        let tx_ = tx.clone();
        let handle = tokio::spawn(async move {
            let mut service = service_factory
                .build(
                    "",
                    ServiceActorContext {
                        senders: senders_.clone(),
                        event_sender: tx_,
                    },
                )
                .unwrap();

            loop {
                let msg = rx.recv().await;
                println!("{:?}", msg);
                match msg {
                    Some(Event::Message { client, message }) => match message {
                        Message::Text(msg) => service.message(client, &msg),
                        Message::Binary(msg) => service.binary(client, &msg),
                        Message::Close(_) => {}
                        msg => println!("Ignoring unhandled message: {:?}", msg),
                    },
                    Some(Event::Join { client }) => service.connect(client),
                    Some(Event::Leave { client }) => service.disconnect(client),
                    Some(Event::TimerEvent) => {
                        service.timer();
                    }
                    None => break,
                }
            }
        });

        Self {
            handle,
            inbound_sender: tx,
            senders,
            next_client_id: AtomicU32::new(1),
        }
    }

    pub fn remove(&self, client: &ClientId) {
        self.inbound_sender
            .try_send(Event::Leave {
                client: client.clone(),
            })
            .unwrap();
        self.senders.remove(client);
    }

    pub fn connect(&self) -> (Sender<Event>, Receiver<Message>, ClientId) {
        let client_id = self.next_client_id();
        let (tx, rx) = tokio::sync::mpsc::channel::<Message>(100);

        self.senders.insert(client_id, tx);
        self.inbound_sender
            .try_send(Event::Join { client: client_id })
            .unwrap();
        (self.inbound_sender.clone(), rx, client_id)
    }

    fn next_client_id(&self) -> ClientId {
        let r = self
            .next_client_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        ClientId(r)
    }
}
