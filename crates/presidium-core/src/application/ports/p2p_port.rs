//! P2P network port — abstract interface for peer-to-peer networking.
//!
//! This port defines the contract for P2P network operations including
//! peer discovery, DHT operations, relay, connection management,
//! direct messaging, and topic-based pub/sub for group chats.
//! The primary implementation will use `rust-libp2p`.

use async_trait::async_trait;

use crate::domain::value_objects::{DeviceId, UserId};

/// Errors specific to P2P networking operations.
#[derive(Debug, thiserror::Error)]
pub enum P2PError {
    /// Failed to connect to a peer.
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// DHT lookup or store operation failed.
    #[error("DHT error: {0}")]
    DhtError(String),

    /// NAT traversal failed.
    #[error("NAT traversal failed: {0}")]
    NatTraversalFailed(String),

    /// No relay available for indirect connection.
    #[error("No relay available: {0}")]
    NoRelayAvailable(String),

    /// Peer not found in the DHT.
    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    /// Failed to send a P2P message to a specific device.
    #[error("Send failure: {0}")]
    SendFailure(String),

    /// Peer is not reachable at the moment.
    #[error("Peer unreachable: {0}")]
    PeerUnreachable(String),
}

/// Information about a peer on the P2P network.
#[derive(Debug, Clone)]
pub struct PeerInfo {
    /// The peer's unique identifier (PeerId in libp2p).
    pub peer_id: String,
    /// The user associated with this peer (if authenticated).
    pub user_id: Option<UserId>,
    /// The peer's reachable multiaddresses.
    pub addresses: Vec<String>,
    /// Whether this peer is currently connected.
    pub is_connected: bool,
}

/// Port for P2P network operations.
///
/// Implementations must provide:
/// - Kademlia DHT for peer and key bundle discovery
/// - Circuit Relay v2 for NAT traversal
/// - QUIC and WebRTC transport
/// - mDNS for local network discovery
/// - GossipSub for group chat message routing
#[async_trait]
pub trait P2PNetworkPort: Send + Sync {
    // ── Lifecycle ──────────────────────────────────────────

    /// Starts the P2P networking stack.
    async fn start(&self) -> Result<(), P2PError>;

    /// Stops the P2P networking stack gracefully.
    async fn stop(&self) -> Result<(), P2PError>;

    // ── Peer Discovery ─────────────────────────────────────

    /// Discovers peers on the network using the DHT.
    async fn discover_peer(&self, user_id: &UserId) -> Result<PeerInfo, P2PError>;

    /// Returns the list of currently connected peers.
    async fn connected_peers(&self) -> Result<Vec<PeerInfo>, P2PError>;

    /// Returns the local peer's information.
    async fn local_peer_info(&self) -> Result<PeerInfo, P2PError>;

    // ── DHT Operations ─────────────────────────────────────

    /// Stores a value in the DHT (e.g., pre-key bundle).
    async fn dht_put(&self, key: &[u8], value: &[u8]) -> Result<(), P2PError>;

    /// Retrieves a value from the DHT.
    async fn dht_get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, P2PError>;

    /// Publishes pre-key bundle data to the DHT for a specific user/device.
    ///
    /// This is a convenience wrapper around `dht_put` that constructs
    /// the appropriate DHT key from the user and device identifiers,
    /// enabling other devices to discover and establish E2EE sessions.
    async fn publish_pre_keys(
        &self,
        user_id: &UserId,
        device_id: &DeviceId,
        bundle: &[u8],
    ) -> Result<(), P2PError>;

    /// Fetches pre-key bundle data from the DHT for a specific user/device.
    ///
    /// This is a convenience wrapper around `dht_get` that looks up
    /// the pre-key bundle for a remote user's device, needed to
    /// establish an E2EE session via the X3DH/PQXDH protocol.
    async fn fetch_pre_keys(
        &self,
        user_id: &UserId,
        device_id: &DeviceId,
    ) -> Result<Vec<u8>, P2PError>;

    // ── Direct Messaging ───────────────────────────────────

    /// Sends an encrypted byte payload directly to a specific device.
    ///
    /// The `data` parameter contains already-encrypted ciphertext
    /// produced by `E2EECryptoPort::encrypt_message`. This method
    /// handles routing and delivery over the P2P network via QUIC
    /// or relayed connection.
    async fn send_p2p(&self, target_device: &DeviceId, data: Vec<u8>) -> Result<(), P2PError>;

    /// Receives the next available incoming P2P message (non-blocking).
    ///
    /// Returns `Ok(Some(data))` if a message is available, or
    /// `Ok(None)` if no messages are currently queued. The caller
    /// should poll this method or use an async stream in production.
    async fn receive_p2p(&self) -> Result<Option<Vec<u8>>, P2PError>;

    // ── Pub/Sub for Group Chats ────────────────────────────

    /// Subscribes to a GossipSub topic for group chat messaging.
    ///
    /// Topics are identified by chat ID strings. Once subscribed,
    /// messages published to this topic will be delivered via
    /// `receive_p2p`. This is a stub for the initial MVP and
    /// will be fully implemented in a subsequent development day.
    async fn subscribe_topic(&self, topic: &str) -> Result<(), P2PError>;
}
