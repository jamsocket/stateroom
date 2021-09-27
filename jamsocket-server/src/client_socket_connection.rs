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
                tracing::warn_span!(
                    "Stopping ClientSocketConnection because heartbeat not responded.",
                    client_id=?act.client_id,
                    ip=%act.ip,
                    room_id=%act.room_id,
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
            tracing::warn_span!(
                "Could not send Disconnect message before closing room",
                room_id=%self.room_id
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
                    tracing::warn_span!(
                        "Error forwarding message to service",
                        room_id=%self.room_id
                    );
                }
            }
            Ok(ws::Message::Binary(data)) => {
                let message = MessageFromClient::Message {
                    from_client: self.client_id,
                    data: MessageData::Binary(data.to_vec()),
                };
                if self.room.do_send(message).is_err() {
                    tracing::warn_span!(
                        "Error forwarding binary message to service",
                        room_id=%self.room_id
                    );
                }
            }
            Ok(ws::Message::Close(_)) => {
                tracing::info_span!(
                    "User has disconnected from room",
                    client_id=?self.client_id,
                    ip=%self.ip,
                    room_id=%self.room_id
                );

                self.close(ctx);
            }
            Err(e) => {
                tracing::error_span!("Encountered error in StreamHandler", error=?e);
            }
            _ => {
                tracing::warn_span!("Unhandled message in StreamHandler", message=?msg);
            }
        }
    }
}
