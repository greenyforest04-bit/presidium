# Presidium Messenger

> Decentralized E2EE P2P messenger of the next generation — where every new device strengthens the network.

## Overview

Presidium Messenger is a fully decentralized, end-to-end encrypted peer-to-peer messenger built with a Rust core and Kotlin Multiplatform mobile client. It combines military-grade cryptography with on-device AI to deliver a private, censorship-resistant communication platform.

### Key Features

- **E2EE**: Signal Protocol (PQXDH + Double Ratchet) with post-quantum resistance
- **P2P**: libp2p (Kademlia DHT, GossipSub, Circuit Relay v2, QUIC, WebRTC)
- **On-Device LLM**: Local moderation and AI assistant (Gemma-2B / Phi-3)
- **Offline-First**: Works without internet, syncs when connectivity returns
- **Network Amplification**: Every new device strengthens the P2P network

## Architecture

The project follows **Hexagonal Architecture** (Ports & Adapters) and **Domain-Driven Design** principles within a Cargo workspace monorepo.

### Workspace Structure

```
presidium/
├── crates/
│   ├── presidium-core/       # Hexagonal core: domain, application ports
│   ├── presidium-crypto/     # E2EE crypto ports & adapters
│   ├── presidium-p2p/        # P2P networking ports & adapters
│   ├── presidium-storage/    # Storage ports & adapters
│   ├── presidium-llm/        # On-device LLM ports & adapters
│   ├── presidium-messaging/  # Messaging domain
│   └── presidium-bridge/     # UniFFI mobile bridge
├── docs/
│   ├── arch/                 # Architecture documentation
│   └── adr/                  # Architecture Decision Records
├── scripts/                  # Development scripts
└── .github/workflows/        # CI/CD pipeline
```

### Hexagonal Architecture per Crate

Each crate follows the same internal structure:

```
src/
├── domain/           # Pure business logic (ZERO infrastructure dependencies)
│   ├── entities/     # Objects with identity
│   ├── value_objects/ # Immutable, equality-by-value
│   ├── aggregates/   # Consistency boundaries
│   └── events/       # Domain events
├── application/      # Use cases and port definitions
│   ├── use_cases/    # Interactors (business orchestration)
│   └── ports/        # Trait interfaces for adapters
└── infrastructure/   # Adapter implementations
    ├── adapters/     # External system integrations
    └── repositories/ # Data persistence
```

### Dependency Direction

```
domain ← application ← infrastructure
```

- `domain` MUST NOT depend on `application` or `infrastructure`
- `application` depends on `domain` only
- `infrastructure` implements `application` ports and uses `domain` types

### Port Inventory

| Port | Crate | Adapter | Purpose |
|------|-------|---------|---------|
| `E2EECryptoPort` | presidium-crypto | `LibSignalCryptoAdapter` | X3DH/PQXDH key agreement, Double Ratchet encrypt/decrypt |
| `P2PNetworkPort` | presidium-p2p | `Libp2pNetworkAdapter` | Kademlia DHT, GossipSub, QUIC, Circuit Relay v2 |
| `StoragePort` | presidium-storage | `RedbStorageAdapter` | High-level entity persistence (messages, chats) |
| `MessageStoragePort` | presidium-storage | `RedbStorageAdapter` | Low-level ciphertext blob storage |
| `MessageTransportPort` | presidium-messaging | — | Message delivery over P2P network |
| `ModerationPort` | presidium-llm | `LocalModerationAdapter` | On-device content moderation + Sarcophagus |
| `LLMPort` | presidium-llm | `CandleLlmAdapter` | GGUF model inference (candle.rs / llama-cpp-rs) |

### Inter-Crate Dependencies

```
presidium-core ← presidium-crypto
presidium-core ← presidium-p2p
presidium-core ← presidium-storage
presidium-core ← presidium-llm
presidium-core ← presidium-messaging
presidium-core ← presidium-bridge
```

No circular dependencies exist. All satellite crates depend only on `presidium-core`.

## Getting Started

### Prerequisites

- Rust 1.81+ (stable)
- cargo-audit, cargo-deny (optional, for CI checks)

### Setup

```bash
# Clone the repository
git clone https://github.com/presidium-messenger/presidium.git
cd presidium

# Run the setup script
./scripts/setup.sh
```

### Common Commands

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace --all-targets -- -D warnings

# Generate documentation
cargo doc --workspace --open

# Run CI checks locally
./scripts/ci-checks.sh
```

## Adding a New Crate

1. Create a new directory under `crates/` with the naming convention `presidium-<name>`
2. Add a `Cargo.toml` inheriting workspace settings
3. Create the `src/` structure: `domain/`, `application/`, `infrastructure/`
4. Add `presidium-core` as a dependency if needed
5. Update the workspace `Cargo.toml` if adding shared dependencies

## Architecture Decision Records

All significant architectural decisions are documented in `docs/adr/`:

- [ADR 001: Cargo Workspace with Hexagonal Architecture](docs/adr/0001-use-workspace-hexagonal.md)
- [ADR 002: Hexagonal Ports Definition](docs/adr/0002-hexagonal-ports-definition.md)

## License

AGPL-3.0-or-later
