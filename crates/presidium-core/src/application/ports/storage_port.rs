//! Storage port — abstract interface for persistent data storage.
//!
//! This port defines the contract for all persistent storage operations.
//! The primary implementation will use an encrypted local database
//! (SQLite with SQLCipher or redb).
//!
//! Two levels of abstraction are provided:
//! - `StoragePort` — high-level, works with domain entities (`Message`, `Chat`)
//! - `MessageStoragePort` — low-level, works with raw ciphertext for
//!   efficient storage of encrypted messages before decryption.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::entities::{Chat, Message};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{ChatId, MessageId, Timestamp, UserId};

/// Errors specific to storage operations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// The database is not available or corrupted.
    #[error("Database error: {0}")]
    Database(String),

    /// A record was not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// A concurrency conflict occurred (optimistic locking).
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Serialization or deserialization failed.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// A domain error occurred during a storage operation.
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
}

/// A raw stored message with ciphertext, used by `MessageStoragePort`.
///
/// This struct represents a message in its encrypted form as stored
/// on disk. The `ciphertext` field contains the E2EE-encrypted payload
/// that can only be decrypted by the intended recipient's device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    /// Unique message identifier.
    pub id: MessageId,
    /// The sender of the message.
    pub sender: UserId,
    /// The recipient of the message.
    pub recipient: UserId,
    /// The E2EE-encrypted message payload.
    pub ciphertext: Vec<u8>,
    /// Timestamp when the message was sent (Unix ms).
    pub timestamp: Timestamp,
    /// Whether the message has been delivered to the recipient.
    pub delivered: bool,
    /// Whether the message has been read by the recipient.
    pub read: bool,
}

/// Port for high-level persistent storage of messages and chats.
///
/// Implementations must provide:
/// - Encrypted storage at rest (SQLCipher or equivalent)
/// - Efficient querying by chat, sender, and time range
/// - Atomic transactions for consistency
#[async_trait]
pub trait StoragePort: Send + Sync {
    /// Stores a message persistently.
    async fn store_message(&self, message: &Message) -> Result<(), StorageError>;

    /// Retrieves messages for a chat, ordered by time.
    async fn get_messages(
        &self,
        chat_id: &ChatId,
        limit: usize,
        before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<Message>, StorageError>;

    /// Stores a chat.
    async fn store_chat(&self, chat: &Chat) -> Result<(), StorageError>;

    /// Retrieves a chat by its identifier.
    async fn get_chat(&self, chat_id: &ChatId) -> Result<Option<Chat>, StorageError>;

    /// Lists all chats for a user.
    async fn list_chats(&self, user_id: &UserId) -> Result<Vec<Chat>, StorageError>;

    /// Deletes a chat and all its messages.
    async fn delete_chat(&self, chat_id: &ChatId) -> Result<(), StorageError>;
}

/// Port for low-level encrypted message storage.
///
/// This port operates on raw ciphertext blobs, which is useful for
/// storing incoming E2EE messages before they are decrypted (e.g.,
/// when the user is offline or the session hasn't been established yet).
/// Implementations will typically use redb or a similar key-value store
/// for fast write-heavy workloads.
#[async_trait]
pub trait MessageStoragePort: Send + Sync {
    /// Saves an outgoing encrypted message.
    async fn save_outgoing_message(&self, msg: StoredMessage) -> Result<(), StorageError>;

    /// Saves an incoming encrypted message.
    async fn save_incoming_message(&self, msg: StoredMessage) -> Result<(), StorageError>;

    /// Marks a message as delivered.
    async fn mark_delivered(&self, msg_id: &MessageId) -> Result<(), StorageError>;

    /// Marks a message as read.
    async fn mark_read(&self, msg_id: &MessageId) -> Result<(), StorageError>;

    /// Retrieves messages for a user, ordered by timestamp.
    async fn get_messages_for_user(
        &self,
        user_id: &UserId,
        limit: u32,
    ) -> Result<Vec<StoredMessage>, StorageError>;
}
