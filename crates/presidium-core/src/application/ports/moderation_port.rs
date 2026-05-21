//! Moderation port — abstract interface for on-device content moderation.
//!
//! This port defines the contract for local content moderation using
//! an on-device LLM. The moderation system detects prohibited content
//! categories and triggers the "Sarcophagus" mechanism when violations
//! are confirmed.
//!
//! ## Sarcophagus Mechanism
//!
//! When the local LLM detects prohibited content (extremism, CSAM, etc.),
//! it creates an encrypted "sarcophagus" — a package containing evidence
//! of the violation. This package is encrypted so that only designated
//! law enforcement bootstrap nodes can decrypt it, preserving E2EE for
//! all other participants while fulfilling legal obligations.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::events::ModerationCategory;
use crate::domain::value_objects::UserId;

/// Errors specific to moderation operations.
#[derive(Debug, thiserror::Error)]
pub enum ModerationError {
    /// The LLM inference failed.
    #[error("LLM inference failed: {0}")]
    InferenceFailed(String),

    /// The LLM model is not loaded.
    #[error("Model not loaded: {0}")]
    ModelNotLoaded(String),

    /// Content analysis timed out.
    #[error("Analysis timeout: {0}")]
    Timeout(String),

    /// Message content is too long for the LLM to process.
    #[error("Message too long for moderation: {0} bytes")]
    TooLong(usize),

    /// Sarcophagus creation failed.
    #[error("Sarcophagus creation failed: {0}")]
    SarcophagusFailed(String),
}

/// The result of a content moderation analysis.
#[derive(Debug, Clone)]
pub struct ModerationResult {
    /// Whether prohibited content was detected.
    pub violation_detected: bool,
    /// The category of the violation, if any.
    pub category: Option<ModerationCategory>,
    /// Confidence score (0.0–1.0).
    pub confidence: f32,
    /// Brief explanation of the decision.
    pub explanation: String,
}

/// Content verdict from moderation — safe, unsafe, or needs human review.
///
/// This enum provides a clear tri-state result for content moderation,
/// distinguishing between clearly safe content, clearly unsafe content,
/// and borderline cases that require additional human review (dual
/// confirmation) before any action is taken.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentVerdict {
    /// Content is safe — no action needed.
    Safe,
    /// Content is unsafe with the specified reason category.
    ///
    /// The string contains the category identifier such as
    /// "extremism", "csam", "drug_trafficking", "fraud".
    Unsafe(String),
    /// Content is borderline and requires human review.
    ///
    /// No Sarcophagus is created until a second confirmation
    /// is received (dual-confirmation principle).
    NeedsReview,
}

/// Port for on-device content moderation via LLM.
///
/// Implementations must provide:
/// - Local LLM inference (candle.rs / llama-cpp-rs)
/// - Low false-positive rate (dual-confirmation recommended)
/// - Privacy-preserving analysis (no data leaves the device)
/// - Sarcophagus mechanism for legally mandated reporting
#[async_trait]
pub trait ModerationPort: Send + Sync {
    /// Analyzes message content for prohibited material.
    ///
    /// This method runs the on-device LLM to classify the content.
    /// If a violation is detected with sufficient confidence, the
    /// Sarcophagus mechanism may be triggered.
    async fn analyze_content(&self, content: &str) -> Result<ModerationResult, ModerationError>;

    /// Checks a message from a specific sender for prohibited content.
    ///
    /// This is a higher-level method that combines content analysis
    /// with contextual information (sender identity) to produce a
    /// verdict. It may apply different thresholds based on the
    /// sender's trust level or conversation context.
    async fn check_message(
        &self,
        sender: &UserId,
        plaintext: &str,
    ) -> Result<ContentVerdict, ModerationError>;

    /// Creates a Sarcophagus — an encrypted violation report.
    ///
    /// When content is confirmed as prohibited (via dual confirmation),
    /// this method creates an encrypted package containing:
    /// - The offender's user ID
    /// - The evidence (anonymized content excerpt)
    /// - The reason (violation category)
    ///
    /// The package is encrypted so that only designated law enforcement
    /// bootstrap nodes can decrypt it. This preserves E2EE for all
    /// other participants while fulfilling legal obligations.
    async fn create_sarcophagus(
        &self,
        offender: &UserId,
        evidence: &str,
        reason: &str,
    ) -> Result<Vec<u8>, ModerationError>;

    /// Checks if the LLM model is loaded and ready for inference.
    async fn is_model_ready(&self) -> Result<bool, ModerationError>;

    /// Loads the moderation LLM model from the local filesystem.
    ///
    /// The model (GGUF format, 2B–14B parameters) is loaded into
    /// memory with the specified quantization level.
    async fn load_model(&self, model_path: &str) -> Result<(), ModerationError>;

    /// Unloads the LLM model to free memory.
    async fn unload_model(&self) -> Result<(), ModerationError>;
}
