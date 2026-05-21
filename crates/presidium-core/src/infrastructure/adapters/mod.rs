//! Adapter implementations for external system interfaces.
//!
//! Adapters are concrete implementations of the port traits that
//! connect the application to external systems. Each adapter is
//! responsible for translating between the domain model and the
//! external system's API.
//!
//! Note: Primary adapter implementations live in their respective
//! satellite crates (presidium-crypto, presidium-p2p, etc.).
//! This module in presidium-core exists for cross-cutting adapters
//! or in-memory test doubles that don't belong to any specific crate.

// Cross-cutting or core-specific adapters will be added here.
// Example:
// - InMemoryStorageAdapter (for testing)
// - LoggingDecorator (wraps any port with tracing)
