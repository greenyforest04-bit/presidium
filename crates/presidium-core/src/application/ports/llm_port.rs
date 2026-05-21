//! LLM inference port — abstract interface for on-device LLM operations.
//!
//! This port defines the contract for local LLM inference, used both
//! for content moderation and for the AI assistant feature. The primary
//! implementations will use `candle.rs` and `llama-cpp-rs` backends
//! with GGUF model format.
//!
//! ## Separation from ModerationPort
//!
//! `LLMPort` is a low-level inference interface concerned only with
//! loading models and generating text. `ModerationPort` uses `LLMPort`
//! internally for content classification. This separation allows the
//! same LLM backend to be shared between moderation and assistant features.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Errors specific to LLM inference operations.
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    /// The requested model is not loaded into memory.
    #[error("Model not loaded")]
    ModelNotLoaded,

    /// Inference did not complete within the timeout.
    #[error("Inference timeout")]
    Timeout,

    /// The prompt is invalid (empty, too long, or malformed).
    #[error("Invalid prompt: {0}")]
    InvalidPrompt(String),

    /// The model file could not be found or loaded.
    #[error("Model load failed: {0}")]
    LoadFailed(String),

    /// An internal error during inference.
    #[error("Internal inference error: {0}")]
    Internal(String),
}

/// Quantization levels for GGUF model loading.
///
/// These correspond to common quantization schemes used in
/// llama.cpp / candle.rs for reducing model size while
/// maintaining acceptable inference quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quantization {
    /// 2-bit quantization — smallest, lowest quality.
    Q2K,
    /// 3-bit quantization.
    Q3K,
    /// 4-bit medium — good balance of size and quality.
    Q4KM,
    /// 4-bit small — smaller than Q4KM.
    Q4KS,
    /// 5-bit medium.
    Q5KM,
    /// 5-bit small.
    Q5KS,
    /// 6-bit quantization.
    Q6K,
    /// 8-bit quantization — larger, higher quality.
    Q8,
    /// 1.58-bit quantization — experimental, very small.
    Q1_5,
}

/// Inference parameters for controlling text generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Maximum number of tokens to generate.
    pub max_tokens: usize,
    /// Sampling temperature (0.0 = deterministic, 1.0 = creative).
    pub temperature: f32,
    /// Top-p (nucleus) sampling threshold.
    pub top_p: f32,
    /// Number of top tokens to consider for top-k sampling.
    pub top_k: usize,
    /// Number of tokens from the prompt to use as context.
    pub context_window: usize,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self { max_tokens: 512, temperature: 0.7, top_p: 0.9, top_k: 40, context_window: 4096 }
    }
}

/// Port for on-device LLM inference.
///
/// Implementations must provide:
/// - GGUF model loading from local filesystem
/// - Text generation with configurable parameters
/// - Model lifecycle management (load/unload/is_loaded)
/// - Thread-safe inference (multiple callers may share one model)
#[async_trait]
pub trait LLMPort: Send + Sync {
    /// Loads a GGUF model from the specified path with the given quantization.
    ///
    /// The model is loaded into memory and prepared for inference.
    /// Only one model can be loaded at a time; loading a new model
    /// automatically unloads the previous one.
    async fn load_model(&self, model_path: &str, quant: Quantization) -> Result<(), LLMError>;

    /// Unloads the currently loaded model to free memory.
    async fn unload_model(&self) -> Result<(), LLMError>;

    /// Returns whether a model is currently loaded and ready.
    fn is_loaded(&self) -> bool;

    /// Runs inference with the given prompt and default configuration.
    ///
    /// Returns the generated text as a string. The prompt is expected
    /// to be pre-formatted according to the model's chat template.
    async fn infer(&self, prompt: &str, max_tokens: usize) -> Result<String, LLMError>;

    /// Runs inference with the given prompt and custom configuration.
    ///
    /// This method provides fine-grained control over generation
    /// parameters such as temperature, top-p, and top-k sampling.
    async fn infer_with_config(
        &self,
        prompt: &str,
        config: &InferenceConfig,
    ) -> Result<String, LLMError>;
}
