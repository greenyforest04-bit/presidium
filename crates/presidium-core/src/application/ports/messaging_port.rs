//! Messaging transport port — abstract interface for message delivery.
//!
//! This port defines the contract for sending and receiving encrypted
//! messages over the P2P network. The primary implementation will use
//! `libp2p` with GossipSub and direct QUIC streams.

use async_trait::async_trait;

use crate::domain::value_objects::UserId;

/// Errors specific to messaging transport operations.
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    /// The recipient is not reachable on the network.
    #[error("Recipient unreachable: {0}")]
    RecipientUnreachable(String),

    /// The message could not be delivered within the timeout.
    #[error("Delivery timeout: {0}")]
    Timeout(String),

    /// A network error occurred.
    #[error("Network error: {0}")]
    Network(String),

    /// The message was rejected by the remote peer.
    #[error("Message rejected: {0}")]
    Rejected(String),
}

/// Port for message transport over the P2P network.
///
/// Implementations must provide reliable, encrypted message delivery
/// between devices. The actual encryption is handled by `E2EECryptoPort`;
/// this port handles the transport layer.
#[async_trait]
pub trait MessageTransportPort: Send + Sync {
    /// Sends an encrypted message to a specific recipient.
    ///
    /// The `ciphertext` is already encrypted by the `E2EECryptoPort`.
    /// This method handles routing and delivery over the P2P network.
    async fn send_message(
        &self,
        recipient_id: &UserId,
        ciphertext: &[u8],
    ) -> Result<(), TransportError>;

    /// Receives the next available encrypted message.
    ///
    /// Returns the raw ciphertext, which must be decrypted using
    /// the `E2EECryptoPort`.
    async fn receive_message(&self) -> Result<Vec<u8>, TransportError>;

    /// Checks if a recipient is currently reachable on the network.
    async fn is_reachable(&self, user_id: &UserId) -> Result<bool, TransportError>;

    /// Broadcasts a message to all connected peers (for group chats).
    async fn broadcast(&self, ciphertext: &[u8]) -> Result<(), TransportError>;
}
