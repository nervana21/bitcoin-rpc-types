#![warn(missing_docs)]
//! Shared Bitcoin RPC type definitions
//!
//! This crate provides the core types and utilities for working with the
//! Bitcoin Core JSON-RPC API. It serves as the shared foundation for the
//! `bitcoin-rpc-*` ecosystem and is intended to be used by all consumers
//! of the interface.
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
