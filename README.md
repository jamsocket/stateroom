![Jamsocket Logo](jamsocket_logo.svg)

# Jamsocket

Jamsocket is a lightweight framework for building applications that are served over
[WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API).

## Modules

Jamsocket has a modular architecture. If all you want to do is generate a Jamsocket service to
be served with an existing Jamsocket WebAssembly server, the main crates you will interact with
will probably be [`jamsocket-cli`](/jamsocket-cli), which provides a command-line tool, and
[`jamsocket-wasm`](/jamsocket-wasm), the main Cargo dependency for building services.

- [`jamsocket`](/jamsocket) is the core, minimal implementation of the service interface.
- [`jamsocket-cli`](/jamsocket-cli) is a command-line interface for interacting with WebAssembly-compiled Jamsocket services.
- [`jamsocket-server`](/jamsocket-server) provides [Actix](https://actix.rs/) actors to facilitate serving Jamsocket services in a WebSocket server.
- [`jamsocket-wasm`](/jamsocket-wasm) provides a macro for generating WebAssembly modules from Jamsocket services.
- [`jamsocket-wasm-host`](/jamsocket-wasm-host) provides a way to import Jamsocket services from WebAssembly modules.