//! Jamsocket is a minimalist framework for developing stateful [WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) applications.
//!
//! Jamsocket apps implement the [SimpleJamsocketService] trait, which provides a way for the
//! app to hook into events when clients connect, disconnect, and send messages. Additionally,
//! Jamsocket provides a simple mechanism for invoking events in the future with a timer.
//!
//! A simple chat server looks like this:
//!
//! ```
//! use jamsocket::*;
//! use std::collections::HashMap;
//!
//! #[derive(Default)]
//! struct ChatServer {
//!     /// The server's only state is a mapping of client ID to username.
//!     client_to_nickname: HashMap<ClientId, String>,
//! }
//!
//! impl SimpleJamsocketService for ChatServer {
//!     fn new(room_id: &str, _: &impl JamsocketContext) -> Self {
//!         Default::default()
//!     }
//!
//!     /// This is called when a user connects.
//!     fn connect(&mut self, client: ClientId, ctx: &impl JamsocketContext) {
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
//!     fn disconnect(&mut self, client: ClientId, ctx: &impl JamsocketContext) {
//!         let username = self.client_to_nickname.remove(&client).unwrap();
//!
//!         // Alert all remaining users that a user has left.
//!         ctx.send_message(MessageRecipient::Broadcast,
//!            &format!("{} has left the chat", &username));
//!     }
//!
//!     /// This is called when a user sends a message.
//!     fn message(&mut self, client: ClientId, message: &str, ctx: &impl JamsocketContext) {
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
use std::marker::PhantomData;

mod client_id;
mod message_recipient;
mod messages;

/// Provides an interface for a [JamsocketService] instance to send messages back to its host environment.
pub trait JamsocketContext: Unpin + 'static + Send + Sync {
    /// Sends a message to a currently connected user, or broadcast a message to all users.
    ///
    /// Recipient can be a `u32` representing an individual user to send a message to, or
    /// `MessageRecipient::Broadcast` to broadcast a message to all connected users.
    /// The message is a string which is sent verbatim to the user(s) indicated.
    fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str);

    /// Sends a binary message to a currently connected user, or broadcast a message to all users.
    ///
    /// See [JamsocketContext::send_message] for details on the semantics of `recipient`.
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

/// A simplified interface for creating a [JamsocketService] that can be exposed as a WebAssembly module.
///
/// See module documentation for usage examples.
#[allow(unused_variables)]
pub trait SimpleJamsocketService: Unpin + Send + Sync + 'static {
    /// Called when the service is created, before any client has had a chance to connect.
    fn new(room_id: &str, context: &impl JamsocketContext) -> Self;

    /// Called each time a client connects to the service.
    fn connect(&mut self, client: ClientId, context: &impl JamsocketContext) {}

    /// Called each time a client disconnects from the service, unless that disconnection
    /// will cause the service to be destroyed.
    fn disconnect(&mut self, client: ClientId, context: &impl JamsocketContext) {}

    /// Called each time a client sends a text message to the service.
    fn message(&mut self, client: ClientId, message: &str, context: &impl JamsocketContext) {}

    /// Called each time a client sends a binary message to the service.
    fn binary(&mut self, client: ClientId, message: &[u8], context: &impl JamsocketContext) {}

    /// Called when [JamsocketContext::set_timer] has been called on this service's context,
    /// after the provided duration.
    fn timer(&mut self, context: &impl JamsocketContext) {}
}

/// The host interface to a Jamsocket service. Implementations should instead implement the trait
/// [SimpleJamsocketService].
#[allow(unused_variables)]
pub trait JamsocketService: Send + Sync + Unpin + 'static {
    /// Called each time a client connects to the service.
    fn connect(&mut self, client: ClientId);

    /// Called each time a client disconnects from the service, unless that disconnection
    /// will cause the service to be destroyed.
    fn disconnect(&mut self, client: ClientId);

    /// Called each time a client sends a text message to the service.
    fn message(&mut self, client: ClientId, message: &str);

    /// Called each time a client sends a binary message to the service.
    fn binary(&mut self, client: ClientId, message: &[u8]);

    /// Called when [JamsocketContext::set_timer] has been called on this service's context,
    /// after the provided duration.
    fn timer(&mut self);
}

/// Enables an object to become a [JamsocketService] of the associated `Service` type.
pub trait JamsocketServiceFactory<C: JamsocketContext>: Send + Sync + 'static {
    /// The type of [JamsocketService] that the object implementing this trait builds.
    type Service: JamsocketService;

    /// Non-destructively build a [JamsocketService] from `self`.
    fn build(&self, room_id: &str, context: C) -> Option<Self::Service>;
}

/// A [JamsocketServiceFactory] that passes through `build()` arguments directly to
/// the associated [SimpleJamsocketService]'s `new()` constructor, and wraps the
/// result in a [WrappedJamsocketService].
pub struct SimpleJamsocketServiceFactory<S: SimpleJamsocketService, C: JamsocketContext> {
    _c: PhantomData<C>,
    _s: PhantomData<S>,
}

impl<S: SimpleJamsocketService, C: JamsocketContext> Default
    for SimpleJamsocketServiceFactory<S, C>
{
    fn default() -> Self {
        SimpleJamsocketServiceFactory {
            _c: PhantomData::default(),
            _s: PhantomData::default(),
        }
    }
}

impl<S: SimpleJamsocketService, C: JamsocketContext> JamsocketServiceFactory<C>
    for SimpleJamsocketServiceFactory<S, C>
{
    type Service = WrappedJamsocketService<S, C>;

    fn build(&self, room_id: &str, context: C) -> Option<Self::Service> {
        Some(WrappedJamsocketService {
            service: S::new(room_id, &context),
            context,
        })
    }
}

/// Combines a [SimpleJamsocketService] with an owned [JamsocketContext] in order to implement
/// [JamsocketService].
pub struct WrappedJamsocketService<S: SimpleJamsocketService, C: JamsocketContext> {
    service: S,
    context: C,
}

impl<S: SimpleJamsocketService, C: JamsocketContext> WrappedJamsocketService<S, C> {
    pub fn new(service: S, context: C) -> Self {
        WrappedJamsocketService { service, context }
    }
}

impl<S: SimpleJamsocketService, C: JamsocketContext> JamsocketService
    for WrappedJamsocketService<S, C>
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
