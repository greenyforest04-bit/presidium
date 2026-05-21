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

/// Unique identifier for a message in the Presidium network.
///
/// Each message is identified by a UUID v4, generated at creation time.
/// This is a value object wrapping a `uuid::Uuid` for type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(uuid::Uuid);

impl MessageId {
    /// Creates a new `MessageId` from a UUID.
    #[must_use]
    pub fn new(id: uuid::Uuid) -> Self {
        Self(id)
    }

    /// Generates a new random `MessageId`.
    #[must_use]
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Returns the underlying UUID.
    #[must_use]
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A Unix timestamp in milliseconds.
///
/// Used for consistent timestamp representation across the domain,
/// especially in storage and P2P protocol messages where `DateTime<Utc>`
/// is not suitable for serialization or wire format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Creates a new `Timestamp` from a Unix millisecond value.
    #[must_use]
    pub fn new(millis: i64) -> Self {
        Self(millis)
    }

    /// Returns the current time as a `Timestamp`.
    #[must_use]
    pub fn now() -> Self {
        Self(chrono::Utc::now().timestamp_millis())
    }

    /// Returns the raw Unix millisecond value.
    #[must_use]
    pub fn as_millis(&self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for Timestamp {
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
        let content =
            MessageContent::try_new("Hello, Presidium!".to_string()).expect("valid content");
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

    #[test]
    fn message_id_random_is_unique() {
        let a = MessageId::random();
        let b = MessageId::random();
        assert_ne!(a, b);
    }

    #[test]
    fn message_id_display() {
        let mid =
            MessageId::new(uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
        assert!(format!("{mid}").contains("550e8400"));
    }

    #[test]
    fn timestamp_now_is_positive() {
        let ts = Timestamp::now();
        assert!(ts.as_millis() > 0);
    }

    #[test]
    fn timestamp_new_and_display() {
        let ts = Timestamp::new(1_700_000_000_000_i64);
        assert_eq!(ts.as_millis(), 1_700_000_000_000_i64);
        assert_eq!(format!("{ts}"), "1700000000000");
    }
}
