//! Domain events — significant occurrences within the domain.
//!
//! Domain events represent things that have happened in the domain
//! that other parts of the system may need to react to. They are
//! immutable records of domain state changes.
//!
//! Examples: `UserRegistered`, `MessageSent`, `SessionEstablished`,
//! `ModerationViolationDetected`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::value_objects::{ChatId, DeviceId, SessionId, UserId};

/// A domain event representing a significant occurrence in the system.
///
/// All domain events carry a timestamp and are immutable once created.
/// They are used for event-driven communication between aggregates
/// and for eventual consistency across bounded contexts.
pub trait DomainEvent: std::fmt::Debug + Send + Sync {
    /// Returns the timestamp when the event occurred.
    fn occurred_at(&self) -> DateTime<Utc>;

    /// Returns a human-readable name of the event type.
    fn event_type(&self) -> &'static str;
}

/// Event: A new user has registered on the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    /// The user who registered.
    pub user_id: UserId,
    /// The device used for initial registration.
    pub device_id: DeviceId,
    /// Timestamp of registration.
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for UserRegistered {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_type(&self) -> &'static str {
        "UserRegistered"
    }
}

/// Event: A new E2EE session has been established between two devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEstablished {
    /// The session identifier.
    pub session_id: SessionId,
    /// The local device in the session.
    pub local_device_id: DeviceId,
    /// The remote device in the session.
    pub remote_device_id: DeviceId,
    /// Timestamp of session establishment.
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for SessionEstablished {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_type(&self) -> &'static str {
        "SessionEstablished"
    }
}

/// Event: A message has been sent in a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSent {
    /// The chat where the message was sent.
    pub chat_id: ChatId,
    /// The sender of the message.
    pub sender_id: UserId,
    /// Timestamp when the message was sent.
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for MessageSent {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_type(&self) -> &'static str {
        "MessageSent"
    }
}

/// Event: A moderation violation has been detected by the on-device LLM.
///
/// This is the trigger for the "Sarcophagus" mechanism. When the local
/// LLM detects prohibited content (extremism, CSAM, etc.), this event
/// is emitted. The encrypted violation report is then forwarded to
/// bootstrap nodes and subsequently to law enforcement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationViolationDetected {
    /// The user who sent the violating content.
    pub violator_id: UserId,
    /// The chat where the violation was detected.
    pub chat_id: ChatId,
    /// The category of the violation.
    pub violation_category: ModerationCategory,
    /// Timestamp of detection.
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for ModerationViolationDetected {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_type(&self) -> &'static str {
        "ModerationViolationDetected"
    }
}

/// Categories of content that trigger moderation violations.
///
/// These categories are strictly defined by legal requirements.
/// The local LLM is trained to detect these specific categories,
/// and false positives must be minimized through dual-confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModerationCategory {
    /// Content promoting extremism and terrorism.
    Extremism,
    /// Child sexual abuse material.
    Csam,
    /// Content related to illegal drug trafficking.
    DrugTrafficking,
    /// Fraud and financial scams.
    Fraud,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_registered_event() {
        let event = UserRegistered {
            user_id: UserId::new("alice".to_string()),
            device_id: DeviceId::new("phone-1".to_string()),
            occurred_at: Utc::now(),
        };
        assert_eq!(event.event_type(), "UserRegistered");
    }

    #[test]
    fn session_established_event() {
        let event = SessionEstablished {
            session_id: SessionId::new("s-1".to_string()),
            local_device_id: DeviceId::new("phone-1".to_string()),
            remote_device_id: DeviceId::new("phone-2".to_string()),
            occurred_at: Utc::now(),
        };
        assert_eq!(event.event_type(), "SessionEstablished");
    }

    #[test]
    fn message_sent_event() {
        let event = MessageSent {
            chat_id: ChatId::new("chat-1".to_string()),
            sender_id: UserId::new("alice".to_string()),
            occurred_at: Utc::now(),
        };
        assert_eq!(event.event_type(), "MessageSent");
    }

    #[test]
    fn moderation_violation_event() {
        let event = ModerationViolationDetected {
            violator_id: UserId::new("bad-actor".to_string()),
            chat_id: ChatId::new("chat-1".to_string()),
            violation_category: ModerationCategory::Extremism,
            occurred_at: Utc::now(),
        };
        assert_eq!(event.event_type(), "ModerationViolationDetected");
    }

    #[test]
    fn moderation_category_serialization() {
        let cat = ModerationCategory::Csam;
        let json = serde_json::to_string(&cat).expect("serialize");
        assert_eq!(json, "\"Csam\"");
    }
}
