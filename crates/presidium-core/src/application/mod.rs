//! Application layer — use cases (interactors) and port definitions.
//!
//! The application layer orchestrates domain objects to fulfill use cases.
//! It defines **ports** (trait interfaces) that infrastructure adapters
//! must implement, following the Hexagonal Architecture pattern.
//!
//! ## Dependency Rule
//! This layer depends on `domain` only. It must NOT depend on
//! `infrastructure` directly.

pub mod ports;
pub mod use_cases;
