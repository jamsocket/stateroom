use actix::{Message, Recipient};
use stateroom::{ClientId, MessagePayload, MessageRecipient};

/// Represents a message or event initiated by a client.
#[derive(Debug, Clone)]
pub enum MessageFromClient {
    /// A client opens a connection to the server.
    Connect(ClientId, Recipient<MessageFromServer>),

    /// A client disconnects from the server (or their connection otherwise drops.)
    Disconnect(ClientId),

    /// A client sends a message.
    Message {
        client: ClientId,
        data: MessagePayload,
    },
}

impl Message for MessageFromClient {
    type Result = ();
}

/// Represents a message sent to one or more clients from the server.
#[derive(Debug, Clone)]
pub struct MessageFromServer {
    pub to_client: MessageRecipient,
    pub data: MessagePayload,
}

impl Message for MessageFromServer {
    type Result = ();
}

impl MessageFromServer {
    #[must_use]
    pub fn new(to_client: MessageRecipient, data: String) -> Self {
        MessageFromServer {
            to_client,
            data: MessagePayload::Text(data),
        }
    }

    #[must_use]
    pub fn new_binary(to_client: MessageRecipient, data: Vec<u8>) -> Self {
        MessageFromServer {
            to_client,
            data: MessagePayload::Bytes(data),
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
