use crate::messages::{MessageData, MessageFromClient, MessageFromServer};
use actix::{Actor, ActorContext, AsyncContext, Handler, Recipient, SpawnHandle, StreamHandler};
use actix_web_actors::ws;
use jamsocket::ClientId;
use std::time::{Duration, Instant};

/// Represents a connection from a service to a client, which consists of a
/// message receiver and a user ID.
pub struct ClientSocketConnection {
    pub room: Recipient<MessageFromClient>,
    pub client_id: ClientId,
    pub room_id: String,
    pub ip: String,
    pub last_seen: Instant,
    pub heartbeat_interval: Duration,
    pub heartbeat_timeout: Duration,
    pub interval_handle: Option<SpawnHandle>,
}

impl ClientSocketConnection {
    fn start_heartbeat_interval(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.interval_handle = Some(ctx.run_interval(self.heartbeat_interval, |act, ctx| {
            if Instant::now() - act.last_seen > act.heartbeat_timeout {
                log::warn!(
                    "Stopping ClientSocketConnection {:?} (IP: {}) from room {} \
                    because heartbeat not responded.",
                    act.client_id,
                    act.ip,
                    act.room_id,
                );
                act.close(ctx);
            } else {
                ctx.ping(b"");
            }
        }));
    }

    fn close(&self, ctx: &mut ws::WebsocketContext<Self>) {
        self.interval_handle.map(|d| ctx.cancel_future(d));

        if self
            .room
            .do_send(MessageFromClient::Disconnect(self.client_id))
            .is_err()
        {
            log::warn!(
                "Could not send Disconnect message before closing room {}",
                self.room_id
            );
        }

        ctx.stop();
    }
}

impl Actor for ClientSocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat_interval(ctx);
    }
}

impl Handler<MessageFromServer> for ClientSocketConnection {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer, ctx: &mut Self::Context) {
        match msg.data {
            MessageData::String(st) => ctx.text(st),
            MessageData::Binary(bin) => ctx.binary(bin),
        };
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => self.last_seen = Instant::now(),
            Ok(ws::Message::Text(text)) => {
                let message = MessageFromClient::Message {
                    from_client: self.client_id,
                    data: MessageData::String(text.to_string()),
                };
                if self.room.do_send(message).is_err() {
                    log::warn!("Error forwarding message to service in room {}", self.room_id);
                }
            }
            Ok(ws::Message::Binary(data)) => {
                let message = MessageFromClient::Message {
                    from_client: self.client_id,
                    data: MessageData::Binary(data.to_vec()),
                };
                if self.room.do_send(message).is_err() {
                    log::warn!("Error forwarding binary message to service in room {}", self.room_id);
                }
            }
            Ok(ws::Message::Close(_)) => {
                log::info!(
                    "User {:?} (IP: {}) has disconnected from room {}",
                    self.client_id,
                    &self.ip,
                    &self.room_id
                );

                self.close(ctx);
            }
            Err(e) => log::error!("Encountered error in StreamHandler: {:?}", &e),
            _ => log::warn!("Encountered unhandled message in StreamHandler: {:?}", &msg),
        }
    }
}
