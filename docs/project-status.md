# Presidium Messenger — Project Status

## Day 1: Project Initialization — COMPLETE

**Date:** 2026-05-22

### Completed Tasks

- [x] Cargo workspace initialized with 7 crates
- [x] Hexagonal skeleton created for all crates (domain/application/infrastructure)
- [x] Domain value objects: `UserId`, `DeviceId`, `SessionId`, `ChatId`, `MessageContent`, `ChatType`
- [x] Domain entities: `User`, `Device`, `Message`, `Chat`
- [x] Domain aggregates: `Conversation` (with message sending and event emission)
- [x] Domain events: `UserRegistered`, `SessionEstablished`, `MessageSent`, `ModerationViolationDetected`
- [x] Domain errors: `DomainError` with typed variants
- [x] Application ports: `E2EECryptoPort`, `MessageTransportPort`, `P2PNetworkPort`, `StoragePort`, `ModerationPort`
- [x] Application use case: `SendMessageUseCase` with mock testing
- [x] Application configuration: `AppConfig` with figment integration
- [x] Observability: `init_tracing()` with dev/production modes
- [x] Quality tools: `rustfmt.toml`, `clippy.toml`, `deny.toml`
- [x] CI/CD: GitHub Actions workflow (check, test, audit, deny, docs)
- [x] Development scripts: `setup.sh`, `ci-checks.sh`
- [x] Architecture Decision Record: ADR 001
- [x] Integration tests for architecture validation
- [x] Unit tests for all domain components

---

## Day 2: Hexagonal Architecture (Ports & Adapters) Implementation — COMPLETE

**Date:** 2026-05-22

### Completed Tasks

- [x] Added value objects: `MessageId(Uuid)`, `Timestamp(i64)` to presidium-core
- [x] Enriched `E2EECryptoPort` with `rotate_ratchet()` method
- [x] Enriched `P2PNetworkPort` with `send_p2p`, `receive_p2p`, `publish_pre_keys`, `fetch_pre_keys`, `subscribe_topic`
- [x] Added `MessageStoragePort` with `StoredMessage` struct for low-level ciphertext storage
- [x] Added `LLMPort` with `Quantization`, `InferenceConfig` for on-device LLM inference
- [x] Enriched `ModerationPort` with `check_message()`, `create_sarcophagus()`, `ContentVerdict`
- [x] Unified `ModerationCategory` — single definition in `domain::events`, referenced by moderation port
- [x] Created adapter stub: `LibSignalCryptoAdapter` in presidium-crypto
- [x] Created adapter stub: `Libp2pNetworkAdapter` in presidium-p2p
- [x] Created adapter stub: `RedbStorageAdapter` in presidium-storage (implements both StoragePort + MessageStoragePort)
- [x] Created adapter stubs: `CandleLlmAdapter` + `LocalModerationAdapter` in presidium-llm
- [x] Added `layers_dependencies.rs` integration test (10 tests)
- [x] Created ADR 0002: Hexagonal Ports Definition
- [x] Updated README.md with architecture section
- [x] All 59 tests passing (46 unit + 10 architecture + 3 doc)

### Architecture Summary

| Port | Crate | Adapter Stub | Future Implementation |
|------|-------|-------------|----------------------|
| `E2EECryptoPort` | presidium-crypto | `LibSignalCryptoAdapter` | libsignal-protocol-rust + PQXDH |
| `P2PNetworkPort` | presidium-p2p | `Libp2pNetworkAdapter` | rust-libp2p (Kademlia, GossipSub, QUIC) |
| `StoragePort` | presidium-storage | `RedbStorageAdapter` | redb (encrypted key-value store) |
| `MessageStoragePort` | presidium-storage | `RedbStorageAdapter` | redb |
| `MessageTransportPort` | presidium-messaging | — (uses P2PNetworkPort) | via P2PNetworkPort |
| `ModerationPort` | presidium-llm | `LocalModerationAdapter` | Uses LLMPort internally |
| `LLMPort` | presidium-llm | `CandleLlmAdapter` | candle.rs / llama-cpp-rs with GGUF |

### Key Design Decisions

- Ports defined in `presidium-core`, re-exported by satellite crates
- Two-level storage: `StoragePort` (domain entities) + `MessageStoragePort` (raw ciphertext)
- `LLMPort` separated from `ModerationPort` for reuse by future AI assistant
- Sarcophagus mechanism in `ModerationPort` creates encrypted violation reports
- `ModerationCategory` unified: single definition in `domain::events`
