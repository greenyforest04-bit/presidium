//! # Presidium Core
//!
//! The hexagonal core of Presidium Messenger containing domain entities,
//! value objects, aggregates, domain events, and application ports.
//!
//! This crate follows the **Hexagonal Architecture** (Ports & Adapters)
//! and **Domain-Driven Design** principles:
//!
//! - `domain` — pure business logic with zero infrastructure dependencies
//! - `application` — use cases (interactors) and port definitions (traits)
//! - `infrastructure` — adapter implementations for external systems
//!
//! ## Architecture Rules
//!
//! - `domain` MUST NOT depend on `application` or `infrastructure`
//! - `application` depends on `domain` only (via port traits)
//! - `infrastructure` depends on `application` and `domain` (implements ports)
//!
//! ## Example
//!
//! ```rust
//! use presidium_core::domain::value_objects::UserId;
//!
//! let user_id = UserId::new("alice".to_string());
//! assert_eq!(user_id.as_str(), "alice");
//! ```

// Re-export public API
pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod observability;
