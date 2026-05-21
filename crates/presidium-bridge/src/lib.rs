//! # Presidium Bridge
//!
//! UniFFI bindings bridge for Presidium Messenger — Kotlin Multiplatform / Android integration.
//!
//! This crate provides the FFI bridge between the Rust core and the
//! Kotlin Multiplatform Android client using Mozilla's UniFFI framework.
//!
//! ## Architecture
//!
//! - `domain` — bridge-specific domain types (FFI-safe wrappers)
//! - `application` — bridge API surface definitions
//! - `infrastructure` — UniFFI macro implementations
//!
//! ## Note
//!
//! This crate is a placeholder and will be developed when the
//! Android client integration begins.

pub mod application;
pub mod domain;
pub mod infrastructure;
