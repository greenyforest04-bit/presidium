//! Infrastructure layer — adapter implementations for external systems.
//!
//! This layer contains concrete implementations of the port traits
//! defined in the `application::ports` module. Adapters bridge the
//! domain/application logic with external systems (databases, network,
//! crypto libraries, etc.).
//!
//! ## Dependency Rule
//! This layer depends on `application` and `domain`. The domain layer
//! must NEVER depend on this layer.

pub mod adapters;
pub mod repositories;
