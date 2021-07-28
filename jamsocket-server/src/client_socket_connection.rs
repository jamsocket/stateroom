use crate::messages::{MessageData, MessageFromClient, MessageFromServer};
use actix::{Actor, ActorContext, AsyncContext, Handler, Recipient, StreamHandler};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

/// Represents a connection from a service to a client, which consists of a
/// message receiver and a user ID.
pub struct ClientSocketConnection {
    pub room: Recipient<MessageFromClient>,
    pub user: u32,
    pub room_id: String,
    pub ip: String,
    pub last_seen: Instant,
    pub heartbeat_interval: Duration,
    pub heartbeat_timeout: Duration,
}

impl ClientSocketConnection {
    fn check_if_dropped(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(self.heartbeat_interval, |act, ctx| {
            if Instant::now() - act.last_seen > act.heartbeat_timeout {
                log::warn!(
                    "Stopping ClientSocketConnection {} (IP: {}) from room {} \
                    because heartbeat not responded.",
                    act.user,
                    act.ip,
                    act.room_id,
                );
                ctx.stop();
            } else {
                ctx.ping(b"");
            }
        });
    }
}

impl Actor for ClientSocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Started");
        self.check_if_dropped(ctx);
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
                    from_user: self.user,
                    data: MessageData::String(text.to_string()),
                };
                self.room.do_send(message).unwrap();
            }
            Ok(ws::Message::Binary(data)) => {
                let message = MessageFromClient::Message {
                    from_user: self.user,
                    data: MessageData::Binary(data.to_vec()),
                };
                self.room.do_send(message).unwrap();
            }
            Ok(ws::Message::Close(_)) => {
                log::info!(
                    "User {} (IP: {}) has disconnected from room {}",
                    self.user,
                    &self.ip,
                    &self.room_id
                );

                self.room
                    .do_send(MessageFromClient::Disconnect(self.user))
                    .unwrap();
            }
            Err(e) => log::error!("Encountered error in StreamHandler: {:?}", &e),
            _ => log::warn!("Encountered unhandled message in StreamHandler: {:?}", &msg),
        }
    }
}
