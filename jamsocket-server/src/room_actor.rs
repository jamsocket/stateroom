use crate::messages::{AssignUserId, MessageFromClient, MessageFromServer};
use actix::{Actor, Context, Handler, Recipient};
use jamsocket::MessageRecipient;
use std::collections::HashMap;

/// Actor model representation of a “room”. A room is a set of clients
/// that share an instance of a Jamsocket instance. Conceptually, this
/// is like a room in a chat service. Events (such as messages) and their
/// side-effects are isolated to the room in which they occur.
pub struct RoomActor {
    service_actor: Recipient<MessageFromClient>,
    connections: HashMap<u32, Recipient<MessageFromServer>>,
    /// User IDs are assigned sequentially within the context of each room,
    /// ensuring that they never overlap. `next_id` stores the next ID that
    /// will be assigned.
    next_id: u32,
}

impl RoomActor {
    pub fn new(service_actor: Recipient<MessageFromClient>) -> Self {
        RoomActor {
            service_actor,
            connections: Default::default(),
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
        match message.to_user {
            MessageRecipient::Broadcast => {
                for addr in self.connections.values() {
                    addr.do_send(message.clone()).unwrap();
                }
            }
            MessageRecipient::User(u) => {
                if let Some(client_connection) = self.connections.get(&u) {
                    client_connection.do_send(message).unwrap();
                } else {
                    log::warn!(
                        "Could not get address of user {:?}; they may have disconnected.",
                        u
                    );
                }
            }
        }
    }
}

impl Handler<MessageFromClient> for RoomActor {
    type Result = ();

    fn handle(&mut self, message: MessageFromClient, _ctx: &mut Context<Self>) {
        match &message {
            MessageFromClient::Connect(u, resp) => {
                self.connections.insert(*u, resp.clone());
                self.service_actor.do_send(message).unwrap();
            }
            MessageFromClient::Disconnect(u) => {
                self.connections.remove(&u);
                self.service_actor.do_send(message).unwrap();
            }
            MessageFromClient::Message { .. } => {
                self.service_actor.do_send(message).unwrap();
            }
        }
    }
}

impl Handler<AssignUserId> for RoomActor {
    type Result = u32;

    fn handle(&mut self, _: AssignUserId, _ctx: &mut Context<Self>) -> u32 {
        let result = self.next_id;
        self.next_id += 1;

        result
    }
}
