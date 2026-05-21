//! # Presidium Messaging
//!
//! Messaging domain for Presidium Messenger — chat management,
//! message routing, and conversation logic.
//!
//! This crate provides the core messaging domain including:
//! - **Conversation management**: creating and managing chats
//! - **Message routing**: delivering messages to the right recipients
//! - **Read receipts**: delivery and read confirmation tracking
//! - **Group chat logic**: participant management, admin controls
//!
//! ## Architecture
//!
//! Follows Hexagonal Architecture with domain/application/infrastructure layers.

pub mod application;
pub mod domain;
pub mod infrastructure;
