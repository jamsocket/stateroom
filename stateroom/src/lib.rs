#![doc = include_str!("../README.md")]

use async_trait::async_trait;
pub use client_id::ClientId;
pub use message_recipient::MessageRecipient;
pub use messages::{MessageFromRoom, MessagePayload, RoomEvent};

mod client_id;
mod message_recipient;
mod messages;

/// Provides an interface for a [StateroomService] instance to send and receive messages from its host environment.
#[async_trait]
pub trait StateroomContext: Unpin + Send {
    async fn next_event(&mut self) -> RoomEvent;

    /// Sends a message to a currently connected user, or broadcast a message to all users.
    ///
    /// Recipient can be a `u32` representing an individual user to send a message to, or
    /// `MessageRecipient::Broadcast` to broadcast a message to all connected users.
    /// The message is a string which is sent verbatim to the user(s) indicated.
    fn send(&self, recipient: impl Into<MessageRecipient>, message: &str);

    /// Sends a binary message to a currently connected user, or broadcast a message to all users.
    ///
    /// See [StateroomContext::send_message] for details on the semantics of `recipient`.
    fn send_binary(&self, recipient: impl Into<MessageRecipient>, message: &[u8]);
}

#[async_trait]
pub trait Stateroom: Unpin + 'static {
    async fn run<C: StateroomContext>(self, ctx: C) -> ();
}
