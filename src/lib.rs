#![warn(missing_docs)]
//! Shared Bitcoin RPC type definitions
//!
//! This crate provides the core types and utilities for working with the
//! Bitcoin Core JSON-RPC API. It serves as a shared foundation that can be
//! used by both the codegen system and external consumers like fuzzing tools.
//!
//! ## Core Types
//! - `BtcMethod` - Complete Bitcoin method definition
//! - `BtcArgument` - Method argument specification
//! - `BtcResult` - Method result specification
//! - `ApiDefinition` - Complete API definition container

pub mod hash_or_height;
pub mod types;

pub use hash_or_height::HashOrHeight;
pub use types::{ApiDefinition, BtcArgument, BtcMethod, BtcResult, Result, SchemaError};
