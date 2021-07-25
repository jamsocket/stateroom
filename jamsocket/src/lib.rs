//! Jamsocket is a minimalist framework for developing stateful [WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) applications.
//!
//! Jamsocket apps implement the `JamsocketService` trait, which provides a way for the
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
//!     /// The server's only state is a mapping of user ID to username.
//!     user_to_nickname: HashMap<u32, String>,
//! }
//!
//! impl SimpleJamsocketService for ChatServer {
//!     /// This is called when a user connects.
//!     fn connect(&mut self, user: u32, ctx: &impl JamsocketContext) {
//!         let username = format!("user{}", user);
//!
//!         // Send a welcome message.
//!         ctx.send_message(user,
//!             &format!("Welcome to the chat! Your name is {}. Send /nick <username> to change it.",
//!                     &username));
//!
//!         // Alert all other connected users to the new user.
//!         ctx.send_message(MessageRecipient::Broadcast,
//!             &format!("{} has joined the chat", &username));
//!     }
//!
//!     /// This is called when a user disconnects.
//!     fn disconnect(&mut self, user: u32, ctx: &impl JamsocketContext) {
//!         let username = self.user_to_nickname.remove(&user).unwrap();
//!
//!         // Alert all remaining users that a user has left.
//!         ctx.send_message(MessageRecipient::Broadcast,
//!            &format!("{} has left the chat", &username));
//!     }
//!
//!     /// This is called when a user sends a message.
//!     fn message(&mut self, user: u32, message: &str, ctx: &impl JamsocketContext) {
//!         if let Some(new_nick) = message.strip_prefix("/nick ") {
//!             // This message is a /nick command, so process accordingly.
//!             let old_nick = self.user_to_nickname.insert(user, new_nick.to_string()).unwrap();
//!             ctx.send_message(MessageRecipient::Broadcast,
//!                &format!("{} is now known as {}", old_nick, new_nick));
//!         } else {
//!             // Broadcast the message to all connected users, prefixed by the username.
//!             let username = self.user_to_nickname.get(&user).unwrap();
//!             ctx.send_message(MessageRecipient::Broadcast,
//!                &format!("{}: {}", username, message));
//!         }
//!     }
//! }

pub use message_recipient::MessageRecipient;

mod message_recipient;

/// Provides an interface for a [JamsocketService] instance to send messages back to its host environment.
pub trait JamsocketContext {
    /// Sends a message to a currently connected user, or broadcast a message to all users.
    ///
    /// Recipient can be a `u32` representing an individual user to send a message to, or
    /// `MessageRecipient::Broadcast` to broadcast a message to all connected users.
    /// The message is a string which is sent verbatim to the user(s) indicated.
    fn send_message(&self, recipient: impl Into<MessageRecipient>, message: &str);

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
pub trait SimpleJamsocketService: Default {
    fn initialize(&mut self, token: &str, context: &impl JamsocketContext) {}

    fn connect(&mut self, user: u32, context: &impl JamsocketContext) {}

    fn disconnect(&mut self, user: u32, context: &impl JamsocketContext) {}

    fn message(&mut self, user: u32, message: &str, context: &impl JamsocketContext) {}

    fn timer(&mut self, context: &impl JamsocketContext) {}
}

/// The main interface to a Jamsocket service.
///
/// If the service wishes to send messages *back* to the calling environment, it is expected to own
/// or borrow a [JamsocketContext] object, but the details are left up to the implementer. See
/// [WrappedJamsocketService] for an example.
#[allow(unused_variables)]
pub trait JamsocketService {
    fn connect(&mut self, user: u32);

    fn disconnect(&mut self, user: u32);

    fn message(&mut self, user: u32, message: &str);

    fn timer(&mut self);
}

/// Enables an object to become a [JamsocketService] of the associated `Service` type.
pub trait JamsocketServiceBuilder<C: JamsocketContext> {
    /// The type of [JamsocketService] that the object implementing this trait becomes.
    type Service: JamsocketService;

    /// Transform `self` into a [JamsocketService].
    fn build(self, token: &str, context: C) -> Self::Service;
}

/// Combines a [SimpleJamsocketService] with an owned [JamsocketContext] in order to implement
/// [JamsocketService].
pub struct WrappedJamsocketService<S: SimpleJamsocketService, C: JamsocketContext> {
    context: C,
    model: S,
}

impl<T: SimpleJamsocketService, C: JamsocketContext> JamsocketServiceBuilder<C> for T {
    type Service = WrappedJamsocketService<T, C>;

    fn build(mut self, token: &str, context: C) -> Self::Service {
        self.initialize(token, &context);

        WrappedJamsocketService {
            context,
            model: self,
        }
    }
}

impl<T: SimpleJamsocketService, C: JamsocketContext> JamsocketService
    for WrappedJamsocketService<T, C>
{
    fn connect(&mut self, user: u32) {
        self.model.connect(user, &self.context);
    }

    fn disconnect(&mut self, user: u32) {
        self.model.disconnect(user, &self.context);
    }

    fn message(&mut self, user: u32, message: &str) {
        self.model.message(user, message, &self.context);
    }

    fn timer(&mut self) {
        self.model.timer(&self.context);
    }
}