![Jamsocket Logo](jamsocket_logo.svg)

# Jamsocket

[![docs.rs](https://img.shields.io/badge/docs-latest-orange)](https://jamsocket.github.io/jamsocket/jamsocket/index.html)

Jamsocket is a lightweight framework for building applications that are served over
[WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API).

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
