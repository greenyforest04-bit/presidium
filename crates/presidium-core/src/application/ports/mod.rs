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

pub mod crypto_port;
pub mod messaging_port;
pub mod moderation_port;
pub mod p2p_port;
pub mod storage_port;
