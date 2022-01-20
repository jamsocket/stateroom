# `stateroom-wasm`

`stateroom-wasm` is a companion crate to `stateroom` that helps you package and
export services as WebAssembly modules.

## `#[stateroom_wasm]` macro

WebAssembly modules must import and export certain named functions in order for the
Stateroom server to understand them. `stateroom_wasm` provides the `#[stateroom_wasm]` module, 
which should be applied to an item (`struct`, `enum`, or `type` alias) that implements 
`SimpleStateroomService`.

A Stateroom-compatible WebAssembly module must contain exactly one service. If you
need to generate multiple services for your application, currently the best approach
is to make a crate for each service.

It's possible to generate bindings for a service that belongs to another module using
a `type` alias:

```rust
use stateroom_wasm::stateroom_wasm;
use some_module::SomeService;

#[stateroom_wasm]
type Service = SomeService;
```

## Execution model

Upon initialization, the generated model creates an instance of your `SimpleStateroomService`
by calling its `new(room_id, context)` constructor. The context object that is passed in the
constructor and subsequent function calls is a global static object that binds to functions
imported from the host environment (like `send_message`).

## Compiling

If you are using the Stateroom command-line interface, `stateroom dev` will build the
current crate using the `wasm32-wasi` target, and then load and serve the generated
WebAssembly module.

If you would like to build it manually, make sure you have the `wasm32-wasi` target installed
and pass it as a target to `cargo build`:

```bash
$ rustup target add wasm32-wasi
$ cargo build --release --target=wasm32-wasi
```

## Embedding

This crate has a counterpart, `stateroom-wasm-host`, which can take a module generated with
this crate and expose it through a StateroomService interface.
