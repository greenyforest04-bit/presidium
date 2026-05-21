//! # Presidium P2P
//!
//! P2P networking ports and adapters for Presidium Messenger.
//!
//! This crate provides the interface and implementation for all
//! peer-to-peer networking operations including:
//! - **libp2p**: Kademlia DHT, GossipSub, Circuit Relay v2
//! - **Transport**: QUIC, WebRTC, TCP fallback
//! - **Discovery**: mDNS for local, DHT for global
//! - **NAT Traversal**: ICE, hole punching, relay
//!
//! ## Architecture
//!
//! Follows Hexagonal Architecture with domain/application/infrastructure layers.

pub mod application;
pub mod domain;
pub mod infrastructure;
