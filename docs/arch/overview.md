# Presidium Messenger — Architecture Overview

## System Architecture

Presidium Messenger is a **decentralized, end-to-end encrypted P2P messenger**
built with a Rust core and Kotlin Multiplatform mobile client.

### High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Mobile Client                      │
│        Kotlin Multiplatform + Jetpack Compose        │
│                    (UniFFI)                          │
├─────────────────────────────────────────────────────┤
│                  Rust Core (Workspace)                │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐            │
│  │  Crypto   │ │   P2P    │ │   LLM    │            │
│  │  (E2EE)   │ │ (libp2p) │ │ (On-dev) │            │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘            │
│       │            │            │                    │
│  ┌────┴────────────┴────────────┴─────┐             │
│  │         Presidium Core             │             │
│  │    (Domain + Application Ports)    │             │
│  └────────────────────────────────────┘             │
│  ┌──────────┐ ┌──────────┐                          │
│  │ Storage  │ │Messaging │                          │
│  │(SQLCipher)│ │ (Domain) │                          │
│  └──────────┘ └──────────┘                          │
├─────────────────────────────────────────────────────┤
│                    P2P Network                       │
│     Kademlia DHT · GossipSub · Circuit Relay         │
│          QUIC · WebRTC · mDNS                        │
└─────────────────────────────────────────────────────┘
```

### Key Design Decisions

1. **Hexagonal Architecture** — each crate has domain/application/infrastructure layers
2. **Ports & Adapters** — business logic depends on trait interfaces, not concrete implementations
3. **Domain-Driven Design** — aggregates, entities, value objects, domain events
4. **E2EE First** — all messages encrypted via Signal Protocol (PQXDH + Double Ratchet)
5. **Offline-First** — the system works without internet, syncing when connectivity returns
6. **On-Device LLM** — moderation and AI assistant run locally, no data leaves the device
