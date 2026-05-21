//! Crypto adapter implementations.
//!
//! Concrete implementations of crypto ports using libsignal-protocol-rust
//! and post-quantum libraries.

mod libsignal_crypto_adapter;

pub use libsignal_crypto_adapter::LibSignalCryptoAdapter;
