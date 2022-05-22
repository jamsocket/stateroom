use crate::messages::{MessageFromClient, MessageFromServer};
use actix::{Actor, ActorContext, AsyncContext, Handler, Recipient, SpawnHandle, StreamHandler};
use actix_web_actors::ws;
use stateroom::{ClientId, MessagePayload};
use std::time::{Duration, Instant};

/// Represents a connection from a service to a client, which consists of a
/// message receiver and a user ID.
pub struct ClientSocketConnection {
    pub room: Recipient<MessageFromClient>,
    pub client_id: ClientId,
    pub last_seen: Instant,
    pub heartbeat_interval: Duration,
    pub heartbeat_timeout: Duration,
    pub interval_handle: Option<SpawnHandle>,
}

impl ClientSocketConnection {
    fn start_heartbeat_interval(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.interval_handle = Some(ctx.run_interval(self.heartbeat_interval, |act, ctx| {
            if Instant::now() - act.last_seen > act.heartbeat_timeout {
                tracing::warn!(
                    client_id=?act.client_id,
                    "Stopping ClientSocketConnection because heartbeat not responded.",
                );
                act.close(ctx);
            } else {
                ctx.ping(b"");
            }
        }));
    }

    fn close(&self, ctx: &mut ws::WebsocketContext<Self>) {
        self.interval_handle.map(|d| ctx.cancel_future(d));

        self.room
            .do_send(MessageFromClient::Disconnect(self.client_id));

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
            MessagePayload::Text(st) => ctx.text(st),
            MessagePayload::Bytes(bin) => ctx.binary(bin),
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
                    client: self.client_id,
                    data: MessagePayload::Text(text.to_string()),
                };
                self.room.do_send(message);
            }
            Ok(ws::Message::Binary(data)) => {
                let message = MessageFromClient::Message {
                    client: self.client_id,
                    data: MessagePayload::Bytes(data.to_vec()),
                };
                self.room.do_send(message);
            }
            Ok(ws::Message::Close(_)) => {
                tracing::info!(
                    client_id=?self.client_id,
                    "User has disconnected from room",
                );

                self.close(ctx);
            }
            Err(error) => tracing::error!(?error, "Encountered error in StreamHandler"),
            _ => tracing::warn!(message=?msg, "Unhandled message in StreamHandler"),
        }
    }
}
