//! # Presidium Crypto
//!
//! E2EE cryptographic ports and adapters for Presidium Messenger.
//!
//! This crate provides the interface and implementation for all
//! cryptographic operations including:
//! - **Signal Protocol**: X3DH/PQXDH key agreement
//! - **Double Ratchet**: Forward-secure message encryption
//! - **Post-Quantum**: Kyber KEM for quantum-resistant key exchange
//! - **Symmetric**: AES-GCM / ChaCha20-Poly1305 for message encryption
//!
//! ## Architecture
//!
//! Follows Hexagonal Architecture:
//! - `domain` — crypto-specific entities and value objects (keys, bundles)
//! - `application` — ports (traits) and use cases
//! - `infrastructure` — adapters (libsignal-protocol-rust, etc.)

pub mod application;
pub mod domain;
pub mod infrastructure;
