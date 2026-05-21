//! LibSignal crypto adapter — stub implementation of `E2EECryptoPort`.
//!
//! This adapter will use `libsignal-protocol-rust` to implement
//! the full X3DH/PQXDH key agreement and Double Ratchet protocol.
//! Currently all methods return `todo!()` as implementation will be
//! completed in subsequent development days.

use async_trait::async_trait;
use presidium_core::application::ports::crypto_port::{CryptoError, E2EECryptoPort, PreKeyBundle};
use presidium_core::domain::value_objects::{DeviceId, SessionId, UserId};

/// Adapter implementing `E2EECryptoPort` using the Signal Protocol.
///
/// This struct will wrap `libsignal-protocol-rust` primitives:
/// - X3DH / PQXDH for key agreement
/// - Double Ratchet for forward-secure messaging
/// - Kyber KEM for post-quantum resistance
pub struct LibSignalCryptoAdapter {
    // Future fields:
    // identity_key_pair: IdentityKeyPair,
    // signed_pre_key: SignedPreKeyRecord,
    // sessions: HashMap<SessionId, SessionState>,
}

impl LibSignalCryptoAdapter {
    /// Creates a new `LibSignalCryptoAdapter`.
    ///
    /// In the full implementation, this will generate or load
    /// the identity key pair and signed pre-key from local storage.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LibSignalCryptoAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
#[allow(unused_variables)]
impl E2EECryptoPort for LibSignalCryptoAdapter {
    async fn create_pre_key_bundle(&self) -> Result<PreKeyBundle, CryptoError> {
        todo!("Day 5: Implement pre-key bundle generation using libsignal-protocol-rust")
    }

    async fn establish_session(
        &self,
        remote_user: &UserId,
        remote_device: &DeviceId,
        bundle: &PreKeyBundle,
    ) -> Result<SessionId, CryptoError> {
        todo!("Day 5: Implement X3DH/PQXDH session establishment")
    }

    async fn encrypt_message(
        &self,
        session_id: &SessionId,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        todo!("Day 5: Implement Double Ratchet encryption")
    }

    async fn decrypt_message(
        &self,
        session_id: &SessionId,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        todo!("Day 5: Implement Double Ratchet decryption")
    }

    async fn delete_session(&self, session_id: &SessionId) -> Result<(), CryptoError> {
        todo!("Day 5: Implement session deletion")
    }

    async fn list_sessions(&self) -> Result<Vec<SessionId>, CryptoError> {
        todo!("Day 5: Implement session listing")
    }

    async fn rotate_ratchet(&self, session_id: &SessionId) -> Result<(), CryptoError> {
        todo!("Day 5: Implement Double Ratchet rotation")
    }
}
