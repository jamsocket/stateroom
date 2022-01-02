![Jamsocket Logo](jamsocket_logo.svg)

# Jamsocket

[![crates.io](https://img.shields.io/crates/v/jamsocket.svg)](https://crates.io/crates/jamsocket)
[![docs.rs](https://img.shields.io/badge/docs-release-brightgreen)](https://docs.rs/jamsocket/0.1.0/jamsocket/)
[![wokflow state](https://github.com/jamsocket/jamsocket/workflows/test/badge.svg)](https://github.com/drifting-in-space/jamsocket/actions/workflows/test.yml)

Jamsocket is a lightweight framework for building services that are accessed through
[WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) connections.

Services can either be native Rust code that runs in the server process, or be compiled into
[WebAssembly](https://webassembly.org/) modules and loaded dynamically.

## Usage

To create a Jamsocket service, implement the `SimpleJamsocketService` trait. There's only one function that you *must* implement, the constructor `new`.

Let's implement a simple shared counter. Any connected client will be able to increment or decrement it by sending 
`increment` or `decrement` messages (other messages will be ignored). Whenever the value is changed, we'll broadcast it 
to every connected client.

```rust
impl SimpleJamsocketService for SharedCounterServer {
    fn new(_: &str,
           _: &impl JamsocketContext) -> Self {
        SharedCounterServer(0)
    }

    fn message(&mut self, _: ClientId,
               message: &str,
               ctx: &impl JamsocketContext) {
        match message {
            "increment" => self.0 += 1,
            "decrement" => self.0 -= 1,
            _ => (),
        }

        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("new value: {}", self.0));
    }
}
```

To serve this service, we will compile it into a WebAssembly module. We import the `#[jamsocket_wasm]`
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
(under default server settings, any string is a vaild room ID and connecting to a non-existant
room will create it).

Now, you can increment the counter by sending the `increment` message using the `ws` handle:

```javascript
ws.send('increment')
```

If everything is set up correctly, the result will be printed out:

```
new value: 1
```

If multiple clients are connected, each one will receive this message. Just like that, we have a mechanism for sharing some (very basic) application state between clients.

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

- [`jamsocket`](https://docs.rs/jamsocket/) is the core, minimal implementation of the service interface.
- [`jamsocket-cli`](https://docs.rs/jamsocket-cli/) is a command-line interface for interacting with WebAssembly-compiled Jamsocket services.
- [`jamsocket-server`](https://docs.rs/jamsocket-server/) provides [Actix](https://actix.rs/) actors to facilitate serving Jamsocket services in a WebSocket server.
- [`jamsocket-wasm`](https://docs.rs/jamsocket-wasm/) provides a macro for generating WebAssembly modules from Jamsocket services.
- [`jamsocket-wasm-host`](https://docs.rs/jamsocket-wasm-host/) provides a way to import Jamsocket services from WebAssembly modules.

## See Also

[Aper](https://github.com/aper-dev/aper) is a state synchronization library which
works with Jamsocket. 
