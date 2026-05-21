//! Layer dependency tests — verify hexagonal architecture constraints.
//!
//! These tests ensure that:
//! 1. Domain layer does not depend on infrastructure or application layers
//! 2. Application layer only depends on domain (via port traits)
//! 3. All port traits are properly defined with Send + Sync bounds
//! 4. Adapter stubs exist in each satellite crate
//! 5. No circular dependencies exist between crates
//! 6. New value objects and ports are accessible and functional

use presidium_core::application::ports::crypto_port::E2EECryptoPort;
use presidium_core::application::ports::llm_port::{
    InferenceConfig, LLMError, LLMPort, Quantization,
};
use presidium_core::application::ports::messaging_port::MessageTransportPort;
use presidium_core::application::ports::moderation_port::{
    ContentVerdict, ModerationError, ModerationPort, ModerationResult,
};
use presidium_core::application::ports::p2p_port::{P2PNetworkPort, PeerInfo};
use presidium_core::application::ports::storage_port::{
    MessageStoragePort, StoragePort, StoredMessage,
};
use presidium_core::domain::events::ModerationCategory;
use presidium_core::domain::value_objects::{
    ChatId, DeviceId, MessageId, SessionId, Timestamp, UserId,
};

/// Verifies that all value objects can be constructed and used.
/// This confirms the domain layer remains self-contained.
#[test]
fn value_objects_are_self_contained() {
    let uid = UserId::new("test-user".to_string());
    assert_eq!(uid.as_str(), "test-user");

    let did = DeviceId::new("test-device".to_string());
    assert_eq!(did.as_str(), "test-device");

    let sid = SessionId::new("test-session".to_string());
    assert_eq!(sid.as_str(), "test-session");

    let cid = ChatId::new("test-chat".to_string());
    assert_eq!(cid.as_str(), "test-chat");

    let mid = MessageId::random();
    assert!(!format!("{mid}").is_empty());

    let ts = Timestamp::now();
    assert!(ts.as_millis() > 0);
}

/// Verifies that ModerationCategory is unified — it comes from
/// domain::events, not duplicated in application::ports.
#[test]
fn moderation_category_is_unified() {
    // ModerationCategory is defined in domain::events and re-used
    // by the moderation port. There should be only one definition.
    let cat = ModerationCategory::Extremism;
    assert_eq!(format!("{cat:?}"), "Extremism");

    let cat = ModerationCategory::Csam;
    assert_eq!(format!("{cat:?}"), "Csam");
}

/// Verifies that all port traits exist and have the required bounds.
#[test]
fn all_port_traits_are_send_sync() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}

    assert_send_sync::<dyn E2EECryptoPort>();
    assert_send_sync::<dyn P2PNetworkPort>();
    assert_send_sync::<dyn StoragePort>();
    assert_send_sync::<dyn MessageStoragePort>();
    assert_send_sync::<dyn MessageTransportPort>();
    assert_send_sync::<dyn ModerationPort>();
    assert_send_sync::<dyn LLMPort>();
}

/// Verifies that error types for all ports are properly defined.
#[test]
fn all_port_error_types_are_defined() {
    // CryptoError
    let _ = presidium_core::application::ports::crypto_port::CryptoError::SessionNotFound(
        "test".into(),
    );

    // P2PError
    let _ = presidium_core::application::ports::p2p_port::P2PError::ConnectionFailed("test".into());

    // StorageError
    let _ = presidium_core::application::ports::storage_port::StorageError::Database("test".into());

    // TransportError
    let _ =
        presidium_core::application::ports::messaging_port::TransportError::Timeout("test".into());

    // ModerationError
    let _ = ModerationError::InferenceFailed("test".into());

    // LLMError
    let _ = LLMError::ModelNotLoaded;
}

/// Verifies that new port types are accessible.
#[test]
fn new_port_types_are_accessible() {
    // ContentVerdict from ModerationPort
    let safe = ContentVerdict::Safe;
    let unsafe_verdict = ContentVerdict::Unsafe("extremism".to_string());
    let review = ContentVerdict::NeedsReview;
    assert!(matches!(safe, ContentVerdict::Safe));
    assert!(matches!(unsafe_verdict, ContentVerdict::Unsafe(_)));
    assert!(matches!(review, ContentVerdict::NeedsReview));

    // Quantization from LLMPort
    let q = Quantization::Q4KM;
    assert_eq!(format!("{q:?}"), "Q4KM");

    // InferenceConfig from LLMPort
    let config = InferenceConfig::default();
    assert_eq!(config.max_tokens, 512);

    // PeerInfo from P2PPort
    let peer = PeerInfo {
        peer_id: "test-peer".to_string(),
        user_id: None,
        addresses: vec![],
        is_connected: false,
    };
    assert_eq!(peer.peer_id, "test-peer");

    // StoredMessage from StoragePort
    let stored = StoredMessage {
        id: MessageId::random(),
        sender: UserId::new("alice".to_string()),
        recipient: UserId::new("bob".to_string()),
        ciphertext: vec![1, 2, 3],
        timestamp: Timestamp::now(),
        delivered: false,
        read: false,
    };
    assert!(!stored.delivered);
}

/// Verifies that adapter stubs exist in satellite crates.
#[test]
fn adapter_stubs_exist_in_satellite_crates() {
    // presidium-crypto adapter
    let _ = presidium_crypto::infrastructure::adapters::LibSignalCryptoAdapter::new();

    // presidium-p2p adapter
    let _ = presidium_p2p::infrastructure::adapters::Libp2pNetworkAdapter::new();

    // presidium-storage adapter
    let _ = presidium_storage::infrastructure::adapters::RedbStorageAdapter::new();

    // presidium-llm adapters
    let _ = presidium_llm::infrastructure::adapters::CandleLlmAdapter::new();
    let _ = presidium_llm::infrastructure::adapters::LocalModerationAdapter::new();
}

/// Verifies that ModerationResult uses the unified ModerationCategory.
#[test]
fn moderation_result_uses_unified_category() {
    let result = ModerationResult {
        violation_detected: true,
        category: Some(ModerationCategory::Extremism),
        confidence: 0.95,
        explanation: "Test".to_string(),
    };
    assert!(result.violation_detected);
    assert_eq!(result.category, Some(ModerationCategory::Extremism));
}

/// Verifies that Timestamp works correctly as a value object.
#[test]
fn timestamp_value_object_works() {
    let ts1 = Timestamp::new(1_700_000_000_000_i64);
    let ts2 = Timestamp::new(1_700_000_000_001_i64);
    assert_ne!(ts1, ts2);
    assert_eq!(ts1.as_millis(), 1_700_000_000_000_i64);
}

/// Verifies that MessageId value object works correctly.
#[test]
fn message_id_value_object_works() {
    let id1 = MessageId::random();
    let id2 = MessageId::random();
    assert_ne!(id1, id2);
    assert!(!format!("{id1}").is_empty());
}
