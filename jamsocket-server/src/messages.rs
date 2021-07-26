use actix::{Message, Recipient};
use jamsocket::MessageRecipient;

/// Represents a message or event initiated by a client.
#[derive(Debug, Clone)]
pub enum MessageFromClient {
    /// A client opens a connection to the server.
    Connect(u32, Recipient<MessageFromServer>),

    /// A client disconnects from the server (or their connection otherwise drops.)
    Disconnect(u32),

    /// A client sends a message.
    Message { from_user: u32, data: String },
}

impl Message for MessageFromClient {
    type Result = ();
}

#[derive(Debug, Clone)]
pub enum MessageData {
    String(String),
    Binary(Vec<u8>),
}

/// Represents a message sent to one or more clients from the server.
#[derive(Debug, Clone)]
pub struct MessageFromServer {
    pub to_user: MessageRecipient,
    pub data: MessageData,
}

impl Message for MessageFromServer {
    type Result = ();
}

impl MessageFromServer {
    pub fn new(to_user: MessageRecipient, data: String) -> Self {
        MessageFromServer { to_user, data: MessageData::String(data) }
    }

    pub fn new_binary(to_user: MessageRecipient, data: Vec<u8>) -> Self {
        MessageFromServer { to_user, data: MessageData::Binary(data) }
    }
}

/// Represents a request to reserve a user ID and return it. User IDs are
/// unique only in the context of a room.
///
/// Currently, user IDs are assigned sequentially, but this is an implementation
/// detail and should not be relied on.
pub struct AssignUserId;

impl Message for AssignUserId {
    type Result = u32;
}
