//! Domain entities — objects with a distinct identity and lifecycle.
//!
//! Entities are domain objects that are distinguished by their identity
//! rather than their attributes. They are mutable and trackable over time.
//!
//! Examples: `User`, `Device`, `Message`, `Chat`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::errors::DomainError;
use super::value_objects::{ChatId, ChatType, DeviceId, MessageContent, UserId};

/// A user in the Presidium network.
///
/// A `User` represents a participant identified by a unique `UserId`.
/// Users can own multiple devices and participate in multiple chats.
/// The user's identity is derived from their cryptographic key material.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// The unique identifier of the user.
    user_id: UserId,
    /// Human-readable display name (optional, can be changed).
    display_name: Option<String>,
    /// The list of devices registered to this user.
    devices: Vec<Device>,
    /// Timestamp of when the user was first seen on the network.
    created_at: DateTime<Utc>,
    /// Timestamp of the last activity.
    last_active_at: DateTime<Utc>,
}

impl User {
    /// Creates a new `User` with the given identifier and no devices.
    #[must_use]
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            display_name: None,
            devices: Vec::new(),
            created_at: now,
            last_active_at: now,
        }
    }

    /// Returns a reference to the user's identifier.
    #[must_use]
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    /// Returns the display name, if set.
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Sets or updates the display name.
    pub fn set_display_name(&mut self, name: String) {
        self.display_name = Some(name);
        self.last_active_at = Utc::now();
    }

    /// Returns a reference to the user's devices.
    #[must_use]
    pub fn devices(&self) -> &[Device] {
        &self.devices
    }

    /// Registers a new device for this user.
    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
        self.last_active_at = Utc::now();
    }

    /// Finds a device by its identifier.
    #[must_use]
    pub fn find_device(&self, device_id: &DeviceId) -> Option<&Device> {
        self.devices.iter().find(|d| d.device_id() == device_id)
    }

    /// Returns the timestamp of creation.
    #[must_use]
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the timestamp of last activity.
    #[must_use]
    pub fn last_active_at(&self) -> DateTime<Utc> {
        self.last_active_at
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Eq for User {}

/// A device registered to a user.
///
/// Each device has its own E2EE key material and pre-key bundle.
/// A user can have multiple devices (phone, desktop, tablet), each
/// participating independently in the Double Ratchet protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// The unique identifier of this device.
    device_id: DeviceId,
    /// Human-readable device name (e.g., "Pixel 8 Pro").
    name: String,
    /// Whether this device is currently online.
    is_online: bool,
    /// Timestamp of when the device was registered.
    registered_at: DateTime<Utc>,
    /// Timestamp of the last time the device was seen online.
    last_seen_at: DateTime<Utc>,
}

impl Device {
    /// Creates a new `Device` with the given identifier and name.
    #[must_use]
    pub fn new(device_id: DeviceId, name: String) -> Self {
        let now = Utc::now();
        Self { device_id, name, is_online: false, registered_at: now, last_seen_at: now }
    }

    /// Returns a reference to the device's identifier.
    #[must_use]
    pub fn device_id(&self) -> &DeviceId {
        &self.device_id
    }

    /// Returns the device name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the device is currently online.
    #[must_use]
    pub fn is_online(&self) -> bool {
        self.is_online
    }

    /// Marks the device as online.
    pub fn set_online(&mut self) {
        self.is_online = true;
        self.last_seen_at = Utc::now();
    }

    /// Marks the device as offline.
    pub fn set_offline(&mut self) {
        self.is_online = false;
        self.last_seen_at = Utc::now();
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.device_id == other.device_id
    }
}

impl Eq for Device {}

/// A message sent within a chat.
///
/// Messages are the fundamental unit of communication in Presidium.
/// They are always transmitted in encrypted form over the P2P network.
/// The plaintext content is only available on the sender's and recipient's
/// devices after successful E2EE decryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier.
    message_id: uuid::Uuid,
    /// The chat this message belongs to.
    chat_id: ChatId,
    /// The sender's user identifier.
    sender_id: UserId,
    /// The encrypted message content (ciphertext).
    content: MessageContent,
    /// Timestamp when the message was sent.
    sent_at: DateTime<Utc>,
    /// Timestamp when the message was delivered (if confirmed).
    delivered_at: Option<DateTime<Utc>>,
    /// Timestamp when the message was read by the recipient (if confirmed).
    read_at: Option<DateTime<Utc>>,
}

impl Message {
    /// Creates a new message with the given parameters.
    ///
    /// # Errors
    /// Returns `DomainError` if the message content exceeds size limits.
    pub fn try_new(
        chat_id: ChatId,
        sender_id: UserId,
        content: String,
    ) -> Result<Self, DomainError> {
        let content = MessageContent::try_new(content)?;
        Ok(Self {
            message_id: uuid::Uuid::new_v4(),
            chat_id,
            sender_id,
            content,
            sent_at: Utc::now(),
            delivered_at: None,
            read_at: None,
        })
    }

    /// Returns the message identifier.
    #[must_use]
    pub fn message_id(&self) -> uuid::Uuid {
        self.message_id
    }

    /// Returns a reference to the chat identifier.
    #[must_use]
    pub fn chat_id(&self) -> &ChatId {
        &self.chat_id
    }

    /// Returns a reference to the sender's user identifier.
    #[must_use]
    pub fn sender_id(&self) -> &UserId {
        &self.sender_id
    }

    /// Returns a reference to the message content.
    #[must_use]
    pub fn content(&self) -> &MessageContent {
        &self.content
    }

    /// Returns the timestamp when the message was sent.
    #[must_use]
    pub fn sent_at(&self) -> DateTime<Utc> {
        self.sent_at
    }

    /// Marks the message as delivered.
    pub fn mark_delivered(&mut self) {
        self.delivered_at = Some(Utc::now());
    }

    /// Marks the message as read.
    pub fn mark_read(&mut self) {
        self.read_at = Some(Utc::now());
    }

    /// Returns whether the message has been delivered.
    #[must_use]
    pub fn is_delivered(&self) -> bool {
        self.delivered_at.is_some()
    }

    /// Returns whether the message has been read.
    #[must_use]
    pub fn is_read(&self) -> bool {
        self.read_at.is_some()
    }
}

/// A chat (1:1 or group) between users.
///
/// Chats aggregate messages and manage the list of participants.
/// Each chat has a unique `ChatId` and a type (direct or group).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    /// The unique identifier of this chat.
    chat_id: ChatId,
    /// The type of chat (direct or group).
    chat_type: ChatType,
    /// The list of participant user IDs.
    participants: Vec<UserId>,
    /// Timestamp of chat creation.
    created_at: DateTime<Utc>,
    /// Timestamp of the last message in this chat.
    last_message_at: Option<DateTime<Utc>>,
}

impl Chat {
    /// Creates a new direct (1:1) chat between two users.
    pub fn new_direct(user_a: UserId, user_b: UserId) -> Result<Self, DomainError> {
        if user_a == user_b {
            return Err(DomainError::InvalidOperation {
                reason: "Cannot create a direct chat with yourself".to_string(),
            });
        }
        Ok(Self {
            chat_id: ChatId::new(uuid::Uuid::new_v4().to_string()),
            chat_type: ChatType::Direct,
            participants: vec![user_a, user_b],
            created_at: Utc::now(),
            last_message_at: None,
        })
    }

    /// Creates a new group chat with the given participants.
    ///
    /// # Errors
    /// Returns `DomainError::InvalidOperation` if fewer than 2 participants.
    pub fn new_group(participants: Vec<UserId>) -> Result<Self, DomainError> {
        if participants.len() < 2 {
            return Err(DomainError::InvalidOperation {
                reason: "A group chat requires at least 2 participants".to_string(),
            });
        }
        Ok(Self {
            chat_id: ChatId::new(uuid::Uuid::new_v4().to_string()),
            chat_type: ChatType::Group,
            participants,
            created_at: Utc::now(),
            last_message_at: None,
        })
    }

    /// Returns a reference to the chat identifier.
    #[must_use]
    pub fn chat_id(&self) -> &ChatId {
        &self.chat_id
    }

    /// Returns the chat type.
    #[must_use]
    pub fn chat_type(&self) -> ChatType {
        self.chat_type
    }

    /// Returns a reference to the participant list.
    #[must_use]
    pub fn participants(&self) -> &[UserId] {
        &self.participants
    }

    /// Returns the timestamp of chat creation.
    #[must_use]
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Updates the last message timestamp.
    pub fn touch_last_message(&mut self) {
        self.last_message_at = Some(Utc::now());
    }
}

impl PartialEq for Chat {
    fn eq(&self, other: &Self) -> bool {
        self.chat_id == other.chat_id
    }
}

impl Eq for Chat {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_creation() {
        let uid = UserId::new("alice".to_string());
        let user = User::new(uid.clone());
        assert_eq!(user.user_id(), &uid);
        assert!(user.display_name().is_none());
        assert!(user.devices().is_empty());
    }

    #[test]
    fn user_set_display_name() {
        let uid = UserId::new("bob".to_string());
        let mut user = User::new(uid);
        user.set_display_name("Bob Smith".to_string());
        assert_eq!(user.display_name(), Some("Bob Smith"));
    }

    #[test]
    fn user_add_device() {
        let uid = UserId::new("alice".to_string());
        let mut user = User::new(uid);
        let device = Device::new(DeviceId::new("phone-1".to_string()), "Pixel 8".to_string());
        user.add_device(device);
        assert_eq!(user.devices().len(), 1);
    }

    #[test]
    fn device_online_offline() {
        let mut device = Device::new(DeviceId::new("phone-1".to_string()), "Pixel 8".to_string());
        assert!(!device.is_online());
        device.set_online();
        assert!(device.is_online());
        device.set_offline();
        assert!(!device.is_online());
    }

    #[test]
    fn message_creation() {
        let msg = Message::try_new(
            ChatId::new("chat-1".to_string()),
            UserId::new("alice".to_string()),
            "Hello, Bob!".to_string(),
        )
        .expect("valid message");
        assert!(!msg.is_delivered());
        assert!(!msg.is_read());
    }

    #[test]
    fn message_delivery_and_read() {
        let mut msg = Message::try_new(
            ChatId::new("chat-1".to_string()),
            UserId::new("alice".to_string()),
            "Hello!".to_string(),
        )
        .expect("valid message");
        msg.mark_delivered();
        assert!(msg.is_delivered());
        assert!(!msg.is_read());
        msg.mark_read();
        assert!(msg.is_read());
    }

    #[test]
    fn direct_chat_creation() {
        let chat =
            Chat::new_direct(UserId::new("alice".to_string()), UserId::new("bob".to_string()))
                .expect("valid direct chat");
        assert_eq!(chat.chat_type(), ChatType::Direct);
        assert_eq!(chat.participants().len(), 2);
    }

    #[test]
    fn direct_chat_with_self_fails() {
        let result =
            Chat::new_direct(UserId::new("alice".to_string()), UserId::new("alice".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn group_chat_creation() {
        let chat = Chat::new_group(vec![
            UserId::new("alice".to_string()),
            UserId::new("bob".to_string()),
            UserId::new("carol".to_string()),
        ])
        .expect("valid group chat");
        assert_eq!(chat.chat_type(), ChatType::Group);
        assert_eq!(chat.participants().len(), 3);
    }

    #[test]
    fn group_chat_too_few_participants() {
        let result = Chat::new_group(vec![UserId::new("alice".to_string())]);
        assert!(result.is_err());
    }
}
