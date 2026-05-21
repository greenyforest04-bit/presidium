//! Candle LLM adapter — stub implementation of `LLMPort`.
//!
//! This adapter will use `candle.rs` (and optionally `llama-cpp-rs`)
//! to implement on-device LLM inference for GGUF models.
//! Currently all methods return `todo!()` as implementation will be
//! completed in subsequent development days.

use async_trait::async_trait;
use presidium_core::application::ports::llm_port::{
    InferenceConfig, LLMError, LLMPort, Quantization,
};

/// Adapter implementing `LLMPort` using candle.rs.
///
/// This struct will wrap a `candle::Model` with:
/// - GGUF model loading and quantization
/// - Token-by-token text generation
/// - KV-cache management for efficient inference
pub struct CandleLlmAdapter {
    // Future fields:
    // model: Option<candle::Model>,
    // tokenizer: Option<Tokenizer>,
    // config: LlmConfig,
}

impl CandleLlmAdapter {
    /// Creates a new `CandleLlmAdapter`.
    ///
    /// In the full implementation, this will initialize the
    /// inference backend without loading a model yet.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CandleLlmAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
#[allow(unused_variables)]
impl LLMPort for CandleLlmAdapter {
    async fn load_model(&self, model_path: &str, quant: Quantization) -> Result<(), LLMError> {
        todo!("Day 9: Implement GGUF model loading with candle.rs")
    }

    async fn unload_model(&self) -> Result<(), LLMError> {
        todo!("Day 9: Implement model unloading and memory cleanup")
    }

    fn is_loaded(&self) -> bool {
        todo!("Day 9: Implement model loaded check")
    }

    async fn infer(&self, prompt: &str, max_tokens: usize) -> Result<String, LLMError> {
        todo!("Day 9: Implement text generation with candle.rs")
    }

    async fn infer_with_config(
        &self,
        prompt: &str,
        config: &InferenceConfig,
    ) -> Result<String, LLMError> {
        todo!("Day 9: Implement configurable text generation")
    }
}
