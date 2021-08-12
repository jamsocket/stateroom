# jamsocket-server

This crate provides [Actix](https://actix.rs/) actors for serving a
Jamsocket service. It can be used both with native Rust Jamsocket services, or
(in conjunction with the `jamsocket-wasm-host` crate), for Jamsocket services
built into WebAssembly modules.

This crate does not provide a server binary, only actors. A server binary using
these actors is implemented in the `jamsocket-cli` crate.
