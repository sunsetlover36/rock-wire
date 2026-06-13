# rock-wire

Shared wire types for [ROCK](https://github.com/sunsetlover36/rock), a programmable runtime for Lua-scripted multiplayer worlds.

`rock-wire` contains the serializable Rust types used at the network boundary: world snapshots, entity component payloads, input actions, signals, auth/session data, and Farcaster webhook/API shapes.

The crate is intentionally small and boring. It exists so ROCK servers, clients, tools, and experiments can agree on the same JSON protocol without copying type definitions around.

## Usage

```toml
[dependencies]
rock-wire = "0.1"
```

## What's Inside

- `WorldSnapshot`, `RoomSnapshot`, and entity update payloads
- incoming client requests for input and signals
- outgoing world, signal, and system packets
- shared component structs such as position, sprites, ownership, and rotation
- auth-related wire types
- Farcaster response and webhook payload types

## Relationship to ROCK

ROCK is the runtime. `rock-wire` is only the shared protocol surface.

If you are looking for the server runtime, documentation, examples, or Lua gamemode API, start with [sunsetlover36/rock](https://github.com/sunsetlover36/rock).
