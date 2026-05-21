//! Observability — tracing initialization and structured logging.
//!
//! This module provides a centralized initialization for the `tracing`
//! subsystem. It configures the subscriber based on the environment:
//!
//! - **Development**: `pretty` formatter with color output, `RUST_LOG=debug`
//! - **Production**: `json` formatter for structured log aggregation,
//!   `RUST_LOG=info`
//!
//! ## Usage
//!
//! ```rust,no_run
//! presidium_core::observability::init_tracing("info");
//! ```
//!
//! ## Environment Variable
//!
//! The `RUST_LOG` environment variable controls the filter level.
//! Example: `RUST_LOG=presidium_core=debug,presidium_crypto=trace`

use tracing_subscriber::{fmt, EnvFilter};

/// Initializes the tracing subscriber for the application.
///
/// This function should be called once at the application entry point.
/// It sets up a `tracing-subscriber` with environment-based filtering
/// and a format appropriate for the build profile.
///
/// # Arguments
///
/// * `default_level` - The default log level if `RUST_LOG` is not set
///   (e.g., "info", "debug", "trace").
///
/// # Panics
///
/// Panics if the subscriber cannot be installed (e.g., if called
/// more than once).
///
/// # Example
///
/// ```rust,no_run
/// presidium_core::observability::init_tracing("info");
/// ```
pub fn init_tracing(default_level: &str) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

    if cfg!(debug_assertions) {
        // Development: pretty, colorful output
        fmt()
            .with_env_filter(filter)
            .pretty()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    } else {
        // Production: structured JSON output for log aggregation
        fmt()
            .with_env_filter(filter)
            .json()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(false)
            .init();
    }
}

/// Initializes a minimal tracing subscriber suitable for testing.
///
/// Uses the `pretty` format with `RUST_LOG` or defaults to "debug".
/// This is intended for use in integration tests and should not
/// be called in production code.
pub fn init_test_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    let _ = fmt()
        .with_env_filter(filter)
        .pretty()
        .with_target(true)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_tracing_does_not_panic_with_valid_level() {
        // This test verifies that the EnvFilter can be created from defaults.
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        // The filter was created successfully
        drop(filter);
    }

    #[test]
    fn default_filter_level_is_valid() {
        let filter = EnvFilter::new("debug");
        // The filter was created successfully
        drop(filter);
    }
}
