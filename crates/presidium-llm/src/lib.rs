//! # Presidium LLM
//!
//! On-device LLM inference ports and adapters for Presidium Messenger.
//!
//! This crate provides the interface and implementation for all
//! on-device LLM operations including:
//! - **Local moderation**: detecting prohibited content via LLM
//! - **AI assistant**: RAG-powered assistant within chats
//! - **Inference engine**: candle.rs / llama-cpp-rs backends
//! - **Quantization**: 4-bit, 8-bit, 1.58-bit model loading
//!
//! ## Architecture
//!
//! Follows Hexagonal Architecture with domain/application/infrastructure layers.

pub mod application;
pub mod domain;
pub mod infrastructure;
