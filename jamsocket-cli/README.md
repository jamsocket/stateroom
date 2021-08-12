# jamsocket-cli

This module implements a command-line interface for building and serving Jamsocket
services.

## Installation

```bash
cargo install jamsocket-cli
```

## Commands

*The `jamsocket` command is new even relative to the other parts of Jamsocket, so
expect this list to grow over time and commands may be renamed or combined over time.*

### `jamsocket dev`

By default, the command `dev` will:
- Build the current module as a `wasm32-wasi` target.
- Locate the wasm output.
- Run a local server that exposes it on port 8080.

It can also be configured using a `jamsocket.toml` file to serve static files
and build a client-side WebAssembly module. See [`cli_opts.rs`](src/cli_opts.rs)
for 

### `jamsocket serve`

The command `serve [path/to/service.wasm]` will set up a server for an existing 
WebAssembly file.
