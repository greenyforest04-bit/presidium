//! Local moderation adapter — stub implementation of `ModerationPort`.
//!
//! This adapter implements on-device content moderation using the
//! local LLM backend. It combines LLM inference with classification
//! logic and the Sarcophagus mechanism for reporting violations.
//! Currently all methods return `todo!()` as implementation will be
//! completed in subsequent development days.

use async_trait::async_trait;
use presidium_core::application::ports::moderation_port::{
    ContentVerdict, ModerationError, ModerationPort, ModerationResult,
};
use presidium_core::domain::value_objects::UserId;

/// Adapter implementing `ModerationPort` using local LLM inference.
///
/// This struct will:
/// - Use `CandleLlmAdapter` (or `LLMPort` trait object) for inference
/// - Apply classification prompts to detect prohibited content
/// - Implement dual-confirmation for borderline cases
/// - Create encrypted Sarcophagus packages for confirmed violations
pub struct LocalModerationAdapter {
    // Future fields:
    // llm: Box<dyn LLMPort>,
    // classification_prompt: String,
    // confidence_threshold: f32,
}

impl LocalModerationAdapter {
    /// Creates a new `LocalModerationAdapter`.
    ///
    /// In the full implementation, this will accept an `LLMPort`
    /// instance and configuration for classification thresholds.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LocalModerationAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
#[allow(unused_variables)]
impl ModerationPort for LocalModerationAdapter {
    async fn analyze_content(&self, content: &str) -> Result<ModerationResult, ModerationError> {
        todo!("Day 9: Implement content analysis using local LLM")
    }

    async fn check_message(
        &self,
        sender: &UserId,
        plaintext: &str,
    ) -> Result<ContentVerdict, ModerationError> {
        todo!("Day 9: Implement message checking with sender context")
    }

    async fn create_sarcophagus(
        &self,
        offender: &UserId,
        evidence: &str,
        reason: &str,
    ) -> Result<Vec<u8>, ModerationError> {
        todo!("Day 10: Implement Sarcophagus encrypted package creation")
    }

    async fn is_model_ready(&self) -> Result<bool, ModerationError> {
        todo!("Day 9: Implement model readiness check")
    }

    async fn load_model(&self, model_path: &str) -> Result<(), ModerationError> {
        todo!("Day 9: Implement model loading for moderation")
    }

    async fn unload_model(&self) -> Result<(), ModerationError> {
        todo!("Day 9: Implement model unloading for moderation")
    }
}
