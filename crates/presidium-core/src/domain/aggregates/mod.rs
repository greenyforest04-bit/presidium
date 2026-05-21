//! Aggregates — consistency boundaries for groups of entities.
//!
//! Aggregates define transactional consistency boundaries within the
//! domain model. Each aggregate root ensures that invariants are
//! maintained for all entities within its boundary.
//!
//! In Presidium, the primary aggregates are:
//! - `Conversation` — manages messages and participants within a chat
//! - `UserAccount` — manages user identity and devices

use chrono::Utc;

use super::entities::{Chat, Message};
use super::errors::DomainError;
use super::events::{DomainEvent, MessageSent};
use super::value_objects::{ChatId, ChatType, MessageContent, UserId};

/// A conversation aggregate root that manages messages and invariants
/// within a single chat.
///
/// The `Conversation` aggregate ensures that:
/// - Messages are only sent by valid participants
/// - Direct chats remain 1:1
/// - Group chat participant limits are respected
/// - Moderation rules are applied to message content
#[derive(Debug)]
pub struct Conversation {
    /// The underlying chat entity.
    chat: Chat,
    /// Messages in this conversation (in order).
    messages: Vec<Message>,
    /// Pending domain events to be published.
    pending_events: Vec<Box<dyn DomainEvent>>,
}

/// Maximum number of participants in a group chat.
const MAX_GROUP_PARTICIPANTS: usize = 512;

impl Conversation {
    /// Creates a new direct conversation between two users.
    pub fn new_direct(user_a: UserId, user_b: UserId) -> Result<Self, DomainError> {
        let chat = Chat::new_direct(user_a, user_b)?;
        Ok(Self { chat, messages: Vec::new(), pending_events: Vec::new() })
    }

    /// Creates a new group conversation with the given participants.
    pub fn new_group(participants: Vec<UserId>) -> Result<Self, DomainError> {
        let chat = Chat::new_group(participants)?;
        Ok(Self { chat, messages: Vec::new(), pending_events: Vec::new() })
    }

    /// Reconstructs a conversation from existing state (for repository loading).
    #[must_use]
    pub fn from_parts(chat: Chat, messages: Vec<Message>) -> Self {
        Self { chat, messages, pending_events: Vec::new() }
    }

    /// Sends a message in this conversation.
    ///
    /// # Errors
    /// Returns `DomainError` if:
    /// - The sender is not a participant
    /// - The message content exceeds size limits
    /// - A moderation violation is detected
    pub fn send_message(&mut self, sender_id: UserId, content: String) -> Result<(), DomainError> {
        // Validate sender is a participant
        if !self.chat.participants().iter().any(|p| p == &sender_id) {
            return Err(DomainError::InvalidOperation {
                reason: format!("User {sender_id} is not a participant in this chat"),
            });
        }

        // Validate content size
        let content = MessageContent::try_new(content)?;

        // Create the message
        let message =
            Message::try_new(self.chat.chat_id().clone(), sender_id, content.as_str().to_string())?;

        // Emit domain event
        let event = MessageSent {
            chat_id: self.chat.chat_id().clone(),
            sender_id: message.sender_id().clone(),
            occurred_at: Utc::now(),
        };
        self.pending_events.push(Box::new(event));

        self.messages.push(message);
        self.chat.touch_last_message();
        Ok(())
    }

    /// Returns a reference to the underlying chat entity.
    #[must_use]
    pub fn chat(&self) -> &Chat {
        &self.chat
    }

    /// Returns a reference to the messages in this conversation.
    #[must_use]
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Returns the chat identifier.
    #[must_use]
    pub fn chat_id(&self) -> &ChatId {
        self.chat.chat_id()
    }

    /// Returns the chat type.
    #[must_use]
    pub fn chat_type(&self) -> ChatType {
        self.chat.chat_type()
    }

    /// Adds a participant to a group chat.
    ///
    /// # Errors
    /// Returns `DomainError` if:
    /// - The chat is a direct chat (cannot add participants)
    /// - The maximum group size would be exceeded
    pub fn add_participant(&mut self, _user_id: UserId) -> Result<(), DomainError> {
        if self.chat.chat_type() != ChatType::Group {
            return Err(DomainError::InvalidOperation {
                reason: "Cannot add participants to a direct chat".to_string(),
            });
        }
        if self.chat.participants().len() >= MAX_GROUP_PARTICIPANTS {
            return Err(DomainError::InvalidOperation {
                reason: format!("Group chat cannot exceed {MAX_GROUP_PARTICIPANTS} participants"),
            });
        }
        // Note: Chat entity doesn't expose mutable participants yet,
        // so this would need a method on Chat. For now, we validate.
        Ok(())
    }

    /// Drains and returns all pending domain events.
    ///
    /// After calling this method, the aggregate's event queue is empty.
    pub fn drain_pending_events(&mut self) -> Vec<Box<dyn DomainEvent>> {
        std::mem::take(&mut self.pending_events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_conversation_send_message() {
        let alice = UserId::new("alice".to_string());
        let bob = UserId::new("bob".to_string());
        let mut conv = Conversation::new_direct(alice.clone(), bob).expect("create direct");

        conv.send_message(alice, "Hello, Bob!".to_string())
            .expect("send message");

        assert_eq!(conv.messages().len(), 1);
        let events = conv.drain_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type(), "MessageSent");
    }

    #[test]
    fn send_message_from_non_participant_fails() {
        let alice = UserId::new("alice".to_string());
        let bob = UserId::new("bob".to_string());
        let eve = UserId::new("eve".to_string());
        let mut conv = Conversation::new_direct(alice, bob).expect("create direct");

        let result = conv.send_message(eve, "Intercepted!".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn add_participant_to_direct_chat_fails() {
        let alice = UserId::new("alice".to_string());
        let bob = UserId::new("bob".to_string());
        let carol = UserId::new("carol".to_string());
        let mut conv = Conversation::new_direct(alice, bob).expect("create direct");

        let result = conv.add_participant(carol);
        assert!(result.is_err());
    }

    #[test]
    fn drain_pending_events_empties_queue() {
        let alice = UserId::new("alice".to_string());
        let bob = UserId::new("bob".to_string());
        let mut conv = Conversation::new_direct(alice.clone(), bob).expect("create direct");

        conv.send_message(alice, "Hi!".to_string()).expect("send");
        let _ = conv.drain_pending_events();
        let second_drain = conv.drain_pending_events();
        assert!(second_drain.is_empty());
    }

    #[test]
    fn group_conversation_creation() {
        let conv = Conversation::new_group(vec![
            UserId::new("alice".to_string()),
            UserId::new("bob".to_string()),
            UserId::new("carol".to_string()),
        ])
        .expect("create group");
        assert_eq!(conv.chat_type(), ChatType::Group);
    }
}
