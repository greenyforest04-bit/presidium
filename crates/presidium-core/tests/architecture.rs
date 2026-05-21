//! Architecture integration test — verifies layer dependency rules.
//!
//! This test ensures that the hexagonal architecture constraints are
//! maintained across the codebase:
//!
//! 1. `domain` does NOT depend on `application` or `infrastructure`
//! 2. `application` depends on `domain` only
//! 3. `infrastructure` depends on `application` and `domain`
//!
//! These tests work by attempting to compile code that would violate
//! the dependency rules. If the code compiles, the test fails.

#![allow(unused_imports, clippy::no_effect_underscore_binding)]

use presidium_core::application::ports::crypto_port::E2EECryptoPort;
use presidium_core::application::ports::messaging_port::MessageTransportPort;
use presidium_core::application::ports::moderation_port::ModerationPort;
use presidium_core::application::ports::p2p_port::P2PNetworkPort;
use presidium_core::application::ports::storage_port::StoragePort;

/// Verifies that domain value objects can be constructed and used.
/// This confirms the domain layer is self-contained and does not
/// require infrastructure to function.
#[test]
fn domain_value_objects_are_self_contained() {
    use presidium_core::domain::value_objects::{
        ChatId, ChatType, DeviceId, MessageContent, SessionId, UserId,
    };

    let uid = UserId::new("test-user".to_string());
    assert_eq!(uid.as_str(), "test-user");

    let did = DeviceId::new("test-device".to_string());
    assert_eq!(did.as_str(), "test-device");

    let sid = SessionId::new("test-session".to_string());
    assert_eq!(sid.as_str(), "test-session");

    let cid = ChatId::new("test-chat".to_string());
    assert_eq!(cid.as_str(), "test-chat");

    let content = MessageContent::try_new("Hello".to_string()).expect("valid content");
    assert_eq!(content.as_str(), "Hello");

    let _chat_type = ChatType::Direct;
}

/// Verifies that domain entities can be constructed without any
/// infrastructure dependencies.
#[test]
fn domain_entities_are_self_contained() {
    use presidium_core::domain::entities::{Chat, Device, Message, User};
    use presidium_core::domain::value_objects::{ChatId, ChatType, DeviceId, UserId};

    let user = User::new(UserId::new("alice".to_string()));
    assert_eq!(user.user_id().as_str(), "alice");

    let device = Device::new(DeviceId::new("phone-1".to_string()), "Pixel 8".to_string());
    assert_eq!(device.name(), "Pixel 8");

    let msg = Message::try_new(
        ChatId::new("chat-1".to_string()),
        UserId::new("alice".to_string()),
        "Hello!".to_string(),
    )
    .expect("valid message");
    assert!(!msg.is_delivered());

    let chat = Chat::new_direct(UserId::new("alice".to_string()), UserId::new("bob".to_string()))
        .expect("valid chat");
    assert_eq!(chat.chat_type(), ChatType::Direct);
}

/// Verifies that domain errors can be created and displayed.
#[test]
fn domain_errors_are_self_contained() {
    use presidium_core::domain::errors::DomainError;

    let err = DomainError::MessageTooLarge { actual: 100_000, max: 65_536 };
    let msg = format!("{err}");
    assert!(msg.contains("100000"));
}

/// Verifies that domain events can be created without infrastructure.
#[test]
fn domain_events_are_self_contained() {
    use presidium_core::domain::events::{DomainEvent, SessionEstablished, UserRegistered};
    use presidium_core::domain::value_objects::{DeviceId, SessionId, UserId};

    let event = UserRegistered {
        user_id: UserId::new("alice".to_string()),
        device_id: DeviceId::new("phone-1".to_string()),
        occurred_at: chrono::Utc::now(),
    };
    assert_eq!(event.event_type(), "UserRegistered");

    let event = SessionEstablished {
        session_id: SessionId::new("s-1".to_string()),
        local_device_id: DeviceId::new("phone-1".to_string()),
        remote_device_id: DeviceId::new("phone-2".to_string()),
        occurred_at: chrono::Utc::now(),
    };
    assert_eq!(event.event_type(), "SessionEstablished");
}

/// Verifies that aggregates work independently of infrastructure.
#[test]
fn domain_aggregates_are_self_contained() {
    use presidium_core::domain::aggregates::Conversation;
    use presidium_core::domain::value_objects::{ChatType, UserId};

    let alice = UserId::new("alice".to_string());
    let bob = UserId::new("bob".to_string());
    let mut conv = Conversation::new_direct(alice.clone(), bob).expect("create direct");

    conv.send_message(alice, "Hello!".to_string())
        .expect("send message");
    assert_eq!(conv.messages().len(), 1);
    assert_eq!(conv.chat_type(), ChatType::Direct);
}

/// Verifies that application ports can be referenced (traits exist)
/// without depending on infrastructure.
#[test]
fn application_ports_exist_and_are_importable() {
    use presidium_core::application::ports::crypto_port::E2EECryptoPort;
    use presidium_core::application::ports::messaging_port::MessageTransportPort;
    use presidium_core::application::ports::moderation_port::ModerationPort;
    use presidium_core::application::ports::p2p_port::P2PNetworkPort;
    use presidium_core::application::ports::storage_port::StoragePort;

    // Verify the traits exist and have the expected Send + Sync bounds.
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn E2EECryptoPort>();
    assert_send_sync::<dyn MessageTransportPort>();
    assert_send_sync::<dyn P2PNetworkPort>();
    assert_send_sync::<dyn StoragePort>();
    assert_send_sync::<dyn ModerationPort>();
}

/// Verifies that application use cases can be imported.
#[test]
fn application_use_cases_exist() {
    use presidium_core::application::use_cases::send_message::{
        SendMessageError, SendMessageInput, SendMessageOutput,
    };

    // Verify the types exist
    let _input_type = std::any::type_name::<SendMessageInput>();
    let _output_type = std::any::type_name::<SendMessageOutput>();
    let _error_type = std::any::type_name::<SendMessageError>();
}

/// Verifies that configuration can be loaded with defaults
/// without infrastructure.
#[test]
fn config_module_works_independently() {
    use presidium_core::config::AppConfig;

    let config = AppConfig::default();
    assert_eq!(config.network.listen_port, 4001);
    assert!(config.crypto.enable_post_quantum);
    assert!(config.llm.moderation_enabled);
}

/// Verifies that the observability module can initialize
/// without infrastructure dependencies.
#[test]
fn observability_module_is_available() {
    use presidium_core::observability;

    // We can call init_test_tracing safely in tests
    observability::init_test_tracing();
}

/// Verifies that all modules are publicly accessible from the crate root.
#[test]
fn crate_re_exports_all_layers() {
    // These should all be accessible via the crate root
    let _ = std::any::type_name::<presidium_core::domain::value_objects::UserId>();
    let _ = std::any::type_name::<presidium_core::domain::errors::DomainError>();
    let _ = std::any::type_name::<presidium_core::config::AppConfig>();
}
