![Jamsocket Logo](jamsocket_logo.svg)

# Jamsocket

[![docs.rs](https://img.shields.io/badge/docs-latest-orange)](https://jamsocket.github.io/jamsocket/jamsocket/index.html)
[![wokflow state](https://github.com/jamsocket/jamsocket/workflows/test/badge.svg)](https://github.com/jamsocket/jamsocket/actions/workflows/test.yml)
[![wokflow state](https://github.com/jamsocket/jamsocket/workflows/docs/badge.svg)](https://github.com/jamsocket/jamsocket/actions/workflows/docs.yml)

Jamsocket is a lightweight framework for building services that are accessed through
[WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) connections.

Services can either be native Rust code that runs in the server process, or be compiled into
[WebAssembly](https://webassembly.org/) modules and loaded dynamically.

## Concepts and Terminology

A Jamsocket server hosts one or more **services**. Services determine how the server responds to various types of
messages.

External connections into a Jamsocket server are called **clients**. Usually, clients have a 1:1 correspondence with
*users*, so the term *user* is (informally) interchangable with *client*.

When clients connect to a Jamsocket server, they connect to a particular **room**. Each room has a 1:1 correspondence with an instance of your service. This means that all service state is scoped to a room. Additionally, client identifiers are scoped to a room, and when a service broadcasts a message it is delivered only to clients in that service instance's room. You can think of rooms as being isolated and independent of each other, analogous to rooms in a chat service.

## Usage

The simplest way to get started with Jamsocket is to implement the `SimpleJamsocketService` trait. There's only one function that you *must* implement, the constructor `new`. This method is passed in a string representing a unique identifier for the room being constructed (as mentioned above, an instance is constructed for every room). It is also passed a reference to a `JamsocketContext`, which we can ignore for now.

Let's implement a simple shared counter. Any connected client will be able to increment or decrement it by sending 
`increment` or `decrement` messages (other messages will be ignored). Whenever the value is changed, we'll broadcast it 
to every connected client.

```rust
use jamsocket::*;

struct SharedCounterServer(i32);

impl SimpleJamsocketService for SharedCounterServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        SharedCounterServer(0)
    }
}
```

This service will compile, but it doesn't actually live up to its name, because it doesn't echo anything. It will just silently ignore incoming messages.

To fix this, implement the `message` function:

```rust
impl SimpleJamsocketService for SharedCounterServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        SharedCounterServer(0)
    }

    fn message(&mut self, _: ClientId, message: &str, ctx: &impl JamsocketContext) {
        match message {
            "increment" => self.0 += 1,
            "decrement" => self.0 -= 1,
            _ => (),
        }

        ctx.send_message(MessageRecipient::Broadcast, &format!("new value: {}", self.0));
    }
}
```

To serve this service, we will compile it to a WebAssembly module. We import the `#[jamsocket_wasm]`
annotation macro and apply it to the existing `SharedCounterServer` declaration.

```rust
use jamsocket_wasm::jamsocket_wasm;

#[jamsocket_wasm]
struct SharedCounterServer(i32);
```

Then, install the `jamsocket` command-line tool and the `wasm32-wasi` target, and run 
`jamsocket dev`:

```bash
$ cargo install jamsocket-cli
$ rustup target add wasm32-wasi
$ jamsocket dev
```

`jamsocket dev` will build your app and serve it on port `:8080`. Then, open
`http://localhost:8080/status` in your browser -- if all went well, you should see the
status message `ok`. Open up developer tools in your browser and type:

```javascript
let ws = new WebSocket('ws://localhost:8080/ws/1');
ws.onmessage = (c) => console.log(c.data);
```

This connects to your service, creating a new room with the id `1` if one doesn't exist
(with default server settings, any string is a vaild room ID and connecting to a non-existant
room will create it).

Now, you can increment the counter by sending the `increment` message using the `ws` handle:

```javascript
ws.send('increment')
```

If everything is set up correctly, the result will be printed out:

```
new value: 1
```

## Using without WebAssembly

If you don't want to compile your service to WebAssembly (for example, if you want to use 
capabilities that are
not exposed by [WASI](https://wasi.dev/)), you can use `jamsocket-server`.

```rust
use jamsocket_server::*;

fn main() -> std::io::Result<()> {
    serve::<SharedCounterServer>()?;

    Ok(())
}
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
