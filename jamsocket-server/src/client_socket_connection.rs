use crate::messages::{MessageData, MessageFromClient, MessageFromServer};
use actix::{Actor, Handler, Recipient, StreamHandler};
use actix_web_actors::ws;

/// Represents a connection from a service to a client, which consists of a
/// message receiver and a user ID.
pub struct ClientSocketConnection {
    pub room: Recipient<MessageFromClient>,
    pub user: u32,
}

impl Actor for ClientSocketConnection {
    type Context = ws::WebsocketContext<Self>;
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
                self.room
                    .do_send(MessageFromClient::Disconnect(self.user))
                    .unwrap();
            }
            _ => (),
        }
    }
}
