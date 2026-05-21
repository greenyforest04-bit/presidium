//! Domain layer — pure business logic and domain model.
//!
//! This module contains the heart of Presidium Messenger: entities, value objects,
//! aggregates, domain events, and domain errors. It has **zero** dependency on
//! infrastructure or application layers, ensuring that business rules are
//! isolated, testable, and stable.

pub mod aggregates;
pub mod entities;
pub mod errors;
pub mod events;
pub mod value_objects;
