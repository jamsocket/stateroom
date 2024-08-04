use axum::extract::ws::Message;
use dashmap::DashMap;
use stateroom::{
    ClientId, MessagePayload, MessageRecipient, StateroomContext, StateroomService,
    StateroomServiceFactory,
};
use std::{
    sync::{atomic::AtomicU32, Arc, Mutex},
    time::Duration,
};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};

/// A [StateroomContext] implementation for [StateroomService]s hosted in the
/// context of a [ServiceActor].
#[derive(Clone)]
pub struct ServerStateroomContext {
    senders: Arc<DashMap<ClientId, Sender<Message>>>,
    event_sender: Arc<Sender<Event>>,
    timer_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl ServerStateroomContext {
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
                    tracing::error!(?client_id, "No sender for client.");
                }
            }
        }
    }
}

impl StateroomContext for ServerStateroomContext {
    fn send_message(
        &self,
        recipient: impl Into<MessageRecipient>,
        message: impl Into<MessagePayload>,
    ) {
        let message: MessagePayload = message.into();
        let message: Message = match message {
            MessagePayload::Text(s) => Message::Text(s),
            MessagePayload::Bytes(b) => Message::Binary(b),
        };
        self.try_send(recipient.into(), message);
    }

    fn set_timer(&self, ms_delay: u32) {
        let sender = self.event_sender.clone();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(ms_delay as u64)).await;
            sender.send(Event::Timer).await.unwrap();
        });

        let mut c = self
            .timer_handle
            .lock()
            .expect("timer handle lock poisoned");
        if let Some(c) = c.take() {
            c.abort();
        }
        *c = Some(handle);
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
    Timer,
}

impl ServerState {
    pub fn new(factory: impl StateroomServiceFactory) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(100);

        let senders = Arc::new(DashMap::new());

        let senders_ = senders.clone();
        let tx_ = tx.clone();
        let handle = tokio::spawn(async move {
            let context = Arc::new(ServerStateroomContext {
                senders: senders_.clone(),
                event_sender: Arc::new(tx_),
                timer_handle: Arc::new(Mutex::new(None)),
            });

            let mut service = factory.build("", context.clone()).unwrap();
            service.init(context.as_ref());

            loop {
                let msg = rx.recv().await;
                match msg {
                    Some(Event::Message { client, message }) => match message {
                        Message::Text(msg) => {
                            service.message(client, MessagePayload::Text(msg), context.as_ref())
                        }
                        Message::Binary(msg) => {
                            service.message(client, MessagePayload::Bytes(msg), context.as_ref())
                        }
                        Message::Close(_) => {}
                        msg => tracing::warn!("Ignoring unhandled message: {:?}", msg),
                    },
                    Some(Event::Join { client }) => service.connect(client, context.as_ref()),
                    Some(Event::Leave { client }) => service.disconnect(client, context.as_ref()),
                    Some(Event::Timer) => {
                        service.timer(context.as_ref());
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
            .try_send(Event::Leave { client: *client })
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
