use crate::messages::{AssignClientId, MessageFromClient, MessageFromServer};
use actix::{dev::MessageResponse, Actor, ActorContext, Context, Handler, Message, Recipient};
use stateroom::{ClientId, MessageRecipient};
use std::collections::HashMap;

/// Actor model representation of a “room”. A room is a set of clients
/// that share an instance of a Stateroom instance. Conceptually, this
/// is like a room in a chat service. Events (such as messages) and their
/// side-effects are isolated to the room in which they occur.
pub struct RoomActor {
    service_actor: Option<Recipient<MessageFromClient>>,
    connections: HashMap<ClientId, Recipient<MessageFromServer>>,
    /// User IDs are assigned sequentially within the context of each room,
    /// ensuring that they never overlap. `next_id` stores the next ID that
    /// will be assigned.
    next_id: u32,
}

struct Shutdown;

impl Message for Shutdown {
    type Result = ();
}

impl RoomActor {
    #[must_use]
    pub fn new(service_actor: Recipient<MessageFromClient>) -> Self {
        RoomActor {
            service_actor: Some(service_actor),
            connections: HashMap::default(),
            next_id: 1,
        }
    }
}

impl Actor for RoomActor {
    type Context = Context<Self>;
}

impl Handler<MessageFromServer> for RoomActor {
    type Result = ();

    fn handle(&mut self, message: MessageFromServer, _ctx: &mut Context<Self>) {
        match message.to_client {
            MessageRecipient::Broadcast => {
                for addr in self.connections.values() {
                    addr.do_send(message.clone());
                }
            }
            MessageRecipient::Client(client_id) => {
                if let Some(client_connection) = self.connections.get(&client_id) {
                    client_connection.do_send(message);
                } else {
                    tracing::warn!(
                        ?client_id,
                        "Could not get address of user, who may have disconnected",
                    );
                }
            }
        }
    }
}

impl Handler<MessageFromClient> for RoomActor {
    type Result = ();

    fn handle(&mut self, message: MessageFromClient, _ctx: &mut Context<Self>) {
        if let Some(service_actor) = &self.service_actor {
            match &message {
                MessageFromClient::Connect(client, resp) => {
                    self.connections.insert(*client, resp.clone());
                    service_actor.do_send(message);
                }
                MessageFromClient::Disconnect(client_id) => {
                    self.connections.remove(client_id);

                    service_actor.do_send(message);
                }
                MessageFromClient::Message { .. } => {
                    service_actor.do_send(message);
                }
            }
        } else {
            tracing::warn!("MessageFromClient received on room with no service attached",);
        }
    }
}

impl MessageResponse<RoomActor, AssignClientId> for ClientId {
    fn handle(self, _: &mut Context<RoomActor>, tx: Option<actix::dev::OneshotSender<ClientId>>) {
        if let Some(tx) = tx {
            if let Err(error) = tx.send(self) {
                // TODO: checking this avoids a linter warning, but I need to better
                // understand the series of events that would lead to this triggering.
                tracing::error!(?error, "Error returning response to AssignClientId");
            }
        }
    }
}

impl Handler<AssignClientId> for RoomActor {
    type Result = ClientId;

    fn handle(&mut self, _: AssignClientId, _ctx: &mut Context<Self>) -> ClientId {
        let result = self.next_id;
        self.next_id += 1;

        result.into()
    }
}

impl Handler<Shutdown> for RoomActor {
    type Result = ();

    fn handle(&mut self, _: Shutdown, ctx: &mut Self::Context) -> Self::Result {
        tracing::info!(
            "Shutting down service actor because no clients are left and the timeout period has elapsed",
        );

        ctx.stop();
    }
}
