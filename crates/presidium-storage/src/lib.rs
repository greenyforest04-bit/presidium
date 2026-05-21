//! # Presidium Storage
//!
//! Storage ports and adapters for Presidium Messenger.
//!
//! This crate provides the interface and implementation for all
//! persistent storage operations including:
//! - **Encrypted local storage**: SQLCipher / encrypted SQLite
//! - **Message persistence**: encrypted messages stored locally
//! - **Key-value store**: for settings and metadata
//!
//! ## Architecture
//!
//! Follows Hexagonal Architecture with domain/application/infrastructure layers.

pub mod application;
pub mod domain;
pub mod infrastructure;
