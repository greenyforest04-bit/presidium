//! Send message use case — orchestrates E2EE encryption and message delivery.
//!
//! This use case coordinates the following steps:
//! 1. Validate the message content against domain rules
//! 2. Encrypt the message using the E2EE crypto port
//! 3. Send the encrypted message via the messaging transport port
//! 4. Store the message locally via the storage port
//! 5. Optionally, analyze the content via the moderation port

use crate::application::ports::crypto_port::E2EECryptoPort;
use crate::application::ports::messaging_port::MessageTransportPort;
use crate::application::ports::moderation_port::ModerationPort;
use crate::application::ports::storage_port::StoragePort;
use crate::domain::value_objects::{ChatId, SessionId, UserId};

/// Input data for the send message use case.
#[derive(Debug, Clone)]
pub struct SendMessageInput {
    /// The chat to send the message to.
    pub chat_id: ChatId,
    /// The sender's user identifier.
    pub sender_id: UserId,
    /// The plaintext message content.
    pub content: String,
    /// The E2EE session to use for encryption.
    pub session_id: SessionId,
}

/// Output data for the send message use case.
#[derive(Debug, Clone)]
pub struct SendMessageOutput {
    /// The unique identifier of the sent message.
    pub message_id: uuid::Uuid,
    /// Whether a moderation violation was detected.
    pub moderation_triggered: bool,
}

/// The send message interactor.
///
/// This use case encrypts and sends a message in an E2EE chat.
/// It demonstrates the Hexagonal Architecture pattern: the use case
/// depends only on port traits, not on concrete implementations.
pub struct SendMessageUseCase<C, T, S, M>
where
    C: E2EECryptoPort,
    T: MessageTransportPort,
    S: StoragePort,
    M: ModerationPort,
{
    crypto: C,
    transport: T,
    #[allow(dead_code)]
    storage: S,
    moderation: M,
}

impl<C, T, S, M> SendMessageUseCase<C, T, S, M>
where
    C: E2EECryptoPort,
    T: MessageTransportPort,
    S: StoragePort,
    M: ModerationPort,
{
    /// Creates a new send message use case with the given port implementations.
    pub fn new(crypto: C, transport: T, storage: S, moderation: M) -> Self {
        Self { crypto, transport, storage, moderation }
    }

    /// Executes the send message use case.
    ///
    /// # Errors
    /// Returns an error if encryption, transport, or storage fails.
    pub async fn execute(
        &self,
        input: SendMessageInput,
    ) -> Result<SendMessageOutput, SendMessageError> {
        // Step 1: Moderate the content (on-device LLM analysis)
        let moderation_result = self
            .moderation
            .analyze_content(&input.content)
            .await
            .map_err(SendMessageError::Moderation)?;

        if moderation_result.violation_detected {
            // The Sarcophagus mechanism will be triggered by the
            // ModerationViolationDetected domain event.
            // We still return a result — the caller decides what to do.
            return Ok(SendMessageOutput {
                message_id: uuid::Uuid::new_v4(),
                moderation_triggered: true,
            });
        }

        // Step 2: Encrypt the message using the E2EE crypto port
        let ciphertext = self
            .crypto
            .encrypt_message(&input.session_id, input.content.as_bytes())
            .await
            .map_err(SendMessageError::Crypto)?;

        // Step 3: Send the encrypted message via the transport port
        // For direct chats, we send directly; for groups, we broadcast.
        self.transport
            .send_message(&input.sender_id, &ciphertext)
            .await
            .map_err(SendMessageError::Transport)?;

        // Step 4: Store the message locally
        // Note: In production, we'd create a proper Message entity here.
        // For now, this demonstrates the flow.

        Ok(SendMessageOutput { message_id: uuid::Uuid::new_v4(), moderation_triggered: false })
    }
}

/// Errors that can occur during the send message use case.
#[derive(Debug, thiserror::Error)]
pub enum SendMessageError {
    /// A cryptographic operation failed.
    #[error("Crypto error: {0}")]
    Crypto(#[from] crate::application::ports::crypto_port::CryptoError),

    /// A transport operation failed.
    #[error("Transport error: {0}")]
    Transport(#[from] crate::application::ports::messaging_port::TransportError),

    /// A storage operation failed.
    #[error("Storage error: {0}")]
    Storage(#[from] crate::application::ports::storage_port::StorageError),

    /// A moderation operation failed.
    #[error("Moderation error: {0}")]
    Moderation(#[from] crate::application::ports::moderation_port::ModerationError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::crypto_port::{CryptoError, PreKeyBundle};
    use crate::application::ports::messaging_port::TransportError;
    use crate::application::ports::moderation_port::{ModerationError, ModerationResult};
    use crate::application::ports::storage_port::StorageError;
    use crate::domain::value_objects::DeviceId;
    use async_trait::async_trait;
    use mockall::mock;

    // Generate mock implementations for all ports
    mock! {
        pub CryptoPortMock {}

        #[async_trait]
        impl E2EECryptoPort for CryptoPortMock {
            async fn create_pre_key_bundle(&self) -> Result<PreKeyBundle, CryptoError>;
            async fn establish_session(&self, remote_user: &UserId, remote_device: &DeviceId, bundle: &PreKeyBundle) -> Result<SessionId, CryptoError>;
            async fn encrypt_message(&self, session_id: &SessionId, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError>;
            async fn decrypt_message(&self, session_id: &SessionId, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError>;
            async fn delete_session(&self, session_id: &SessionId) -> Result<(), CryptoError>;
            async fn list_sessions(&self) -> Result<Vec<SessionId>, CryptoError>;
        }
    }

    mock! {
        pub TransportPortMock {}

        #[async_trait]
        impl MessageTransportPort for TransportPortMock {
            async fn send_message(&self, recipient_id: &UserId, ciphertext: &[u8]) -> Result<(), TransportError>;
            async fn receive_message(&self) -> Result<Vec<u8>, TransportError>;
            async fn is_reachable(&self, user_id: &UserId) -> Result<bool, TransportError>;
            async fn broadcast(&self, ciphertext: &[u8]) -> Result<(), TransportError>;
        }
    }

    mock! {
        pub StoragePortMock {}

        #[async_trait]
        impl StoragePort for StoragePortMock {
            async fn store_message(&self, message: &crate::domain::entities::Message) -> Result<(), StorageError>;
            async fn get_messages(&self, chat_id: &ChatId, limit: usize, before: Option<chrono::DateTime<chrono::Utc>>) -> Result<Vec<crate::domain::entities::Message>, StorageError>;
            async fn store_chat(&self, chat: &crate::domain::entities::Chat) -> Result<(), StorageError>;
            async fn get_chat(&self, chat_id: &ChatId) -> Result<Option<crate::domain::entities::Chat>, StorageError>;
            async fn list_chats(&self, user_id: &UserId) -> Result<Vec<crate::domain::entities::Chat>, StorageError>;
            async fn delete_chat(&self, chat_id: &ChatId) -> Result<(), StorageError>;
        }
    }

    mock! {
        pub ModerationPortMock {}

        #[async_trait]
        impl ModerationPort for ModerationPortMock {
            async fn analyze_content(&self, content: &str) -> Result<ModerationResult, ModerationError>;
            async fn is_model_ready(&self) -> Result<bool, ModerationError>;
            async fn load_model(&self, model_path: &str) -> Result<(), ModerationError>;
            async fn unload_model(&self) -> Result<(), ModerationError>;
        }
    }

    #[tokio::test]
    async fn send_message_success() {
        let mut crypto_mock = MockCryptoPortMock::new();
        let mut transport_mock = MockTransportPortMock::new();
        let storage_mock = MockStoragePortMock::new();
        let mut moderation_mock = MockModerationPortMock::new();

        // Moderation returns no violation
        moderation_mock.expect_analyze_content().returning(|_| {
            Ok(ModerationResult {
                violation_detected: false,
                category: None,
                confidence: 0.0,
                explanation: "Clean content".to_string(),
            })
        });

        // Crypto encrypts successfully
        crypto_mock.expect_encrypt_message().returning(|_, _| {
            Ok(vec![1, 2, 3, 4]) // dummy ciphertext
        });

        // Transport sends successfully
        transport_mock
            .expect_send_message()
            .returning(|_, _| Ok(()));

        let use_case =
            SendMessageUseCase::new(crypto_mock, transport_mock, storage_mock, moderation_mock);

        let input = SendMessageInput {
            chat_id: ChatId::new("chat-1".to_string()),
            sender_id: UserId::new("alice".to_string()),
            content: "Hello, Bob!".to_string(),
            session_id: SessionId::new("session-1".to_string()),
        };

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
        let output = result.expect("output");
        assert!(!output.moderation_triggered);
    }

    #[tokio::test]
    async fn send_message_moderation_violation() {
        let crypto_mock = MockCryptoPortMock::new();
        let transport_mock = MockTransportPortMock::new();
        let storage_mock = MockStoragePortMock::new();
        let mut moderation_mock = MockModerationPortMock::new();

        // Moderation detects a violation
        moderation_mock.expect_analyze_content().returning(|_| {
            Ok(ModerationResult {
                violation_detected: true,
                category: Some(
                    crate::application::ports::moderation_port::ModerationCategory::Extremism,
                ),
                confidence: 0.95,
                explanation: "Extremist content detected".to_string(),
            })
        });

        let use_case =
            SendMessageUseCase::new(crypto_mock, transport_mock, storage_mock, moderation_mock);

        let input = SendMessageInput {
            chat_id: ChatId::new("chat-1".to_string()),
            sender_id: UserId::new("bad-actor".to_string()),
            content: "prohibited content".to_string(),
            session_id: SessionId::new("session-1".to_string()),
        };

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
        let output = result.expect("output");
        assert!(output.moderation_triggered);
    }
}
