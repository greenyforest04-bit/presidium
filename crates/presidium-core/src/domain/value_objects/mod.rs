//! Value objects — immutable, equality-by-value domain primitives.
//!
//! Value objects represent domain concepts identified by their attributes
//! rather than a unique identity. They are immutable and compared by value.
//!
//! Examples: `UserId`, `DeviceId`, `MessageContent`, `SessionId`.

use serde::{Deserialize, Serialize};

/// Unique identifier for a user in the Presidium network.
///
/// A `UserId` is a string-based value object that uniquely identifies
/// a participant in the network. It is typically derived from the user's
/// public key fingerprint or a human-readable alias.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    /// Creates a new `UserId` from a string.
    ///
    /// # Panics
    /// In production, this should validate the format. For now, it accepts
    /// any non-empty string.
    #[must_use]
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Returns a string slice of the user identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generates a random `UserId` for testing purposes.
    #[must_use]
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a device within a user's device fleet.
///
/// Presidium supports multiple devices per user (like Signal).
/// Each device has its own E2EE session and pre-key bundle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(String);

impl DeviceId {
    /// Creates a new `DeviceId` from a string.
    #[must_use]
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Returns a string slice of the device identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generates a random `DeviceId` for testing purposes.
    #[must_use]
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl std::fmt::Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for an E2EE session between two devices.
///
/// Each session corresponds to a Double Ratchet instance and is
/// identified by a combination of the local and remote device IDs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
    /// Creates a new `SessionId` from a string.
    #[must_use]
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Returns a string slice of the session identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generates a random `SessionId` for testing purposes.
    #[must_use]
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The content of a message, with enforced size limits.
///
/// Message content is limited to prevent abuse and ensure
/// efficient P2P transmission. The current limit is 64KB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageContent(String);

/// Maximum message content size in bytes (64 KB).
const MAX_MESSAGE_SIZE: usize = 65_536;

impl MessageContent {
    /// Creates a new `MessageContent` after validating size constraints.
    ///
    /// # Errors
    /// Returns `DomainError::MessageTooLarge` if the content exceeds
    /// the maximum allowed size.
    pub fn try_new(content: String) -> Result<Self, super::errors::DomainError> {
        if content.len() > MAX_MESSAGE_SIZE {
            return Err(super::errors::DomainError::MessageTooLarge {
                actual: content.len(),
                max: MAX_MESSAGE_SIZE,
            });
        }
        Ok(Self(content))
    }

    /// Returns a string slice of the message content.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the length of the message content in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the message content is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// The type of a chat — 1:1 private conversation or group chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChatType {
    /// Direct 1:1 encrypted conversation.
    Direct,
    /// Group conversation with multiple participants.
    Group,
}

/// Unique identifier for a chat (1:1 or group).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChatId(String);

impl ChatId {
    /// Creates a new `ChatId` from a string.
    #[must_use]
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Returns a string slice of the chat identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generates a random `ChatId` for testing purposes.
    #[must_use]
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_new_and_display() {
        let uid = UserId::new("alice".to_string());
        assert_eq!(uid.as_str(), "alice");
        assert_eq!(format!("{uid}"), "alice");
    }

    #[test]
    fn user_id_random_is_unique() {
        let a = UserId::random();
        let b = UserId::random();
        assert_ne!(a, b);
    }

    #[test]
    fn device_id_new_and_display() {
        let did = DeviceId::new("phone-1".to_string());
        assert_eq!(did.as_str(), "phone-1");
        assert_eq!(format!("{did}"), "phone-1");
    }

    #[test]
    fn session_id_new_and_display() {
        let sid = SessionId::new("session-abc".to_string());
        assert_eq!(sid.as_str(), "session-abc");
    }

    #[test]
    fn message_content_valid() {
        let content = MessageContent::try_new("Hello, Presidium!".to_string()).expect("valid content");
        assert_eq!(content.as_str(), "Hello, Presidium!");
        assert_eq!(content.len(), 17);
        assert!(!content.is_empty());
    }

    #[test]
    fn message_content_too_large() {
        let big = "x".repeat(MAX_MESSAGE_SIZE + 1);
        let result = MessageContent::try_new(big);
        assert!(result.is_err());
    }

    #[test]
    fn message_content_empty() {
        let content = MessageContent::try_new(String::new()).expect("empty is valid");
        assert!(content.is_empty());
    }

    #[test]
    fn chat_id_random_is_unique() {
        let a = ChatId::random();
        let b = ChatId::random();
        assert_ne!(a, b);
    }

    #[test]
    fn chat_type_serialization() {
        let direct = ChatType::Direct;
        let json = serde_json::to_string(&direct).expect("serialize");
        assert_eq!(json, "\"Direct\"");
        let decoded: ChatType = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, direct);
    }
}
