![Jamsocket Logo](jamsocket_logo.svg)

# Jamsocket

[![docs.rs](https://img.shields.io/badge/docs-latest-orange)](https://jamsocket.github.io/jamsocket/jamsocket/index.html)
[![wokflow state](https://github.com/jamsocket/jamsocket/workflows/test/badge.svg)](https://github.com/jamsocket/jamsocket/actions/workflows/test.yml)
[![wokflow state](https://github.com/jamsocket/jamsocket/workflows/docs/badge.svg)](https://github.com/jamsocket/jamsocket/actions/workflows/docs.yml)

Jamsocket is a lightweight framework for building applications that are served over
[WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API).

## Concepts

Jamsocket provides infrastructure for exposing a **service** to remote users over a WebSocket connection. Using Jamsocket consists of implementing a service, and then wiring Jamsocket up to serve it as desired.

Services are objects that can receive messages, perform arbitrary processing, and send messages. Jamsocket services differ from traditional stateless web endpoints in two ways:

1. They can retain state in memory between messages.
2. They may send a message to one (or more) users as the result of another user.

These differences enable real-time experiences that would be difficult to accomplish otherwise. To tame the complexity (both in a computational sense, and a developer ergonomics sense), Jamsocket has a concept of **rooms**. These are analogous to the way the term room is used in a chat application. In the context of Jamsocket, this means:

1. All service state is isolated to the room it is in. Each room represents an independent instance of the service.
2. An event (e.g. the service receiving a message) may only cause messages to be sent to a client in the same room as the client which sent it.

Every client connection is a connection to a particular room.

## Usage

The simplest way to get started with Jamsocket is to implement the `SimpleJamsocketService` trait. There's only one function that you *must* implement, the constructor `new`. This method is passed in a string representing a unique identifier for the room being constructed (as mentioned above, an instance is constructed for every room). It is also passed a reference to a `JamsocketContext`, which we can ignore for now.

```rust
use jamsocket::*;

struct EchoServer;

impl SimpleJamsocketService for EchoServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        EchoServer
    }
}
```

This service will compile, but it doesn't actually live up to its name, because it doesn't echo anything. It will just silently ignore incoming messages.

To fix this, implement the `message` function:

```rust
impl SimpleJamsocketService for EchoServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        EchoServer
    }

    fn message(&mut self, user: u32, message: &str, ctx: &impl JamsocketContext) {
        ctx.send_message(user, &format!("echo: {}", message));
    }
}
```

This time, we don't ignore the `JamsocketContext`: we need it to send back a message to the client.
The first parameter of `send_message` is the user that we are sending it to. In this case, we simply
return it to the client which sent it, which meets the [standard definition of echo](https://en.wikipedia.org/wiki/Echo_(computing)) in computing.

Alternatively, instead of sending it only to the client which sent the message, we could broadcast it to
all connected users in the same room. If you've ever yelled in a cave, this probably matches your intuition
of "echo" better than the implementation above. To do this, we simply replace `user` with 
`MessageRecipient::Broadcast`. 

```rust
fn message(&mut self, user: u32, message: &str, ctx: &impl JamsocketContext) {
    ctx.send_message(MessageRecipient::Broadcast, &format!("echo: {}", message));
}
```

It's also possible to send a message to an individual connected user other than the user who sent a message,
or to send messages to multiple users -- the only constraint is that you can only send messages to users in
the same room.

To actually connect to a `JamsocketService`, we need to serve it. We have two choices:

1. Create an embedded server in which our service runs directly in the server process.
2. Compile the service to WebAssembly and serve it that way.

To do #1, we can use the `serve` method of `jamsocket_server`.

```rust
use jamsocket_server::*;

fn main() -> std::io::Result<()> {
    serve::<EchoServer>()?;

    Ok(())
}
```

To do #2, we instead annotate the struct that implements `SimpleJamsocketService`, and
compile to the `wasm32-wasi` target.

```rust
use jamsocket_wasm::jamsocket_wasm;

#[jamsocket_wasm]
struct EchoServer;
```

*(TODO: document the build process)*

We can then use `jamsocket-cli` to serve it:

```bash
$ jamsocket-cli serve path/to/output.wasm
```

## Modules

Jamsocket has a modular architecture. If all you want to do is generate a Jamsocket service to
be served with an existing Jamsocket WebAssembly server, the main crates you will interact with
will probably be [`jamsocket-cli`](/jamsocket-cli), which provides a command-line tool, and
[`jamsocket-wasm`](/jamsocket-wasm), the main Cargo dependency for building services.

- [`jamsocket`](https://jamsocket.github.io/jamsocket/jamsocket/index.html) is the core, minimal implementation of the service interface.
- [`jamsocket-cli`](https://jamsocket.github.io/jamsocket/jamsocket_cli/index.html) is a command-line interface for interacting with WebAssembly-compiled Jamsocket services.
- [`jamsocket-server`](https://jamsocket.github.io/jamsocket/jamsocket_server/index.html) provides [Actix](https://actix.rs/) actors to facilitate serving Jamsocket services in a WebSocket server.
- [`jamsocket-wasm`](https://jamsocket.github.io/jamsocket/jamsocket_wasm/index.html) provides a macro for generating WebAssembly modules from Jamsocket services.
- [`jamsocket-wasm-host`](https://jamsocket.github.io/jamsocket/jamsocket_wasm_host/index.html) provides a way to import Jamsocket services from WebAssembly modules.
