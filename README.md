# Stateroom

[![crates.io](https://img.shields.io/crates/v/stateroom.svg)](https://crates.io/crates/stateroom)
[![docs.rs](https://img.shields.io/badge/docs-release-brightgreen)](https://docs.rs/stateroom/0.1.0/stateroom/)
[![wokflow state](https://github.com/drifting-in-space/stateroom/actions/workflows/test.yml/badge.svg)](https://github.com/drifting-in-space/stateroom/actions/workflows/test.yml)

Stateroom is a minimalist framework for building lightweight, single-threaded services that send and
receive messages through [WebSockets](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API).

Services can either be native Rust code that runs in the server process, or be compiled into
[WebAssembly](https://webassembly.org/) modules and loaded dynamically.

## Usage

To create a Stateroom service, implement the `SimpleStateroomService` trait. There's only one function that you *must* implement, the constructor `new`.

Let's implement a simple shared counter. Any connected client will be able to increment or decrement it by sending 
`increment` or `decrement` messages (other messages will be ignored). Whenever the value is changed, we'll broadcast it 
to every connected client.

```rust
use stateroom_wasm::*;

#[stateroom_wasm]
#[derive(Default)]
struct EchoServer;

impl StateroomService for EchoServer {
    fn connect(&mut self, client_id: ClientId, ctx: &impl StateroomContext) {
        ctx.send_message(client_id, format!("User {:?} connected.", client_id));
    }

    fn message(&mut self, client_id: ClientId, message: MessagePayload, ctx: &impl StateroomContext) {
        let Some(message) = message.text() else {
            return;
        };

        ctx.send_message(
            MessageRecipient::Broadcast,
            format!("User {:?} sent '{}'", client_id, message),
        );
    }

    fn disconnect(&mut self, client_id: ClientId, ctx: &impl StateroomContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            format!("User {:?} left.", client_id),
        );
    }
}
```

To serve this service, we will compile it into a WebAssembly module. We import the `#[stateroom_wasm]`
annotation macro and apply it to the existing `SharedCounter` declaration.

```rust
use stateroom_wasm::*;

#[stateroom_wasm]
#[derive(Default)]
struct SharedCounter(i32);

impl StateroomService for SharedCounter {}
```

Then, install the `stateroom` command-line tool and the `wasm32-wasi` target, and run 
`stateroom dev`:

```bash
$ cargo install stateroom-cli
$ rustup target add wasm32-wasi
$ stateroom dev
```

`stateroom dev` will build your app and serve it on port `:8080`. Then, open
`http://localhost:8080/status` in your browser -- if all went well, you should see the
status message `ok`. Open up developer tools in your browser and type:

```javascript
let ws = new WebSocket('ws://localhost:8080/ws');
ws.onmessage = (c) => console.log(c.data);
```

This connects to your service, creating a new room with the id `1` if one doesn't exist
(under default server settings, any string is a vaild room ID and connecting to a non-existant
room will create it).

Now, you can increment the counter by sending the `increment` message using the `ws` handle:

```javascript
ws.send('increment')
```

If everything is set up correctly, the result will be printed out:

```text
new value: 1
```

If multiple clients are connected, each one will receive this message. Just like that, we have a mechanism for sharing some (very basic) application state between clients.

## Modules

Stateroom has a modular architecture. If all you want to do is generate a Stateroom service to
be served with an existing Stateroom WebAssembly server, the main crates you will interact with
will probably be [`stateroom-cli`](/stateroom-cli), which provides a command-line tool, and
[`stateroom-wasm`](/stateroom-wasm), the main Cargo dependency for building services.

- [`stateroom`](https://docs.rs/stateroom/) is the core, minimal implementation of the service interface.
- [`stateroom-cli`](https://docs.rs/stateroom-cli/) is a command-line interface for interacting with WebAssembly-compiled Stateroom services.
- [`stateroom-server`](https://docs.rs/stateroom-server/) provides an [Axum](https://github.com/tokio-rs/axum)-based WebSocket server that runs a Stateroom service.
- [`stateroom-wasm`](https://docs.rs/stateroom-wasm/) provides a macro for generating WebAssembly modules from Stateroom services.
- [`stateroom-wasm-host`](https://docs.rs/stateroom-wasm-host/) provides a way to import Stateroom services from WebAssembly modules.

## See Also

[Aper](https://github.com/aper-dev/aper) is a state synchronization library which
works with Stateroom. 
