# QTM

QTM is a local QuantPerm substrate runtime and observer CLI.

This project creates deterministic local substrates, commits their QuantPerm
state into QTM coordinate/commitment pairs, replays persisted transition
history, and exposes a small `qp qtm` command surface for creating, opening,
transitioning, and exiling substrate events.

## Scope

This repository is currently a QuantPerm/QTM state runtime, not a production
cryptocurrency wallet.

It does handle:

- Local substrate namespaces under `~/.qp/substrates/<name>`.
- Deterministic `Perm` and `QuantPerm` construction from indices and entropy.
- QTM commitment and coordinate generation.
- Transition persistence and replay.
- A projection layer for BIP44-style coin concepts such as Bitcoin, Ethereum,
  Tron, and Solana.

It does not yet handle:

- Standard HD private key or address derivation.
- xpub/xprv export.
- Blockchain transaction signing.
- RPC balance lookup.
- Transaction broadcast.
- Production-grade secret storage.

## Primary Dependency

The primary domain dependency is:

```toml
quantom_value = { git = "https://github.com/QuantPerm-Labs/Quantom-VALUE" }
```

`quantom_value` provides the core QuantPerm model used throughout this project,
including `Perm`, `QuantPerm`, `Heritage`, and transition state. The local
`qp-hd` code wraps that core with:

- CLI commands in `src/main.rs` and `src/commands.rs`.
- QTM commitment logic in `src/protocolvalue.rs`.
- QP44 projection logic in `src/qp44.rs`.
- Economic/proof gate logic in `src/economic_gate.rs`.
- Seed and BIP39 input handling in `src/purpose.rs`.

Other dependencies are supporting libraries for hashing, hex encoding, CLI
parsing, environment loading, serialization, async/network plumbing, and
cryptographic primitives.

## CLI

The binary is named `qp`.

```sh
cargo run -- qtm create <name>
cargo run -- qtm open <name>
cargo run -- qtm transit <name> --purpose 44 --coin 60 --account 0 --change 0 --external 0
cargo run -- qtm exile <name>
```

`qtm create` initializes a substrate. `qtm open` prints the current persisted
surface. `qtm transit` appends a transition event. `qtm exile` removes the
latest event from the substrate history.

## Configuration

Runtime transitions that request the protocol seed expect a `.env` value:

```env
PROTOCOL_SEED=<64 hex characters>
```

The seed must decode to exactly 32 bytes.
