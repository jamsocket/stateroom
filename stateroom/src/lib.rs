//! Stateroom is a minimalist framework for developing stateful
//! [WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) applications.
//!
//! Stateroom apps implement the [SimpleStateroomService] trait, which provides a way for the
//! app to hook into events when clients connect, disconnect, and send messages. Additionally,
//! Stateroom provides a simple mechanism for invoking events in the future with a timer.
//!
//! A simple chat server looks like this:
//!
//! ```
//! use stateroom::*;
//! use std::collections::HashMap;
//!
//! #[derive(Default)]
//! struct ChatServer {
//!     /// The server's only state is a mapping of client ID to username.
//!     client_to_nickname: HashMap<ClientId, String>,
//! }
//!
//! impl SimpleStateroomService for ChatServer {
//!     fn new(room_id: &str, _: &impl StateroomContext) -> Self {
//!         Default::default()
//!     }
//!
//!     /// This is called when a user connects.
//!     fn connect(&mut self, client: ClientId, ctx: &impl StateroomContext) {
//!         let username = format!("client{}", u32::from(client));
//!
//!         // Send a welcome message.
//!         ctx.send_message(client,
//!             &format!("Welcome to the chat! Your name is {}. \
//!                      Send /nick <username> to change it.",
//!                      &username));
//!
//!         // Alert all other connected users to the new user.
//!         ctx.send_message(MessageRecipient::Broadcast,
//!             &format!("{} has joined the chat", &username));
//!     }
//!
//!     /// This is called when a user disconnects.
//!     fn disconnect(&mut self, client: ClientId, ctx: &impl StateroomContext) {
//!         let username = self.client_to_nickname.remove(&client).unwrap();
//!
//!         // Alert all remaining users that a user has left.
//!         ctx.send_message(MessageRecipient::Broadcast,
//!            &format!("{} has left the chat", &username));
//!     }
//!
//!     /// This is called when a user sends a message.
//!     fn message(&mut self, client: ClientId, message: &str, ctx: &impl StateroomContext) {
//!         if let Some(new_nick) = message.strip_prefix("/nick ") {
//!             // This message is a /nick command, so process accordingly.
//!             let old_nick = self.client_to_nickname.insert(client, new_nick.to_string()).unwrap();
//!             ctx.send_message(MessageRecipient::Broadcast,
//!                &format!("{} is now known as {}", old_nick, new_nick));
//!         } else {
//!             // Broadcast the message to all connected users, prefixed by the username.
//!             let username = self.client_to_nickname.get(&client).unwrap();
//!             ctx.send_message(MessageRecipient::Broadcast,
//!                &format!("{}: {}", username, message));
//!         }
//!     }
//! }

pub use client_id::ClientId;
pub use message_recipient::MessageRecipient;
pub use messages::{MessageFromProcess, MessagePayload, MessageToProcess};
use std::convert::Infallible;

mod client_id;
mod message_recipient;
mod messages;

/// Provides an interface for a [StateroomService] instance to send messages back to its host environment.
pub trait StateroomContext: Unpin + 'static + Send + Sync {
    /// Sends a message to a currently connected user, or broadcast a message to all users.
    ///
    /// Recipient can be a `u32` representing an individual user to send a message to, or
    /// `MessageRecipient::Broadcast` to broadcast a message to all connected users.
    /// The message is a string which is sent verbatim to the user(s) indicated.
    fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str);

    /// Sends a binary message to a currently connected user, or broadcast a message to all users.
    ///
    /// See [StateroomContext::send_message] for details on the semantics of `recipient`.
    fn send_binary(&self, recipient: impl Into<MessageRecipient>, message: &[u8]);

    /// Sets a timer to wake up the service in the given number of milliseconds by invoking `timer()`.
    ///
    /// Each instance of a service can only have one (or zero) timer outstanding at any time; if this
    /// is called before an existing timer expires, the previous timer is replaced. This provides a
    /// very basic primitive that more complex timer behavior can be built on, using state and logic
    /// stored in your service. For example, you could implement multiple concurrent timers using a
    /// priority queue and ensuring that the environment timer always reflects the head of the queue.
    fn set_timer(&self, ms_delay: u32);
}

/// A simplified interface for creating a [StateroomService] that can be exposed as a WebAssembly module.
///
/// See module documentation for usage examples.
#[allow(unused_variables)]
pub trait SimpleStateroomService: Unpin + Send + Sync + 'static {
    /// Called when the service is created, before any client has had a chance to connect.
    fn new(room_id: &str, context: &impl StateroomContext) -> Self;

    /// Called each time a client connects to the service.
    fn connect(&mut self, client: ClientId, context: &impl StateroomContext) {}

    /// Called each time a client disconnects from the service, unless that disconnection
    /// will cause the service to be destroyed.
    fn disconnect(&mut self, client: ClientId, context: &impl StateroomContext) {}

    /// Called each time a client sends a text message to the service.
    fn message(&mut self, client: ClientId, message: &str, context: &impl StateroomContext) {}

    /// Called each time a client sends a binary message to the service.
    fn binary(&mut self, client: ClientId, message: &[u8], context: &impl StateroomContext) {}

    /// Called when [StateroomContext::set_timer] has been called on this service's context,
    /// after the provided duration.
    fn timer(&mut self, context: &impl StateroomContext) {}
}

/// The host interface to a Stateroom service. Implementations should instead implement the trait
/// [SimpleStateroomService].
#[allow(unused_variables)]
pub trait StateroomService: Send + Sync + Unpin + 'static {
    /// Called each time a client connects to the service.
    fn connect(&mut self, client: ClientId);

    /// Called each time a client disconnects from the service, unless that disconnection
    /// will cause the service to be destroyed.
    fn disconnect(&mut self, client: ClientId);

    /// Called each time a client sends a text message to the service.
    fn message(&mut self, client: ClientId, message: &str);

    /// Called each time a client sends a binary message to the service.
    fn binary(&mut self, client: ClientId, message: &[u8]);

    /// Called when [StateroomContext::set_timer] has been called on this service's context,
    /// after the provided duration.
    fn timer(&mut self);
}

/// Enables an object to become a [StateroomService] of the associated `Service` type.
pub trait StateroomServiceFactory<C: StateroomContext>: Send + Sync + 'static {
    /// The type of [StateroomService] that the object implementing this trait builds.
    type Service: StateroomService;
    type Error: std::fmt::Debug;

    /// Non-destructively build a [StateroomService] from `self`.
    fn build(&self, room_id: &str, context: C) -> Result<Self::Service, Self::Error>;
}

impl<C: StateroomContext, S: SimpleStateroomService + Clone> StateroomServiceFactory<C> for S {
    type Service = WrappedStateroomService<S, C>;
    type Error = Infallible;

    fn build(&self, _room_id: &str, context: C) -> Result<Self::Service, Self::Error> {
        Ok(WrappedStateroomService::new(self.clone(), context))
    }
}

/// Combines a [SimpleStateroomService] with an owned [StateroomContext] in order to implement
/// [StateroomService].
pub struct WrappedStateroomService<S: SimpleStateroomService, C: StateroomContext> {
    service: S,
    context: C,
}

impl<S: SimpleStateroomService, C: StateroomContext> WrappedStateroomService<S, C> {
    pub fn new(service: S, context: C) -> Self {
        WrappedStateroomService { service, context }
    }
}

impl<S: SimpleStateroomService, C: StateroomContext> StateroomService
    for WrappedStateroomService<S, C>
{
    fn connect(&mut self, client: ClientId) {
        self.service.connect(client, &self.context);
    }

    fn disconnect(&mut self, client: ClientId) {
        self.service.disconnect(client, &self.context);
    }

    fn message(&mut self, client: ClientId, message: &str) {
        self.service.message(client, message, &self.context);
    }

    fn timer(&mut self) {
        self.service.timer(&self.context);
    }

    fn binary(&mut self, client: ClientId, message: &[u8]) {
        self.service.binary(client, message, &self.context);
    }
}
