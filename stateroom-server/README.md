# stateroom-server

This crate provides [Actix](https://actix.rs/) actors for serving a
Stateroom service. It can be used both with native Rust Stateroom services, or
(in conjunction with the `stateroom-wasm-host` crate), for Stateroom services
built into WebAssembly modules.

This crate does not provide a server binary, only actors. A server binary using
these actors is implemented in the `stateroom-cli` crate.
