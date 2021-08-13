use actix::{Message, Recipient};
use jamsocket::{ClientId, MessageRecipient};

/// Represents a message or event initiated by a client.
#[derive(Debug, Clone)]
pub enum MessageFromClient {
    /// A client opens a connection to the server.
    Connect(ClientId, Recipient<MessageFromServer>),

    /// A client disconnects from the server (or their connection otherwise drops.)
    Disconnect(ClientId),

    /// A client sends a message.
    Message {
        from_client: ClientId,
        data: MessageData,
    },
}

impl Message for MessageFromClient {
    type Result = ();
}

/// Message received or to be sent over a WebSocket connection, which may be
/// textual or binary.
#[derive(Debug, Clone)]
pub enum MessageData {
    String(String),
    Binary(Vec<u8>),
}

/// Represents a message sent to one or more clients from the server.
#[derive(Debug, Clone)]
pub struct MessageFromServer {
    pub to_client: MessageRecipient,
    pub data: MessageData,
}

impl Message for MessageFromServer {
    type Result = ();
}

impl MessageFromServer {
    #[must_use]
    pub fn new(to_client: MessageRecipient, data: String) -> Self {
        MessageFromServer {
            to_client,
            data: MessageData::String(data),
        }
    }

    #[must_use]
    pub fn new_binary(to_client: MessageRecipient, data: Vec<u8>) -> Self {
        MessageFromServer {
            to_client,
            data: MessageData::Binary(data),
        }
    }
}

/// Represents a request to reserve a client ID and return it. Client IDs are
/// unique only in the context of a room.
///
/// Currently, client IDs are assigned sequentially, but this is an implementation
/// detail and should not be relied on.
pub struct AssignClientId;

impl Message for AssignClientId {
    type Result = ClientId;
}
