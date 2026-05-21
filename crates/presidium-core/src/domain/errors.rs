//! Domain errors — business rule violations and domain-level failures.
//!
//! Domain errors represent violations of business rules within the
//! Presidium Messenger domain. They are distinct from infrastructure
//! errors (network failures, I/O errors) and application errors.
//!
//! Use `thiserror` to define well-structured, typed errors that
//! propagate cleanly through the domain layer.

use thiserror::Error;

/// Errors that arise from violations of domain business rules.
///
/// These errors indicate that an operation cannot proceed because
/// it would violate the invariants of the domain model. They should
/// be propagated using the `?` operator and handled at the
/// application layer.
#[derive(Error, Debug)]
pub enum DomainError {
    /// A message exceeds the maximum allowed size.
    #[error("Message too large: {actual} bytes (max: {max})")]
    MessageTooLarge {
        /// Actual size in bytes.
        actual: usize,
        /// Maximum allowed size in bytes.
        max: usize,
    },

    /// An operation is invalid in the current context.
    #[error("Invalid operation: {reason}")]
    InvalidOperation {
        /// Explanation of why the operation is invalid.
        reason: String,
    },

    /// A required entity was not found.
    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound {
        /// The type of entity that was not found.
        entity_type: String,
        /// The identifier of the missing entity.
        id: String,
    },

    /// A concurrency conflict occurred (optimistic locking).
    #[error("Concurrency conflict on {entity_type}: expected version {expected}, found {found}")]
    ConcurrencyConflict {
        /// The type of entity with the conflict.
        entity_type: String,
        /// Expected version number.
        expected: u64,
        /// Actual version number found.
        found: u64,
    },

    /// A domain invariant was violated.
    #[error("Domain invariant violated: {invariant}")]
    InvariantViolated {
        /// Description of the violated invariant.
        invariant: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_error_message_too_large_display() {
        let err = DomainError::MessageTooLarge { actual: 100_000, max: 65_536 };
        let msg = format!("{err}");
        assert!(msg.contains("100000"));
        assert!(msg.contains("65536"));
    }

    #[test]
    fn domain_error_invalid_operation_display() {
        let err = DomainError::InvalidOperation { reason: "test reason".to_string() };
        assert!(format!("{err}").contains("test reason"));
    }

    #[test]
    fn domain_error_not_found_display() {
        let err =
            DomainError::NotFound { entity_type: "User".to_string(), id: "alice".to_string() };
        let msg = format!("{err}");
        assert!(msg.contains("User"));
        assert!(msg.contains("alice"));
    }
}
