use axum::extract::ws::Message;
use dashmap::DashMap;
use stateroom::{
    ClientId, MessageRecipient, StateroomContext, StateroomService, StateroomServiceFactory,
};
use std::sync::{atomic::AtomicU32, Arc};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};

/// A [StateroomContext] implementation for [StateroomService]s hosted in the
/// context of a [ServiceActor].
#[derive(Clone)]
pub struct ServiceActorContext {
    senders: Arc<DashMap<ClientId, Sender<Message>>>,
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
                    todo!()
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
        // self.try_send(MessageFromServer::new_binary(
        //     recipient.into(),
        //     message.to_vec(),
        // ));
        todo!()
    }

    fn set_timer(&self, ms_delay: u32) {
        // self.set_timer_recipient.do_send(SetTimer(ms_delay));
        todo!()
    }
}

#[derive(Debug)]
pub struct ServerState {
    pub handle: JoinHandle<()>,
    pub inbound_sender: Sender<(ClientId, Message)>,
    pub receivers: Arc<DashMap<ClientId, Sender<Message>>>,
    pub next_client_id: AtomicU32,
}

impl ServerState {
    pub fn new<T: StateroomService + Send + Sync + 'static>(
        service_factory: impl StateroomServiceFactory<ServiceActorContext, Service = T> + Send + 'static,
    ) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(ClientId, Message)>(100);

        let receivers = Arc::new(DashMap::new());

        let receivers_ = receivers.clone();
        let handle = tokio::spawn(async move {
            let mut service = service_factory
                .build(
                    "",
                    ServiceActorContext {
                        senders: receivers_.clone(),
                    },
                )
                .unwrap();

            loop {
                let msg = rx.recv().await;
                match msg {
                    Some((client, Message::Text(msg))) => service.message(client, &msg),
                    Some(_) => todo!(),
                    None => break,
                }
            }
        });

        Self {
            handle,
            inbound_sender: tx,
            receivers,
            next_client_id: AtomicU32::new(1),
        }
    }

    pub fn remove(&self, client: &ClientId) {
        self.receivers.remove(client);
    }

    pub fn connect(&self) -> (Sender<(ClientId, Message)>, Receiver<Message>, ClientId) {
        let client_id = self
            .next_client_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let client_id = ClientId(client_id);
        let (tx, rx) = tokio::sync::mpsc::channel::<Message>(100);
        self.receivers.insert(self.next_client_id(), tx);
        (self.inbound_sender.clone(), rx, client_id)
    }

    fn next_client_id(&self) -> ClientId {
        let r = self
            .next_client_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        ClientId(r)
    }
}
