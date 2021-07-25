![Jamsocket Logo](jamsocket_logo.svg)

# Jamsocket

Jamsocket is a lightweight framework for building applications that are served over
[WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API).

## Modules

Jamsocket has a modular architecture:

- [`jamsocket`](/jamsocket) is the core, minimal implementation of the service interface.
- [`jamsocket-wasm`](/jamsocket-wasm) provides facilities both for generating WebAssembly modules from Jamsocket services, and for embedding those services in an application.