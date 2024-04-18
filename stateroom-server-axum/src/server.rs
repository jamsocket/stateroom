use axum::extract::ws::Message;
use dashmap::DashMap;
use stateroom::{ClientId, MessageRecipient, StateroomContext, StateroomService, StateroomServiceFactory};
use std::sync::atomic::AtomicU32;
use tokio::{sync::mpsc::{Receiver, Sender}, task::JoinHandle};

/// A [StateroomContext] implementation for [StateroomService]s hosted in the
/// context of a [ServiceActor].
#[derive(Clone)]
pub struct ServiceActorContext {}

impl StateroomContext for ServiceActorContext {
    fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str) {
        // self.try_send(MessageFromServer::new(
        //     recipient.into(),
        //     message.to_string(),
        // ));
        todo!()
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
    pub receivers: DashMap<ClientId, Sender<(ClientId, Message)>>,
    pub next_client_id: AtomicU32,
}

impl ServerState {
    pub fn new<T: StateroomService + Send + Sync + 'static>(service_factory: impl StateroomServiceFactory<ServiceActorContext, Service = T> + Send + 'static) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(ClientId, Message)>(100);
        
        let handle = tokio::spawn(async move {
            let mut service = service_factory.build("", ServiceActorContext {}).unwrap();

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
            receivers: DashMap::new(),
            next_client_id: AtomicU32::new(1),
        }
    }

    pub fn remove(&self, client: &ClientId) {
        self.receivers.remove(client);
    }

    pub fn connect(&self) -> (Sender<(ClientId, Message)>, Receiver<(ClientId, Message)>, ClientId) {
        let client_id = self.next_client_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let client_id = ClientId(client_id);
        let (tx, rx) = tokio::sync::mpsc::channel::<(ClientId, Message)>(100);
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
