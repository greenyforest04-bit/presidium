//! Moderation port — abstract interface for on-device content moderation.
//!
//! This port defines the contract for local content moderation using
//! an on-device LLM. The moderation system detects prohibited content
//! categories and triggers the "Sarcophagus" mechanism when violations
//! are confirmed.

use async_trait::async_trait;

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

/// Categories of prohibited content that the moderation system detects.
///
/// These categories are strictly defined by legal requirements and
/// are the only valid triggers for the Sarcophagus mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Port for on-device content moderation via LLM.
///
/// Implementations must provide:
/// - Local LLM inference (candle.rs / llama-cpp-rs)
/// - Low false-positive rate (dual-confirmation recommended)
/// - Privacy-preserving analysis (no data leaves the device)
#[async_trait]
pub trait ModerationPort: Send + Sync {
    /// Analyzes message content for prohibited material.
    ///
    /// This method runs the on-device LLM to classify the content.
    /// If a violation is detected with sufficient confidence, the
    /// Sarcophagus mechanism is triggered.
    async fn analyze_content(&self, content: &str) -> Result<ModerationResult, ModerationError>;

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
