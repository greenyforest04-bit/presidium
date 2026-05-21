//! redb storage adapter — stub implementation of `StoragePort` and `MessageStoragePort`.
//!
//! This adapter will use `redb` (a simple, high-performance key-value store)
//! for encrypted local storage of messages, chats, and metadata.
//! Currently all methods return `todo!()` as implementation will be
//! completed in subsequent development days.

use async_trait::async_trait;
use presidium_core::application::ports::storage_port::{
    MessageStoragePort, StorageError, StoragePort, StoredMessage,
};
use presidium_core::domain::entities::{Chat, Message};
use presidium_core::domain::value_objects::{ChatId, MessageId, UserId};

/// Adapter implementing `StoragePort` and `MessageStoragePort` using redb.
///
/// This struct will wrap a `redb::Database` with:
/// - Encrypted tables for messages and chats
/// - Atomic transactions for consistency
/// - Efficient key-based lookups
pub struct RedbStorageAdapter {
    // Future fields:
    // db: redb::Database,
    // encryption_key: [u8; 32],
}

impl RedbStorageAdapter {
    /// Creates a new `RedbStorageAdapter`.
    ///
    /// In the full implementation, this will open or create
    /// the encrypted redb database at the configured path.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RedbStorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
#[allow(unused_variables)]
impl StoragePort for RedbStorageAdapter {
    async fn store_message(&self, message: &Message) -> Result<(), StorageError> {
        todo!("Day 7: Implement message storage in redb")
    }

    async fn get_messages(
        &self,
        chat_id: &ChatId,
        limit: usize,
        before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<Message>, StorageError> {
        todo!("Day 7: Implement message retrieval from redb")
    }

    async fn store_chat(&self, chat: &Chat) -> Result<(), StorageError> {
        todo!("Day 7: Implement chat storage in redb")
    }

    async fn get_chat(&self, chat_id: &ChatId) -> Result<Option<Chat>, StorageError> {
        todo!("Day 7: Implement chat retrieval from redb")
    }

    async fn list_chats(&self, user_id: &UserId) -> Result<Vec<Chat>, StorageError> {
        todo!("Day 7: Implement chat listing from redb")
    }

    async fn delete_chat(&self, chat_id: &ChatId) -> Result<(), StorageError> {
        todo!("Day 7: Implement chat deletion from redb")
    }
}

#[async_trait]
#[allow(unused_variables)]
impl MessageStoragePort for RedbStorageAdapter {
    async fn save_outgoing_message(&self, msg: StoredMessage) -> Result<(), StorageError> {
        todo!("Day 7: Implement outgoing ciphertext message storage")
    }

    async fn save_incoming_message(&self, msg: StoredMessage) -> Result<(), StorageError> {
        todo!("Day 7: Implement incoming ciphertext message storage")
    }

    async fn mark_delivered(&self, msg_id: &MessageId) -> Result<(), StorageError> {
        todo!("Day 7: Implement message delivery status update")
    }

    async fn mark_read(&self, msg_id: &MessageId) -> Result<(), StorageError> {
        todo!("Day 7: Implement message read status update")
    }

    async fn get_messages_for_user(
        &self,
        user_id: &UserId,
        limit: u32,
    ) -> Result<Vec<StoredMessage>, StorageError> {
        todo!("Day 7: Implement user message listing from redb")
    }
}
