//! Port definitions — trait interfaces for infrastructure adapters.
//!
//! Ports define the boundaries of the hexagonal architecture. Each port
//! is a trait that the application layer uses to interact with external
//! systems (databases, network, crypto, etc.). Infrastructure adapters
//! implement these traits.
//!
//! ## Naming Convention
//! - **Inbound ports** (driven by external actors): `XxxUseCase` trait
//! - **Outbound ports** (driving external systems): `XxxPort` trait
//!
//! All port traits must be `Send + Sync` for async safety.
//!
//! ## Port Inventory (Day 2)
//!
//! | Port | Crate | Purpose |
//! |------|-------|---------|
//! | `E2EECryptoPort` | presidium-crypto | E2EE encryption (Signal + PQ) |
//! | `P2PNetworkPort` | presidium-p2p | P2P networking (libp2p) |
//! | `StoragePort` | presidium-storage | High-level entity storage |
//! | `MessageStoragePort` | presidium-storage | Low-level ciphertext storage |
//! | `MessageTransportPort` | presidium-messaging | Message delivery transport |
//! | `ModerationPort` | presidium-llm | Content moderation + Sarcophagus |
//! | `LLMPort` | presidium-llm | On-device LLM inference |

pub mod crypto_port;
pub mod llm_port;
pub mod messaging_port;
pub mod moderation_port;
pub mod p2p_port;
pub mod storage_port;
