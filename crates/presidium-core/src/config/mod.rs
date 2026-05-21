//! Application configuration — typed configuration management.
//!
//! This module provides strongly-typed configuration structures that
//! are loaded from `presidium.toml` or environment variables using
//! the `figment` crate.
//!
//! ## Configuration Sources (in priority order)
//! 1. Environment variables (prefixed with `PRESIDIUM_`)
//! 2. `presidium.toml` in the current directory
//! 3. Default values
//!
//! ## Example `presidium.toml`
//! ```toml
//! [network]
//! listen_port = 4001
//! bootstrap_peers = ["/dns4/bootstrap.presidium.dev/tcp/4001"]
//!
//! [storage]
//! database_path = "./data/presidium.db"
//!
//! [crypto]
//! pre_key_count = 100
//! ```

use serde::{Deserialize, Serialize};

/// Root application configuration.
///
/// Contains all configuration sections for Presidium Messenger.
/// Loaded from environment variables or `presidium.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Network-related configuration.
    pub network: NetworkConfig,
    /// Storage-related configuration.
    pub storage: StorageConfig,
    /// Cryptography-related configuration.
    pub crypto: CryptoConfig,
    /// LLM-related configuration.
    pub llm: LlmConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            crypto: CryptoConfig::default(),
            llm: LlmConfig::default(),
        }
    }
}

impl AppConfig {
    /// Loads configuration from `presidium.toml` and environment variables.
    ///
    /// Environment variables override the TOML file and should be
    /// prefixed with `PRESIDIUM_` (e.g., `PRESIDIUM_NETWORK__LISTEN_PORT`).
    ///
    /// # Errors
    /// Returns a `figment::Error` if the configuration is invalid or
    /// cannot be loaded.
    pub fn load() -> Result<Self, figment::Error> {
        use figment::{providers::Env, providers::Format, providers::Serialized, providers::Toml, Figment};

        Figment::from(Serialized::defaults(Self::default()))
            .merge(Toml::file("presidium.toml"))
            .merge(Env::prefixed("PRESIDIUM_").split("__"))
            .extract()
    }

    /// Loads configuration with a custom config file path.
    ///
    /// # Errors
    /// Returns a `figment::Error` if the configuration is invalid.
    pub fn load_from(path: &str) -> Result<Self, figment::Error> {
        use figment::{providers::Env, providers::Format, providers::Serialized, providers::Toml, Figment};

        Figment::from(Serialized::defaults(Self::default()))
            .merge(Toml::file(path))
            .merge(Env::prefixed("PRESIDIUM_").split("__"))
            .extract()
    }
}

/// Network configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// The port to listen on for P2P connections.
    pub listen_port: u16,
    /// List of bootstrap peer multiaddresses for initial discovery.
    pub bootstrap_peers: Vec<String>,
    /// Whether to enable mDNS for local network discovery.
    pub enable_mdns: bool,
    /// Whether to enable Circuit Relay v2 for NAT traversal.
    pub enable_relay: bool,
    /// Maximum number of concurrent P2P connections.
    pub max_connections: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_port: 4001,
            bootstrap_peers: Vec::new(),
            enable_mdns: true,
            enable_relay: true,
            max_connections: 100,
        }
    }
}

/// Storage configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Path to the encrypted database file.
    pub database_path: String,
    /// Maximum database size in megabytes.
    pub max_size_mb: u32,
    /// Whether to enable WAL mode for better write performance.
    pub enable_wal: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            database_path: "./data/presidium.db".to_string(),
            max_size_mb: 512,
            enable_wal: true,
        }
    }
}

/// Cryptography configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    /// Number of pre-keys to generate and upload.
    pub pre_key_count: u32,
    /// Whether to enable post-quantum key exchange (PQXDH).
    pub enable_post_quantum: bool,
    /// Pre-key rotation interval in hours.
    pub pre_key_rotation_hours: u32,
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            pre_key_count: 100,
            enable_post_quantum: true,
            pre_key_rotation_hours: 24,
        }
    }
}

/// LLM configuration for on-device inference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Path to the GGUF model file.
    pub model_path: String,
    /// Quantization level (e.g., "Q4_K_M", "Q8_0").
    pub quantization: String,
    /// Maximum context window size in tokens.
    pub max_context_tokens: u32,
    /// Number of threads for inference.
    pub inference_threads: u32,
    /// Whether moderation is enabled by default.
    pub moderation_enabled: bool,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            model_path: "./models/gemma-2b-q4_k_m.gguf".to_string(),
            quantization: "Q4_K_M".to_string(),
            max_context_tokens: 4096,
            inference_threads: 4,
            moderation_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let config = AppConfig::default();
        assert_eq!(config.network.listen_port, 4001);
        assert_eq!(config.crypto.pre_key_count, 100);
        assert!(config.crypto.enable_post_quantum);
        assert!(config.llm.moderation_enabled);
    }

    #[test]
    fn network_config_defaults() {
        let config = NetworkConfig::default();
        assert!(config.enable_mdns);
        assert!(config.enable_relay);
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    fn storage_config_defaults() {
        let config = StorageConfig::default();
        assert!(config.database_path.contains("presidium.db"));
        assert!(config.enable_wal);
    }

    #[test]
    fn crypto_config_defaults() {
        let config = CryptoConfig::default();
        assert!(config.enable_post_quantum);
        assert_eq!(config.pre_key_rotation_hours, 24);
    }

    #[test]
    fn llm_config_defaults() {
        let config = LlmConfig::default();
        assert!(config.model_path.contains(".gguf"));
        assert_eq!(config.max_context_tokens, 4096);
        assert_eq!(config.inference_threads, 4);
    }

    #[test]
    fn config_serialization_roundtrip() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).expect("serialize");
        let decoded: AppConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.network.listen_port, config.network.listen_port);
        assert_eq!(decoded.crypto.pre_key_count, config.crypto.pre_key_count);
    }
}
