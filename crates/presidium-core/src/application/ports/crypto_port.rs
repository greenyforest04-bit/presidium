//! E2EE Crypto port — abstract interface for cryptographic operations.
//!
//! This port defines the contract for all end-to-end encryption operations.
//! The primary implementation will use `libsignal-protocol-rust` with
//! PQXDH (post-quantum extended triple Diffie-Hellman) and Double Ratchet.
//!
//! ## Security Guarantees
//! - Forward secrecy: compromise of one message key does not compromise
//!   previous message keys.
//! - Post-compromise security: recovery from key compromise after a
//!   ratchet step.
//! - Post-quantum resistance: PQXDH adds Kyber KEM to the initial
//!   key exchange.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::errors::DomainError;
use crate::domain::value_objects::{DeviceId, SessionId, UserId};

/// Errors specific to cryptographic operations.
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    /// No session found for the given identifier.
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    /// Decryption failed — the ciphertext could not be decrypted.
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// Encryption failed — the plaintext could not be encrypted.
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    /// Key generation failed.
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),

    /// Invalid key bundle — the pre-key bundle is malformed or expired.
    #[error("Invalid pre-key bundle: {0}")]
    InvalidPreKeyBundle(String),

    /// A domain-level error occurred during a crypto operation.
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
}

/// A pre-key bundle for initial session establishment (X3DH/PQXDH).
///
/// This bundle contains the public keys needed to establish a new
/// E2EE session with a remote device. It is uploaded to a distribution
/// server (or shared via P2P DHT).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreKeyBundle {
    /// The device this bundle belongs to.
    pub device_id: DeviceId,
    /// The identity key (Ed25519 → X25519).
    pub identity_key: Vec<u8>,
    /// The signed pre-key (X25519).
    pub signed_pre_key: Vec<u8>,
    /// Signature of the signed pre-key by the identity key.
    pub signed_pre_key_signature: Vec<u8>,
    /// One-time pre-key (X25519, optional but recommended).
    pub one_time_pre_key: Option<Vec<u8>>,
    /// Post-quantum Kyber KEM ciphertext (PQXDH extension).
    pub pq_kem_ciphertext: Option<Vec<u8>>,
}

/// Port for E2EE cryptographic operations.
///
/// Implementations must provide:
/// - X3DH/PQXDH key agreement for session establishment
/// - Double Ratchet for message encryption/decryption
/// - Forward secrecy and post-compromise security
#[async_trait]
pub trait E2EECryptoPort: Send + Sync {
    /// Generates a new pre-key bundle for this device.
    ///
    /// This bundle should be uploaded to the key distribution mechanism
    /// (P2P DHT or bootstrap server) so that other devices can initiate
    /// sessions with this device.
    async fn create_pre_key_bundle(&self) -> Result<PreKeyBundle, CryptoError>;

    /// Establishes a new E2EE session with a remote device using its
    /// pre-key bundle (X3DH/PQXDH key agreement).
    ///
    /// Returns a `SessionId` that can be used for subsequent
    /// encryption/decryption operations.
    async fn establish_session(
        &self,
        remote_user: &UserId,
        remote_device: &DeviceId,
        bundle: &PreKeyBundle,
    ) -> Result<SessionId, CryptoError>;

    /// Encrypts a plaintext message for the given session.
    ///
    /// The Double Ratchet advances the sending chain, providing
    /// forward secrecy for each message.
    async fn encrypt_message(
        &self,
        session_id: &SessionId,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, CryptoError>;

    /// Decrypts a ciphertext message for the given session.
    ///
    /// The Double Ratchet advances the receiving chain. If the message
    /// is from a new sending chain, a DH ratchet step is performed.
    async fn decrypt_message(
        &self,
        session_id: &SessionId,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, CryptoError>;

    /// Deletes an E2EE session (e.g., when a device is removed).
    async fn delete_session(&self, session_id: &SessionId) -> Result<(), CryptoError>;

    /// Lists all active session identifiers for the local device.
    async fn list_sessions(&self) -> Result<Vec<SessionId>, CryptoError>;

    /// Rotates the Double Ratchet for the given session.
    ///
    /// This should be called periodically or after a configurable number
    /// of messages to ensure post-compromise security. A ratchet rotation
    /// performs a DH step that creates a new chain key, making previously
    /// compromised keys unusable for decrypting future messages.
    async fn rotate_ratchet(&self, session_id: &SessionId) -> Result<(), CryptoError>;
}
