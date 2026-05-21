# ADR 0002: Hexagonal Ports Definition

## Status

Accepted

## Date

2026-05-22

## Context

After establishing the Cargo workspace and hexagonal skeleton (ADR 0001), we need to define concrete **Ports** (trait interfaces) and stub **Adapters** for the key domains required by the MVP (1:1 E2EE chat). Ports define the boundaries of the hexagonal architecture — they are the only way the application layer interacts with external systems.

## Decision

We define **7 outbound ports** across the workspace, each with its own error type and supporting data structures:

| Port | Crate | Purpose | Key Methods |
|------|-------|---------|-------------|
| `E2EECryptoPort` | presidium-crypto | End-to-end encryption | create_pre_key_bundle, establish_session, encrypt_message, decrypt_message, rotate_ratchet |
| `P2PNetworkPort` | presidium-p2p | Peer-to-peer networking | start, stop, discover_peer, dht_put/get, send_p2p, receive_p2p, publish_pre_keys, fetch_pre_keys, subscribe_topic |
| `StoragePort` | presidium-storage | High-level entity persistence | store_message, get_messages, store_chat, get_chat, list_chats, delete_chat |
| `MessageStoragePort` | presidium-storage | Low-level ciphertext storage | save_outgoing_message, save_incoming_message, mark_delivered, mark_read, get_messages_for_user |
| `MessageTransportPort` | presidium-messaging | Message delivery | send_message, receive_message, is_reachable, broadcast |
| `ModerationPort` | presidium-llm | Content moderation + Sarcophagus | analyze_content, check_message, create_sarcophagus, load_model, unload_model, is_model_ready |
| `LLMPort` | presidium-llm | On-device LLM inference | load_model, unload_model, is_loaded, infer, infer_with_config |

Additionally, we define:

- **Value objects**: `MessageId(Uuid)`, `Timestamp(i64)` — for type-safe identifiers and timestamps
- **Error types**: Each port has its own error enum with `thiserror` for structured error handling
- **Data structures**: `PreKeyBundle`, `StoredMessage`, `PeerInfo`, `ContentVerdict`, `InferenceConfig`, `Quantization`
- **Unified `ModerationCategory`**: Defined once in `domain::events`, referenced by `ModerationPort` to avoid duplication

### Adapter Stubs

For each port, we create a stub adapter that implements the trait with `todo!()` methods:

| Adapter | Implements Port | Future Implementation |
|---------|----------------|----------------------|
| `LibSignalCryptoAdapter` | E2EECryptoPort | libsignal-protocol-rust + PQXDH |
| `Libp2pNetworkAdapter` | P2PNetworkPort | rust-libp2p (Kademlia, GossipSub, QUIC) |
| `RedbStorageAdapter` | StoragePort + MessageStoragePort | redb (encrypted key-value store) |
| `CandleLlmAdapter` | LLMPort | candle.rs / llama-cpp-rs with GGUF |
| `LocalModerationAdapter` | ModerationPort | Uses LLMPort internally for classification |

### Key Design Choices

1. **Ports live in `presidium-core`**: All port trait definitions are in `presidium-core/src/application/ports/` so they can be shared across crates without circular dependencies. Satellite crates re-export them.

2. **Two storage ports**: `StoragePort` works with domain entities (high-level), while `MessageStoragePort` works with raw ciphertext blobs (low-level). This separation allows efficient storage of encrypted messages before decryption.

3. **LLMPort is separate from ModerationPort**: LLM inference is a general capability used by both moderation and the future AI assistant. `ModerationPort` uses `LLMPort` internally.

4. **Sarcophagus in ModerationPort**: The `create_sarcophagus` method creates encrypted violation reports that only law enforcement bootstrap nodes can decrypt, preserving E2EE for all other participants.

5. **P2P port includes DHT convenience methods**: `publish_pre_keys` and `fetch_pre_keys` wrap `dht_put`/`dht_get` for the common use case of key bundle distribution.

## Consequences

### Positive
- Clear separation between domain logic and infrastructure
- Each adapter can be replaced with a mock for testing (already demonstrated in `SendMessageUseCase`)
- No circular dependencies — all crates depend only on `presidium-core`
- Type-safe error handling with `thiserror` ensures no error is silently swallowed
- Future implementations can be developed independently per crate

### Negative
- Adding a new port method requires updating all adapters (including mocks)
- `todo!()` stubs will panic at runtime if called — must be replaced before production
- Re-exporting from `presidium-core` means satellite crates are tightly coupled to core's API

### Risks
- Port interfaces may need to evolve as implementations reveal missing methods
- The `MessageStoragePort` vs `StoragePort` split may cause confusion — needs clear documentation
