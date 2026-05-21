//! Storage port — abstract interface for persistent data storage.
//!
//! This port defines the contract for all persistent storage operations.
//! The primary implementation will use an encrypted local database
//! (SQLite with SQLCipher or similar).

use async_trait::async_trait;

use crate::domain::entities::{Chat, Message};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{ChatId, UserId};

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

    /// A domain error occurred during a storage operation.
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
}

/// Port for persistent storage of messages, chats, and metadata.
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
