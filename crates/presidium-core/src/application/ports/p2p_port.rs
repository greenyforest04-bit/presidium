//! P2P network port — abstract interface for peer-to-peer networking.
//!
//! This port defines the contract for P2P network operations including
//! peer discovery, DHT operations, relay, and connection management.
//! The primary implementation will use `rust-libp2p`.

use async_trait::async_trait;

use crate::domain::value_objects::UserId;

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
#[async_trait]
pub trait P2PNetworkPort: Send + Sync {
    /// Starts the P2P networking stack.
    async fn start(&self) -> Result<(), P2PError>;

    /// Stops the P2P networking stack gracefully.
    async fn stop(&self) -> Result<(), P2PError>;

    /// Discovers peers on the network using the DHT.
    async fn discover_peer(&self, user_id: &UserId) -> Result<PeerInfo, P2PError>;

    /// Stores a value in the DHT (e.g., pre-key bundle).
    async fn dht_put(&self, key: &[u8], value: &[u8]) -> Result<(), P2PError>;

    /// Retrieves a value from the DHT.
    async fn dht_get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, P2PError>;

    /// Returns the list of currently connected peers.
    async fn connected_peers(&self) -> Result<Vec<PeerInfo>, P2PError>;

    /// Returns the local peer's information.
    async fn local_peer_info(&self) -> Result<PeerInfo, P2PError>;
}
