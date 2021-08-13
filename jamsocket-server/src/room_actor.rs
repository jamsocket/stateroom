use crate::{
    messages::{AssignClientId, MessageFromClient, MessageFromServer},
    ServiceShutdownPolicy,
};
use actix::{
    dev::MessageResponse, Actor, ActorContext, AsyncContext, Context, Handler, Message, Recipient,
    SpawnHandle,
};
use jamsocket::{ClientId, MessageRecipient};
use std::{collections::HashMap, time::Duration};

/// Actor model representation of a “room”. A room is a set of clients
/// that share an instance of a Jamsocket instance. Conceptually, this
/// is like a room in a chat service. Events (such as messages) and their
/// side-effects are isolated to the room in which they occur.
pub struct RoomActor {
    room_id: String,
    service_actor: Option<Recipient<MessageFromClient>>,
    connections: HashMap<ClientId, Recipient<MessageFromServer>>,
    /// User IDs are assigned sequentially within the context of each room,
    /// ensuring that they never overlap. `next_id` stores the next ID that
    /// will be assigned.
    next_id: u32,
    shutdown_policy: ServiceShutdownPolicy,
    shutdown_handle: Option<SpawnHandle>,
}

struct Shutdown;

impl Message for Shutdown {
    type Result = ();
}

impl RoomActor {
    #[must_use]
    pub fn new(
        room_id: String,
        service_actor: Recipient<MessageFromClient>,
        shutdown_policy: ServiceShutdownPolicy,
    ) -> Self {
        RoomActor {
            room_id,
            service_actor: Some(service_actor),
            connections: HashMap::default(),
            next_id: 1,
            shutdown_policy,
            shutdown_handle: None,
        }
    }

    fn handle_empty_room(&mut self, ctx: &mut Context<Self>) {
        match self.shutdown_policy {
            ServiceShutdownPolicy::Immediate => {
                log::info!(
                    "Shutting down service actor for {} because no clients are left.",
                    &self.room_id
                );

                ctx.stop();
            }
            ServiceShutdownPolicy::Never => (),
            ServiceShutdownPolicy::AfterSeconds(secs) => {
                self.shutdown_handle =
                    Some(ctx.notify_later(Shutdown, Duration::from_secs(secs.into())));
            }
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
                    if addr.do_send(message.clone()).is_err() {
                        log::warn!("Could not forward server-sent message to client in room {}", self.room_id);
                    }
                }
            }
            MessageRecipient::Client(u) => {
                if let Some(client_connection) = self.connections.get(&u) {
                    if client_connection.do_send(message).is_err() {
                        log::warn!("Could not forward server-sent binary message to client in room {}", self.room_id);
                    }
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

    fn handle(&mut self, message: MessageFromClient, ctx: &mut Context<Self>) {
        if let Some(service_actor) = &self.service_actor {
            match &message {
                MessageFromClient::Connect(client, resp) => {
                    self.connections.insert(*client, resp.clone());
                    if service_actor.do_send(message).is_err() {
                        log::warn!("Couldn't forward client message to service in room {}", self.room_id);
                    }

                    // If this task was scheduled to shut down becuse the room is empty,
                    // cancel that.
                    self.shutdown_handle.take().map(|t| ctx.cancel_future(t));
                }
                MessageFromClient::Disconnect(client_id) => {
                    self.connections.remove(client_id);

                    let empty_room = self.connections.is_empty();
                    let send_message = !empty_room || self.shutdown_policy != ServiceShutdownPolicy::Immediate;

                    #[allow(clippy::collapsible_if)]
                    if send_message {
                        if service_actor.do_send(message).is_err() {
                            log::warn!("Couldn't forward client message to service in room {}", self.room_id);
                        }
                    }

                    if empty_room {
                        self.handle_empty_room(ctx);
                    }
                }
                MessageFromClient::Message { .. } => {
                    if service_actor.do_send(message).is_err() {
                        log::warn!("Couldn't forward message from client to service in room {}", self.room_id);
                    }
                }
            }
        } else {
            log::warn!(
                "MessageFromClient received on room with no service attached ({}).",
                self.room_id
            );
        }
    }
}

impl MessageResponse<RoomActor, AssignClientId> for ClientId {
    fn handle(self, _: &mut Context<RoomActor>, tx: Option<actix::dev::OneshotSender<ClientId>>) {
        if let Some(tx) = tx {
            if let Err(e) = tx.send(self) {
                // TODO: checking this avoids a linter warning, but I need to better
                // understand the series of events that would lead to this triggering.
                log::warn!("Error returning response to AssignClientId: {:?}", e);
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
        log::info!(
            "Shutting down service actor for {} because no clients are left and the timeout period has elapsed.",
            &self.room_id
        );

        ctx.stop();
    }
}
